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
    uint256 private _domainSize;      // the only value you plan to change now
    uint256 private _nPublicInputs;   // initialize once to your current correct value
    bytes32 private _vkHash;          // commitment for ops/monitoring

    // IMPORTANT: always keep the gap at the end
    uint256[50] private __gap;

    // -------- Errors & Events --------
    error DomainSizeMustBePowerOfTwo();
    event DomainSizeUpdated(uint256 oldValue, uint256 newValue);
    event VKHashUpdated(bytes32 oldValue, bytes32 newValue);

    // -------- Initialize --------
    function initialize(
        uint256 domainSize_
        // uint256 nPublicInputs_
    ) public initializer {
        __Ownable_init(msg.sender);
        __UUPSUpgradeable_init();

        // Optional: enforce power-of-two domain size (2^k)
        // 0 is invalid, and a power of two has exactly one bit set.
        if (domainSize_ == 0 || (domainSize_ & (domainSize_ - 1)) != 0) {
            revert DomainSizeMustBePowerOfTwo();
        }

        _domainSize     = domainSize_;
        // _nPublicInputs  = nPublicInputs_;
        // _vkHash         = keccak256(abi.encode(_domainSize, _nPublicInputs));
    }

    // -------- Admin setters (you can remove if you prefer "upgrade-only") --------

    function setDomainSize(uint256 newDomainSize) external onlyOwner {
        if (newDomainSize == 0 || (newDomainSize & (newDomainSize - 1)) != 0) {
            revert DomainSizeMustBePowerOfTwo();
        }
        uint256 old = _domainSize;
        _domainSize = newDomainSize;

        // keep commitment consistent
        // bytes32 oldHash = _vkHash;
        // _vkHash = keccak256(abi.encode(_domainSize, _nPublicInputs));

        emit DomainSizeUpdated(old, newDomainSize);
        // if (oldHash != _vkHash) emit VKHashUpdated(oldHash, _vkHash);
    }

    // If you ever need to adjust nPublicInputs later, add a guarded setter like:
    // function setNPublicInputs(uint256 v) external onlyOwner {
    //     uint256 oldN = _nPublicInputs;
    //     _nPublicInputs = v;
    //     bytes32 oldHash = _vkHash;
    //     _vkHash = keccak256(abi.encode(_domainSize, _nPublicInputs));
    //     emit VKHashUpdated(oldHash, _vkHash);
    // }

    // -------- IVKProvider --------

    // function vkHash() external view returns (bytes32) {
    //     return _vkHash;
    // }

    function getVerificationKey() external view returns (Types.VerificationKey memory vk) {
        // Materialize the struct in memory; fill ONLY fields you have now.
        // All unspecified fields default to zero; ensure your verifier tolerates that,
        // or expand storage to include and populate them.
        vk.domain_size    = _domainSize;
        // vk.nPublicInputs  = _nPublicInputs;
        return vk;
    }

    // Convenience getters for ops
    function domainSize() external view returns (uint256) { return _domainSize; }
    // function nPublicInputs() external view returns (uint256) { return _nPublicInputs; }

    // -------- UUPS gate --------
    function _authorizeUpgrade(address) internal override onlyOwner {}
}
