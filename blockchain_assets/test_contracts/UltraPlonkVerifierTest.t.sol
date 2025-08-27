// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../contracts/proof_verification/UltraPlonkVerifier.sol";

contract UltraPlonkVerifierTest is Test {
    UltraPlonkVerifier verifier;
    bytes proofBytes;
    bytes publicDataBytes;


    function setUp() public {
        verifier = new UltraPlonkVerifier();

        // Load proof from file
        string memory proofHexString = string(vm.readFile("./blockchain_assets/test_contracts/ultraPlonkProof.json"));
        proofBytes = vm.parseBytes(proofHexString);

        // Example of known-valid public input (adjust as needed)
        publicDataBytes = hex"0000000000000000000000000000000000000000000000000000000000000002";
    }
    
    // Completeness: valid proof and public input
    function testValidProofVerification() public view {
        bool verified = verifier.verify(proofBytes, publicDataBytes);
        assertTrue(verified, "Proof should be verified successfully");
    }

    // Soundness: Corrupt a byte in the proof
    function testCorruptedProofFailsVerification() public {
        bytes memory corruptedProof = bytes(proofBytes);
        corruptedProof[10] ^= 0x01; // Flip 1 bit
        vm.expectRevert(); // Expect a revert on malformed G1
        verifier.verify(corruptedProof, publicDataBytes);
    }

   // Soundness: Copy wires_poly_comms_1 into wires_poly_comms_2
// Both are valid G1 points, but this breaks the integrity of the proof
function testDuplicatedG1PointFailsVerification() public view {
     // Make deep copy
    bytes memory modifiedProof = new bytes(proofBytes.length);
    for (uint i = 0; i < proofBytes.length; i++) {
        modifiedProof[i] = proofBytes[i];
    }

    // Each G1 point is 64 bytes (32 bytes x, 32 bytes y)
    // wires_poly_comms_1: offset 0x00
    // wires_poly_comms_2: offset 0x40

    uint256 srcOffset = 0x00;
    uint256 dstOffset = 0x40;

    // Copy wires_poly_comms_1 (x, y) into wires_poly_comms_2 (x, y)
    for (uint i = 0; i < 64; i++) {
        modifiedProof[dstOffset + i] = modifiedProof[srcOffset + i];
    }

    // Debug: Load and print the two G1 points
    bytes32 x1; bytes32 y1;
    bytes32 x2; bytes32 y2;
    assembly {
        x1 := mload(add(modifiedProof, add(srcOffset, 32)))
        y1 := mload(add(modifiedProof, add(srcOffset, 64)))
        x2 := mload(add(modifiedProof, add(dstOffset, 32)))
        y2 := mload(add(modifiedProof, add(dstOffset, 64)))
    }

    // Hard check
    assertEq(x1, x2, "X values must be equal");
    assertEq(y1, y2, "Y values must be equal");

    // Finally: test that verification fails
    bool verified = verifier.verify(modifiedProof, publicDataBytes);
    assertFalse(verified, "Proof with duplicated G1 points should fail verification");
}





    // Soundness: Modify public input slightly
    function testWrongPublicInputFailsVerification() public view {
        bytes memory wrongPublicInput = bytes(publicDataBytes);
        wrongPublicInput[31] ^= 0x01; // Flip LSB of input

        bool verified = verifier.verify(proofBytes, wrongPublicInput);
        assertFalse(verified, "Wrong public input should fail verification");
    }

    // Soundness: Empty public input
    function testEmptyPublicInputFailsVerification() public view {
        bytes memory emptyInput = new bytes(0);

        bool verified = verifier.verify(proofBytes, emptyInput);
        assertFalse(verified, "Empty public input should fail verification");
    }
}
