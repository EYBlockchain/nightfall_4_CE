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
        // IMPORTANT: since the implementation has `constructor(){ _disableInitializers(); }`
        // we must initialize THROUGH THE PROXY, not by calling initialize on the impl.
        X509 x509Impl = new X509();
        bytes memory x509Init = abi.encodeCall(X509.initialize, (address(this)));
        x509Contract = X509(address(new ERC1967Proxy(address(x509Impl), x509Init)));

        address sanctionedUser = address(0x123);
        SanctionsListMock sanctionsListMock = new SanctionsListMock(
            sanctionedUser
        );

        // Verifier (mock implements INFVerifier)
        verifier = new MockVerifier();

        // ---------------------------
        // Nightfall (UUPS + initialize)
        // ---------------------------
        Nightfall nfImpl = new Nightfall();
        uint256 initialNullifierRoot = 5626012003977595441102792096342856268135928990590954181023475305010363075697;
        bytes memory nfInit = abi.encodeCall(
            Nightfall.initialize,
            (
                initialNullifierRoot,
                uint256(0),
                uint256(0),
                int256(0),
                verifier,
                address(x509Contract),
                address(sanctionsListMock)
            )
        );
        nightfall = Nightfall(
            address(new ERC1967Proxy(address(nfImpl), nfInit))
        );

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
                2, // rotation_blocks
                1 // grace_blocks
            )
        );
        roundRobin = RoundRobin(
            payable(address(new ERC1967Proxy(address(rrImpl), rrInit)))
        );

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
        assertEq(
            roundRobin.get_current_proposer_address(),
            default_proposer_address
        );

        // only one proposer and it’s self-linked
        assertEq(roundRobin.get_proposers().length, 1);
        assertEq(roundRobin.get_proposers()[0].url, default_proposer_url);
        assertEq(roundRobin.get_proposers()[0].addr, default_proposer_address);
        assertEq(
            roundRobin.get_proposers()[0].next_addr,
            default_proposer_address
        );
        assertEq(
            roundRobin.get_proposers()[0].previous_addr,
            default_proposer_address
        );

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
        assertEq(
            roundRobin.get_proposers()[1].next_addr,
            default_proposer_address
        );
        assertEq(
            roundRobin.get_proposers()[1].previous_addr,
            default_proposer_address
        );
        assertEq(roundRobin.get_proposers()[0].addr, default_proposer_address);
        assertEq(roundRobin.get_proposers()[0].next_addr, proposer2_address);
        assertEq(
            roundRobin.get_proposers()[0].previous_addr,
            proposer2_address
        );

        // rotate after finalization window and rotation window
        vm.roll(block.number + 64 + 5);
        roundRobin.rotate_proposer();
        assertEq(roundRobin.get_current_proposer_address(), proposer2_address);

       // Check proposer url exists in mapping
        bool exists_proposer = roundRobin.proposer_urls(proposer2_url);
        assertEq(exists_proposer, true, "Proposer 2 URL doesn't  exists");
         // current proposer (address(this)) deregisters → pays exit penalty
        roundRobin.remove_proposer();
        // check if the current proposer doesn't hold any stale pointers
        assertEq(roundRobin.current.next_addr, default_proposer_address, "Next addr incorrect after removal");
        assertEq(roundRobin.current.previous_addr, default_proposer_address, "Previous addr incorrect after removal");
        // check if proposer url is removed from mapping
        bool exists_proposer2 = roundRobin.proposer_urls(proposer2_url);
        assertEq(exists_proposer2, false, "Proposer 2 URL still exists");

        uint256 newEscrow = roundRobin.escrow();
        uint256 newStake1 = roundRobin.pending_withdraws(
            default_proposer_address
        );
        uint256 newStake2 = roundRobin.pending_withdraws(proposer2_address);
        assertEq(newEscrow, 2, "Escrow after penalty incorrect");
        assertEq(newStake1, 0, "Proposer 1 pending withdraw incorrect");
        assertEq(newStake2, 3, "Proposer 2 pending withdraw incorrect"); // 5 - 2 penalty

        // rotate to remaining proposer
        vm.roll(block.number + 64);
        assertEq(
            roundRobin.get_current_proposer_address(),
            default_proposer_address
        );

        // only one proposer remains and is self-linked
        assertEq(roundRobin.get_proposers().length, 1);
        assertEq(roundRobin.get_proposers()[0].url, default_proposer_url);
        assertEq(roundRobin.get_proposers()[0].addr, default_proposer_address);
        assertEq(
            roundRobin.get_proposers()[0].next_addr,
            default_proposer_address
        );
        assertEq(
            roundRobin.get_proposers()[0].previous_addr,
            default_proposer_address
        );

        // cannot remove the last proposer
        vm.prank(default_proposer_address);
        vm.expectRevert("Cannot deregister the only active proposer");
        roundRobin.remove_proposer();
    }

    /// @dev With only one proposer, skip_inactive_proposer must revert.
    function test_skipInactiveProposer_revertsWithSingleProposer() public {
        assertEq(roundRobin.get_proposers().length, 1);

        // Even rolling forward in time, we still cannot skip because proposer_count == 1
        vm.roll(block.number + 1000);

        vm.expectRevert("Cannot skip single proposer");
        roundRobin.skip_inactive_proposer();
    }

     /// @dev With two proposers but before GRACE_BLOCKS has elapsed, skip must revert.
    function test_skipInactiveProposer_revertsBeforeGrace() public {
        x509Contract.enableAllowlisting(false);

        // Add second proposer so proposer_count > 1
        roundRobin.add_proposer{value: 5}(proposer2_url);
        assertEq(roundRobin.get_proposers().length, 2);

        uint256 grace = roundRobin.GRACE_BLOCKS(); // = 1 in this setup

        // Move fewer than GRACE_BLOCKS ahead:
        // if grace == 1, this means "no roll"
        if (grace > 0) {
            vm.roll(block.number + grace - 1);
        }

        vm.expectRevert("Proposer not yet inactive");
        roundRobin.skip_inactive_proposer();
    }

    /// @dev After GRACE_BLOCKS of inactivity with >1 proposer, skip should slash and rotate.
    function test_skipInactiveProposer_slashesAndRotatesAfterGrace() public {
        x509Contract.enableAllowlisting(false);

        // Add second proposer
        roundRobin.add_proposer{value: 5}(proposer2_url);
        proposer2_address = roundRobin.get_proposers()[1].addr;

        // Current proposer should be the default
        assertEq(
            roundRobin.get_current_proposer_address(),
            default_proposer_address
        );

        uint256 grace = roundRobin.GRACE_BLOCKS();
        uint256 escrowBefore = roundRobin.escrow();
        uint256 lazyPenalty = roundRobin.LAZY_PENALTY();

        // Advance at least GRACE_BLOCKS since lastProposedAt
        vm.roll(block.number + grace);

        // Anyone (address(this)) calls skip
        roundRobin.skip_inactive_proposer();

        // We should have rotated to proposer2
        assertEq(
            roundRobin.get_current_proposer_address(),
            proposer2_address
        );

        // Escrow should drop by LAZY_PENALTY
        uint256 escrowAfter = roundRobin.escrow();
        assertEq(
            escrowAfter,
            escrowBefore - lazyPenalty,
            "Escrow not dinged correctly"
        );

        // Default proposer's stake should be reduced by LAZY_PENALTY
        (uint256 stakeDefault, , , , ) = roundRobin.proposers(
            default_proposer_address
        );
        assertEq(
            stakeDefault,
            5 - lazyPenalty,
            "Default proposer stake not reduced by lazy penalty"
        );
    }

}