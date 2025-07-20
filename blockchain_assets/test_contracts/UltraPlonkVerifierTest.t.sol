// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../contracts/proof_verification/UltraPlonkVerifier.sol";
import "forge-std/console2.sol";

contract UltraPlonkVerifierTest is Test {
    UltraPlonkVerifier verifier;

    function testValidProofVerification() public {
        verifier = new UltraPlonkVerifier();
        // Load valid proof and block fields from JSON or hardcoded (for now)
        string memory proofHexString = string(vm.readFile("./blockchain_assets/test_contracts/ultraPlonkProof.json"));
        bytes memory proofBytes = vm.parseBytes(proofHexString);

        bytes memory publicDataBytes = hex"0000000000000000000000000000000000000000000000000000000000000002";


        bool verified = verifier.verify(proofBytes, publicDataBytes);
        assertTrue(verified, "Proof should be verified successfully");
    }
}
