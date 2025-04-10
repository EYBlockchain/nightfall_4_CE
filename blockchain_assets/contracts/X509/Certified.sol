// SPDX-License-Identifier: CC0-1.0

pragma solidity ^0.8.0;

import './X509Interface.sol';
import './SanctionsListInterface.sol';

contract Certified {
    X509Interface x509;
    SanctionsListInterface sanctionsList;
    address public owner;

    modifier onlyOwner() {
        require(msg.sender == owner, "Caller is not the owner");
        _;
    }


    constructor(X509Interface _x509, SanctionsListInterface _sanctionsList) {
        x509 = _x509;
        sanctionsList = _sanctionsList;
        owner = msg.sender;  
    }

    function setAuthorities(address sanctionsListAddress, address x509Address) public onlyOwner {
        x509 = X509Interface(x509Address);
        sanctionsList = SanctionsListInterface(sanctionsListAddress);
    }

    // this modifier checks all of the 'authorisation' contract interfaces to see if we are allowed to transact
    modifier onlyCertified() {
        require(
            x509.x509Check(msg.sender),
            'Certified: You are not authorised to transact using Nightfall'
        );
        require(
            !sanctionsList.isSanctioned(msg.sender),
            'Certified: You are on the Chainalysis sanctions list'
        );
        _;
    }
}
