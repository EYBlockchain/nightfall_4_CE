// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

import "./Types.sol";
import "./IVKProvider.sol";

contract RollupProofVerificationKeyUUPS is
    Initializable,
    UUPSUpgradeable,
    OwnableUpgradeable,
    IVKProvider
{
    // -------- Storage v1 --------
    Types.VerificationKey private _vk;     // the whole VK, canonical source
    bytes32               private _vkHash; // keccak256(abi.encode(_vk))
    uint64                private _vkVersion; // increment on every replacement

    // Gap for future upgrades
    uint256[46] private __gap;

    // -------- Events --------
    event VKInitialized(bytes32 vkHash, uint64 version);
    event VKReplaced(bytes32 oldHash, bytes32 newHash, uint64 newVersion);

    function _decodeAndSanity(bytes calldata vkBlob) private pure returns (Types.VerificationKey memory vk) {
        vk = abi.decode(vkBlob, (Types.VerificationKey));
    }

    // -------- Initialize --------
    /// @notice First-time initialization with a full VK.
    /// @param vkBlob ABI-encoded Types.VerificationKey (abi.encode(vk))
    function initialize(bytes calldata vkBlob) public initializer {
        __Ownable_init(msg.sender);
        __UUPSUpgradeable_init();

        Types.VerificationKey memory vk = _decodeAndSanity(vkBlob);
        _vk       = vk;
        _vkHash   = keccak256(vkBlob);
        _vkVersion = 1;
        emit VKInitialized(_vkHash, _vkVersion);
    }

    // -------- Whole-VK replacement (atomic) --------
    /// @notice Replace the entire VerificationKey atomically.
    /// @param vkBlob ABI-encoded Types.VerificationKey (abi.encode(vk))
    function replaceVK(bytes calldata vkBlob) external onlyOwner {
        Types.VerificationKey memory vk = _decodeAndSanity(vkBlob);

        bytes32 oldHash = _vkHash;
        _vk       = vk;
        _vkHash   = keccak256(vkBlob);
        _vkVersion += 1;

        emit VKReplaced(oldHash, _vkHash, _vkVersion);
    }

    // -------- IVKProvider --------
    function getVerificationKey() external view returns (Types.VerificationKey memory) {
        return _vk;
    }

    function vkHash() external view returns (bytes32) {
        return _vkHash;
    }

    // (Optional) operational getters
    function vkVersion() external view returns (uint64) { return _vkVersion; }

    // -------- UUPS gate --------
    function _authorizeUpgrade(address) internal override onlyOwner {}
}
