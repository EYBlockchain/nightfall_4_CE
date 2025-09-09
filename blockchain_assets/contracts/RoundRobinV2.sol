// SPDX-License-Identifier: CC0
pragma solidity ^0.8.20;

import "./RoundRobin.sol";

/// Tell OZ validator what this upgrades from (fully-qualified path + name)
/// @custom:oz-upgrades-from blockchain_assets/contracts/RoundRobinUUPS.sol:RoundRobin
contract RoundRobinV2 is RoundRobin {
    /// Behavior change for the test: disable rotations in V2
    /// NOTE: V1's `rotate_proposer` must be `virtual` for this to override cleanly.
    function rotate_proposer() external pure override {
        revert("RoundRobinV2: rotate disabled for test");
    }

    /// Optional: add a tiny helper so you can detect V2 onchain if you want
    function version() external pure returns (string memory) {
        return "RoundRobinV2";
    }
}
