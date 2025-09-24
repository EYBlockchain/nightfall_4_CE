// SPDX-License-Identifier: CC0-1.0

pragma solidity ^0.8.0;
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
abstract contract Allowlist is Initializable {
    address public owner;
    bool public allowlisting;
    mapping(address => bool) public users;

    modifier onlyOwner() {
        require(msg.sender == owner, "Caller is not the owner");
        _;
    }

    // === Initializer ===
    // Call this from proxy's initializer (X509.initialize)
    function __Allowlist_init(address owner_) internal onlyInitializing {
        __Allowlist_init_unchained(owner_);
    }

    function __Allowlist_init_unchained(
        address owner_
    ) internal onlyInitializing {
        require(owner == address(0), "Allowlist: already initialized");
        owner = owner_;
        allowlisting = true;
    }

    // ownership management to mirror old pattern
    function transferOwnership(address newOwner) external onlyOwner {
        require(newOwner != address(0), "Invalid owner");
        owner = newOwner;
    }

    function addUserToAllowlist(address _user) internal {
        users[_user] = true;
    }

    function enableAllowlisting(bool _allowlisting) external onlyOwner {
        allowlisting = _allowlisting;
    }

    function isAllowlisted(address _user) public view returns (bool) {
        if (allowlisting == false) return true; // allowlisting is turned off
        return users[_user];
    }

    // Reserve storage for future upgrades
    uint256[50] private __gap;
}
