// SPDX-License-Identifier: CC0
pragma solidity >=0.8.0;


import "./Types.sol";

interface IVKProvider {
    function getVerificationKey() external view returns (Types.VerificationKey memory vk);
    // function vkHash() external view returns (bytes32);
}

