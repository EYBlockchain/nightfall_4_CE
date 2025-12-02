// SPDX-License-Identifier: CC0
pragma solidity >=0.8.20;

import "./INFVerifier.sol";

// Mock verifier contract that just returns `true`.
contract MockVerifier is INFVerifier {
    bool private defaultResult = true;

    function verify(
        bytes calldata accBytes,
        bytes calldata proofBytes,
        bytes calldata publicInputsHashBytes,
        uint256 rollup_batch_size
    ) external view override returns (bool result) {
        accBytes;
        proofBytes;
        publicInputsHashBytes;
        rollup_batch_size;
        result = defaultResult;
        return result;
    }
}
