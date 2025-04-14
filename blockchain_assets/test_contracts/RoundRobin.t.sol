// SPDX-License-Identifier: CC0
pragma solidity >=0.8.19;

import "../contracts/RoundRobin.sol";
import "../contracts/Nightfall.sol";
import "../contracts/proof_verification/MockVerifier.sol";
import "forge-std/Test.sol";
import "forge-std/console2.sol";
import "../contracts/SanctionsListMock.sol";

contract RoundRobinTest is Test {
    address public default_proposer_address =
        address(0xa0Ee7A142d267C1f36714E4a8F75612F20a79720);
    address public proposer2_address;
    string public default_proposer_url = "http://localhost:3000";
    string public proposer2_url = "http://localhost:3001";

    RoundRobin roundRobin;
    Nightfall nightfall;
    MockVerifier verifier;

    function setUp() public {
        roundRobin = new RoundRobin(
            default_proposer_address,
            default_proposer_url,
            0, // no stake
            0, // no ding
            0 // allow to rotate immediately
        );
        X509 x509Contract = new X509(address(this));
        address sanctionedUser = address(0x123);
        SanctionsListMock sanctionsListMock = new SanctionsListMock(
            sanctionedUser
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

        roundRobin.add_proposer(proposer2_url);
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

        roundRobin.rotate_proposer();
        // check that the rotation has succeeded
        assertEq(roundRobin.get_current_proposer_address(), proposer2_address);

        // try removing proposers
        // proposer2 has the same address as the wallet funding this contract so it is the one that would
        // be removed. However it is also the current proposer, so this should fail.
        vm.expectRevert("You cannot remove the current proposer");
        roundRobin.remove_proposer();
        // rotate to the next proposer
        roundRobin.rotate_proposer();
        assertEq(
            roundRobin.get_current_proposer_address(),
            default_proposer_address
        );
        // now we should be able to remove proposer2
        roundRobin.remove_proposer();
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
    }
}
