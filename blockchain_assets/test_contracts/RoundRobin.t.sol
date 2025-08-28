// SPDX-License-Identifier: CC0
pragma solidity >=0.8.19;

import "../contracts/RoundRobin.sol";
import "../contracts/Nightfall.sol";
import "../contracts/proof_verification/MockVerifier.sol";
import "forge-std/Test.sol";
import "forge-std/console2.sol";
import "../contracts/SanctionsListMock.sol";
import "../contracts/X509/X509.sol";


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
        vm.deal(address(this), 100);
        // x509Contract = new X509(address(this));
        x509Contract = new X509();
        x509Contract.initialize(address(this));
        address sanctionedUser = address(0x123);
        SanctionsListMock sanctionsListMock = new SanctionsListMock(
            sanctionedUser
        );
        roundRobin = new RoundRobin{value: 5}(
            address(x509Contract),
            address(sanctionsListMock),
            default_proposer_address,
            default_proposer_url,
            5, // stake
            3, // ding
            2, // exit_penalty
            1, // allow to reregister immediately (no cooling_blocks)
            0 // allow to rotate immediately
        );
        

        verifier = new MockVerifier();

        nightfall = new Nightfall(
            verifier,
            address(x509Contract),
            address(sanctionsListMock)
        );
        roundRobin.set_nightfall(address(nightfall));
    }

    function test_round_robin() public {
        uint256 initialEscrow = roundRobin.escrow();
        assertEq(
            initialEscrow,
            5,
            "Initial escrow should be equal to the stake amount"
        );
        assertEq(
            roundRobin.get_current_proposer_address(),
            default_proposer_address
        );
        // check the current proposer is the only one in the list and is linked to itself
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

        // as this test is about the Round Robin proposer management
        // we temporally turn off x509
        x509Contract.enableAllowlisting(false);

        roundRobin.add_proposer{value: 5}(proposer2_url);
        uint256 updatedEscrow = roundRobin.escrow();
        assertEq(
            updatedEscrow,
            10,
            "Escrow should be equal to the stake amount * 2 after adding a proposer"
        );
        // check the current proposer is the first one in the list and is linked to the second proposer and vice versa
        assertEq(roundRobin.get_proposers().length, 2);
        proposer2_address = roundRobin.get_proposers()[1].addr;
        assertEq(roundRobin.get_proposers()[1].url, proposer2_url);
        assertEq(roundRobin.get_proposers()[1].addr, proposer2_address);
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
        assertEq(roundRobin.get_proposers()[0].url, default_proposer_url);
        // rotate when 64 blocks have passed
        vm.roll(block.number + 64);
        // rotate the proposer
        roundRobin.rotate_proposer();
        // check that the rotation has succeeded
        assertEq(roundRobin.get_current_proposer_address(), proposer2_address);

        // try removing proposers
        // proposer2 has the same address as the wallet funding this contract so it is the one that would
        // be removed. It is also the current proposer, so we can remove it but it will need to pay the exit penalty;
        roundRobin.remove_proposer();
        uint256 newEscrow = roundRobin.escrow();
        uint256 newStake = roundRobin.pending_withdraws(default_proposer_address);
        uint256 newStake2 = roundRobin.pending_withdraws(proposer2_address);
        
        // proposer 1 is not removed so its pending withdraws should be 0
        // proposer 2 is removed so its pending withdraws should be 3 (5 - 2) after paying the exit penalty
        // In other words, escrow should keep the stakes of active proposers
        // Ding and exit penalty are not tracked in escrow
        assertEq(newEscrow, 5, "Escrow after penalty incorrect");
        assertEq(newStake, 0, "Proposer 1's pending withdraw incorrect");
        assertEq(newStake2, 3, "Proposer 2's pending withdraw incorrect");
        
        // rotate to the next proposer
        vm.roll(block.number + 64);
        roundRobin.rotate_proposer();
        assertEq(
            roundRobin.get_current_proposer_address(),
            default_proposer_address
        );
        // check the current proposer is the only one in the list and is linked to itself
        assertEq(roundRobin.get_proposers().length, 1);
        assertEq(roundRobin.get_proposers()[0].url, default_proposer_url );
        assertEq(roundRobin.get_proposers()[0].addr, default_proposer_address);
        assertEq(
            roundRobin.get_proposers()[0].next_addr,
            default_proposer_address
        );
        assertEq(
            roundRobin.get_proposers()[0].previous_addr,
            default_proposer_address
        );
        // now we only have one proposer left, so we can't remove it
        vm.prank(default_proposer_address);
        vm.expectRevert("Cannot deregister the only active proposer");
        roundRobin.remove_proposer();
    }
}
