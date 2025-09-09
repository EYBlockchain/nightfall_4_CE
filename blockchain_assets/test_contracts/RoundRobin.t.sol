// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../contracts/RoundRobin.sol";
import "../contracts/Nightfall.sol";
import "../contracts/proof_verification/MockVerifier.sol";
import "../contracts/SanctionsListMock.sol";
import "../contracts/X509/X509.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract RoundRobinTest is Test {
    address public default_proposer_address =
        address(0xa0Ee7A142d267C1f36714E4a8F75612F20a79720);
    address public proposer2_address;
    string public default_proposer_url = "http://localhost:3000";
    string public proposer2_url = "http://localhost:3001";

    X509 x509Contract;
    RoundRobin roundRobin;
    Nightfall nightfall;
    MockVerifier verifier;

    function setUp() public {
        vm.deal(address(this), 100 ether); // give the test contract funds

        // X509 + sanctions
        x509Contract = new X509();
        x509Contract.initialize(address(this));
        address sanctionedUser = address(0x123);
        SanctionsListMock sanctionsListMock = new SanctionsListMock(sanctionedUser);

        // Verifier (mock implements INFVerifier)
        verifier = new MockVerifier();

        // ---------------------------
        // Nightfall (UUPS + initialize)
        // ---------------------------
        Nightfall nfImpl = new Nightfall();
        uint256 initialNullifierRoot = 5626012003977595441102792096342856268135928990590954181023475305010363075697;
        bytes memory nfInit = abi.encodeCall(
            Nightfall.initialize,
            (initialNullifierRoot, uint256(0), uint256(0), int256(0), verifier, address(x509Contract), address(sanctionsListMock))
        );
        nightfall = Nightfall(address(new ERC1967Proxy(address(nfImpl), nfInit)));

        // ---------------------------
        // RoundRobin (UUPS + initialize)
        // ---------------------------
        RoundRobin rrImpl = new RoundRobin();
        bytes memory rrInit = abi.encodeCall(
            RoundRobin.initialize,
            (
                address(x509Contract),
                address(sanctionsListMock),
                5, // stake
                3, // ding
                2, // exit_penalty
                1, // cooling_blocks
                0  // rotation_blocks
            )
        );
        roundRobin = RoundRobin(payable(address(new ERC1967Proxy(address(rrImpl), rrInit))));

        // Bootstrap default proposer (pay stake) and wire Nightfall
        roundRobin.bootstrapDefaultProposer{value: 5}(
            default_proposer_address,
            default_proposer_url,
            address(nightfall)
        );
    }

    function test_round_robin() public {
        uint256 initialEscrow = roundRobin.escrow();
        assertEq(initialEscrow, 5, "Initial escrow should equal stake");
        assertEq(roundRobin.get_current_proposer_address(), default_proposer_address);

        // only one proposer and it’s self-linked
        assertEq(roundRobin.get_proposers().length, 1);
        assertEq(roundRobin.get_proposers()[0].url, default_proposer_url);
        assertEq(roundRobin.get_proposers()[0].addr, default_proposer_address);
        assertEq(roundRobin.get_proposers()[0].next_addr, default_proposer_address);
        assertEq(roundRobin.get_proposers()[0].previous_addr, default_proposer_address);

        // turn off x509 checks for this test
        x509Contract.enableAllowlisting(false);

        // add second proposer (msg.sender = address(this))
        roundRobin.add_proposer{value: 5}(proposer2_url);
        uint256 updatedEscrow = roundRobin.escrow();
        assertEq(updatedEscrow, 10, "Escrow = 2 * stake after adding");

        // list / linking checks
        assertEq(roundRobin.get_proposers().length, 2);
        proposer2_address = roundRobin.get_proposers()[1].addr;
        assertEq(roundRobin.get_proposers()[1].url, proposer2_url);
        assertEq(roundRobin.get_proposers()[1].next_addr, default_proposer_address);
        assertEq(roundRobin.get_proposers()[1].previous_addr, default_proposer_address);
        assertEq(roundRobin.get_proposers()[0].addr, default_proposer_address);
        assertEq(roundRobin.get_proposers()[0].next_addr, proposer2_address);
        assertEq(roundRobin.get_proposers()[0].previous_addr, proposer2_address);

        // rotate after finalization window
        vm.roll(block.number + 64);
        roundRobin.rotate_proposer();
        assertEq(roundRobin.get_current_proposer_address(), proposer2_address);

        // current proposer (address(this)) deregisters → pays exit penalty
        roundRobin.remove_proposer();
        uint256 newEscrow = roundRobin.escrow();
        uint256 newStake1 = roundRobin.pending_withdraws(default_proposer_address);
        uint256 newStake2 = roundRobin.pending_withdraws(proposer2_address);
        assertEq(newEscrow, 5, "Escrow after penalty incorrect");
        assertEq(newStake1, 0, "Proposer 1 pending withdraw incorrect");
        assertEq(newStake2, 3, "Proposer 2 pending withdraw incorrect"); // 5 - 2 penalty

        // rotate to remaining proposer
        vm.roll(block.number + 64);
        roundRobin.rotate_proposer();
        assertEq(roundRobin.get_current_proposer_address(), default_proposer_address);

        // only one proposer remains and is self-linked
        assertEq(roundRobin.get_proposers().length, 1);
        assertEq(roundRobin.get_proposers()[0].url, default_proposer_url );
        assertEq(roundRobin.get_proposers()[0].addr, default_proposer_address);
        assertEq(roundRobin.get_proposers()[0].next_addr, default_proposer_address);
        assertEq(roundRobin.get_proposers()[0].previous_addr, default_proposer_address);

        // cannot remove the last proposer
        vm.prank(default_proposer_address);
        vm.expectRevert("Cannot deregister the only active proposer");
        roundRobin.remove_proposer();
    }
}
