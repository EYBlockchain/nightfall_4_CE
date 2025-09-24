// SPDX-License-Identifier: CC0-1.0
pragma solidity ^0.8.20;

import "./X509Interface.sol";
import "./SanctionsListInterface.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

/// @notice Base contract providing X.509 and sanctions gating for upgradeable contracts.
/// @dev No constructor. Call __Certified_init(...) from the child’s initialize().
abstract contract Certified is Initializable {
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    X509Interface internal x509;
    SanctionsListInterface internal sanctionsList;
    address public owner;

    event OwnershipTransferred(
        address indexed previousOwner,
        address indexed newOwner
    );
    event AuthoritiesUpdated(
        address indexed sanctionsList,
        address indexed x509
    );

    modifier onlyOwner() {
        require(msg.sender == owner, "Certified: caller is not the owner");
        _;
    }

    /// @dev Must be called by inheriting contract inside its `initialize`.
    function __Certified_init(
        address _owner,
        address x509Addr,
        address sanctionsAddr
    ) internal onlyInitializing {
        require(_owner != address(0), "Certified: owner is zero");
        owner = _owner;
        x509 = X509Interface(x509Addr);
        sanctionsList = SanctionsListInterface(sanctionsAddr);
        emit OwnershipTransferred(address(0), _owner);
        emit AuthoritiesUpdated(sanctionsAddr, x509Addr);
    }

    /// @notice Update the authority contracts.
    function setAuthorities(
        address sanctionsListAddress,
        address x509Address
    ) external onlyOwner {
        x509 = X509Interface(x509Address);
        sanctionsList = SanctionsListInterface(sanctionsListAddress);
        emit AuthoritiesUpdated(sanctionsListAddress, x509Address);
    }

    /// @notice Transfer ownership of the Certified gate.
    function transferOwnership(address newOwner) external onlyOwner {
        require(newOwner != address(0), "Certified: new owner is zero");
        emit OwnershipTransferred(owner, newOwner);
        owner = newOwner;
    }

    /// @notice Gate modifier: requires valid X509 and not sanctioned.
    modifier onlyCertified() {
        require(
            x509.x509Check(msg.sender),
            "Certified: not authorised by X509"
        );
        require(
            !sanctionsList.isSanctioned(msg.sender),
            "Certified: address is sanctioned"
        );
        _;
    }

    // Storage gap for future upgrades
    uint256[50] private __gap;
}
