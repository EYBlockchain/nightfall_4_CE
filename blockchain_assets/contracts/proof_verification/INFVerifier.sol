// SPDX-License-Identifier: CC0
<<<<<<< HEAD
pragma solidity >=0.8.0;
=======
pragma solidity >=0.8.20;
>>>>>>> Jiajie/workflows

interface INFVerifier {
    function verify(
        bytes calldata accBytes,
        bytes calldata proofBytes, 
        bytes calldata publicInputsHashBytes
    ) external view returns (bool result);
}
