// SPDX-License-Identifier: CC0-1.0

pragma solidity ^0.8.0;

abstract contract Allowlist {
    address public owner;
    bool public allowlisting;
    mapping(address => bool) public users;

    modifier onlyOwner() {
        require(msg.sender == owner, "Caller is not the owner");
        _;
    }

    constructor(address owner_) {
        allowlisting = true;
        owner = owner_;  
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
}
