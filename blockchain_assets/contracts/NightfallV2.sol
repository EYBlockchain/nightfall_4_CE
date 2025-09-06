// SPDX-License-Identifier: CC0
pragma solidity ^0.8.20;

import "./Nightfall.sol";

/// @custom:oz-upgrades-from blockchain_assets/contracts/Nightfall.sol:Nightfall
contract NightfallV2 is Nightfall {
    // simple marker to prove behavior changed post-upgrade
    function versionMarker() external pure returns (string memory) {
        return "NightfallV2";
    }
}
