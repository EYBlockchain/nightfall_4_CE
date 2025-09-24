// SPDX-License-Identifier: CC0
pragma solidity ^0.8.20;

import "./Nightfall.sol";

/// @custom:oz-upgrades-from blockchain_assets/contracts/Nightfall.sol:Nightfall
contract NightfallV2 is Nightfall {
    
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    // simple marker to prove behavior changed post-upgrade
    // This is used in contract unit test.
    function versionMarker() external pure returns (string memory) {
        return "NightfallV2";
    }
}
