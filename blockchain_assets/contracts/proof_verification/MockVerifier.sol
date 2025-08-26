// SPDX-License-Identifier: CC0
pragma solidity >=0.8.0;

import "./INFVerifier.sol";

// Mock verifier contract that just returns `true`.
contract MockVerifier is INFVerifier {
    bool private defaultResult = true;

    function verify(
        bytes calldata _proofBytes, 
        bytes calldata _publicInputsHashBytes
    ) external view override returns (bool result) {
        result = defaultResult;
        return result;
    }
}
