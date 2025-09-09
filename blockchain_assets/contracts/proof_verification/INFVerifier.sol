// SPDX-License-Identifier: CC0
pragma solidity >=0.8.20;

interface INFVerifier {
    function verify(
        bytes calldata accBytes,
        bytes calldata proofBytes, 
        bytes calldata publicInputsHashBytes
    ) external view returns (bool result);
}
