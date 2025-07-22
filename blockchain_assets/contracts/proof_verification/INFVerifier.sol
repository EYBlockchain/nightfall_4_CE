// SPDX-License-Identifier: CC0
pragma solidity >=0.6.0;

interface INFVerifier {
    function verify(bytes calldata proofBytes, bytes calldata publicInputsHashBytes) external returns (bool result);
}
