// SPDX-License-Identifier: CC0
pragma solidity >=0.6.0;

interface INFVerifier {
    function verify(
        bytes calldata,
        bytes32[] calldata publicInputs
    ) external returns (bool result);
}
