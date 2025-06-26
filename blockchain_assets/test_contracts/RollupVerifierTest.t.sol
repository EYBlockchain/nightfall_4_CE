// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.20;

import "forge-std/Test.sol";
import {TesterVerifier} from "./TesterVerifier.sol";
import "forge-std/Vm.sol";

contract RollupVerifierTest is Test {
    TesterVerifier verifier;

    bytes32[] public inputs;
    bytes public proof;

    function setUp() public {
        string memory root = vm.projectRoot();
        string memory path = string.concat(
            root,
            "/blockchain_assets/test_contracts/testproof.txt"
        );
        string memory proofString = vm.readLine(path);
        proof = vm.parseBytes(proofString);

        bytes32 pi_1 = bytes32(uint256(4));
        bytes32 pi_2 = bytes32(uint256(11));
        bytes32 pi_3 = bytes32(uint256(11227));
        bytes32 pi_4 = bytes32(uint256(44));

        inputs.push(pi_1);
        inputs.push(pi_2);
        inputs.push(pi_3);
        inputs.push(pi_4);

        verifier = new TesterVerifier();
    }

    function testRollupVerifier() public view {
        assertEq(verifier.verify(proof, inputs), true);
    }
}
