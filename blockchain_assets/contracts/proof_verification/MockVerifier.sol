// SPDX-License-Identifier: CC0
<<<<<<< HEAD
pragma solidity >=0.8.0;
=======
pragma solidity >=0.8.20;
>>>>>>> Jiajie/workflows

import "./INFVerifier.sol";

// Mock verifier contract that just returns `true`.
contract MockVerifier is INFVerifier {
    bool private defaultResult = true;

    function verify(
        bytes calldata accBytes,
        bytes calldata proofBytes, 
        bytes calldata publicInputsHashBytes
    ) external view override returns (bool result) {
        accBytes;
        proofBytes;
        publicInputsHashBytes;
        result = defaultResult;
        return result;
    }
}
