// SPDX-License-Identifier: CC0
pragma solidity ^0.8.20;

import "../contracts/RoundRobinUUPS.sol";
import "../contracts/Nightfall.sol";
import "../contracts/proof_verification/MockVerifier.sol";
import "forge-std/Test.sol";
import "../contracts/SanctionsListMock.sol";
import "../contracts/X509/X509.sol";

// Use a UUPS proxy in tests so we exercise the initialize/upgradeable path.
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract RoundRobinTest is Test {
    address public default_proposer_address =
        address(0xa0Ee7A142d267C1f36714E4a8F75612F20a79720);
    address public proposer2_address;
    string public default_proposer_url = "http://localhost:3000";
    string public proposer2_url = "http://localhost:3001";

    X509 x509Contract;
    RoundRobin roundRobin;     // this will point to the proxy
    Nightfall nightfall;
    MockVerifier verifier;

    function setUp() public {
        // Give this test contract a little ETH to fund stakes.
        vm.deal(address(this), 100);

        // Deploy X509 (upgradeable pattern but for tests we deploy directly).
        x509Contract = new X509();
        x509Contract.initialize(address(this));

        // Sanctions list mock.
        address sanctionedUser = address(0x123);
        SanctionsListMock sanctionsListMock = new SanctionsListMock(sanctionedUser);

        // Deploy Nightfall deps.
        verifier  = new MockVerifier();
        nightfall = new Nightfall(
            verifier,
            address(x509Contract),
            address(sanctionsListMock)
        );

        // ---- Deploy RoundRobin implementation + UUPS proxy ----
        // Implementation (its constructor is not used by the proxy).
        RoundRobin impl = new RoundRobin();

        // Initialize via proxy (constructor args are moved to initialize()).
        bytes memory initData = abi.encodeCall(
            RoundRobin.initialize,
            (
                address(x509Contract),            // x509_address
                address(sanctionsListMock),       // sanctionsListAddress
                5,  // stake
                3,  // ding
                2,  // exit_penalty
                1,  // cooling_blocks (allow immediate reregister in tests)
                0   // rotation_blocks (rotate as soon as FINALIZATION_BLOCKS pass)
            )
        );

        ERC1967Proxy proxy = new ERC1967Proxy(address(impl), initData);
        roundRobin = RoundRobin(payable(address(proxy)));

        // Seed the default proposer and deposit initial stake (payable step).
        roundRobin.bootstrapDefaultProposer{value: 5}(
            default_proposer_address,
            default_proposer_url,
            address(nightfall)
        );

        // (Optional) Wire Nightfall back to RoundRobin if your app expects it.
        // nightfall.set_proposer_manager(roundRobin);

        // For test convenience (roundRobin already knows Nightfall from bootstrap).
        // roundRobin.set_nightfall(address(nightfall));
    }

    function test_round_robin() public {
        uint256 initialEscrow = roundRobin.escrow();
        assertEq(initialEscrow, 5, "Initial escrow should equal the stake");

        assertEq(
            roundRobin.get_current_proposer_address(),
            default_proposer_address
        );

        // Current (only) proposer is linked to itself
        assertEq(roundRobin.get_proposers().length, 1);
        assertEq(roundRobin.get_proposers()[0].url,  default_proposer_url);
        assertEq(roundRobin.get_proposers()[0].addr, default_proposer_address);
        assertEq(roundRobin.get_proposers()[0].next_addr,     default_proposer_address);
        assertEq(roundRobin.get_proposers()[0].previous_addr, default_proposer_address);

        // This test is about proposer mgmt, so relax x509 checks.
        x509Contract.enableAllowlisting(false);

        // Add a second proposer (msg.sender = this test contract).
        roundRobin.add_proposer{value: 5}(proposer2_url);

        uint256 updatedEscrow = roundRobin.escrow();
        assertEq(updatedEscrow, 10, "Escrow should be stake * 2 after adding");

        // Check ring structure after adding proposer 2
        assertEq(roundRobin.get_proposers().length, 2);
        proposer2_address = roundRobin.get_proposers()[1].addr;

        assertEq(roundRobin.get_proposers()[1].url,  proposer2_url);
        assertEq(roundRobin.get_proposers()[1].addr, proposer2_address);
        assertEq(roundRobin.get_proposers()[1].next_addr,     default_proposer_address);
        assertEq(roundRobin.get_proposers()[1].previous_addr, default_proposer_address);

        assertEq(roundRobin.get_proposers()[0].addr, default_proposer_address);
        assertEq(roundRobin.get_proposers()[0].next_addr, proposer2_address);
        assertEq(roundRobin.get_proposers()[0].previous_addr, proposer2_address);
        assertEq(roundRobin.get_proposers()[0].url,  default_proposer_url);

        // Rotate after FINALIZATION_BLOCKS (64) since rotation_blocks = 0
        vm.roll(block.number + 64);
        roundRobin.rotate_proposer();
        assertEq(roundRobin.get_current_proposer_address(), proposer2_address);

        // Remove current proposer (proposer2). This pays the exit penalty (2).
        roundRobin.remove_proposer();

        uint256 newEscrow  = roundRobin.escrow();
        uint256 newStake1  = roundRobin.pending_withdraws(default_proposer_address);
        uint256 newStake2  = roundRobin.pending_withdraws(proposer2_address);

        // Proposer 1 still active → pending 0
        // Proposer 2 removed → pending 3 (5 - 2 exit penalty)
        assertEq(newEscrow, 5, "Escrow after penalty should be 5");
        assertEq(newStake1, 0, "Proposer 1 pending withdraw should be 0");
        assertEq(newStake2, 3, "Proposer 2 pending withdraw should be 3");

        // Rotate again; only default proposer remains
        vm.roll(block.number + 64);
        roundRobin.rotate_proposer();
        assertEq(roundRobin.get_current_proposer_address(), default_proposer_address);

        // Only one proposer left; ring is self-linked
        assertEq(roundRobin.get_proposers().length, 1);
        assertEq(roundRobin.get_proposers()[0].url,  default_proposer_url);
        assertEq(roundRobin.get_proposers()[0].addr, default_proposer_address);
        assertEq(roundRobin.get_proposers()[0].next_addr,     default_proposer_address);
        assertEq(roundRobin.get_proposers()[0].previous_addr, default_proposer_address);

        // Can't deregister the only remaining proposer
        vm.prank(default_proposer_address);
        vm.expectRevert("Cannot deregister the only proposer");
        roundRobin.remove_proposer();
    }
}
