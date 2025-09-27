// SPDX-License-Identifier: CC0
pragma solidity ^0.8.20;

import "./RoundRobin.sol";

/// @custom:oz-upgrades-from blockchain_assets/contracts/RoundRobin.sol:RoundRobin
contract RoundRobinV3 is RoundRobin {

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    event StakeRequirementUpdated(
        uint oldStake,
        uint newStake,
        uint effectiveFromL1Block
    );

    // Owner can raise/lower the stake required for *new* proposers.
    function setStakeRequirement(uint newStake) external onlyOwner {
        require(newStake >= EXIT_PENALTY, "new stake < exit penalty");
        uint old = STAKE;
        STAKE = newStake;
        emit StakeRequirementUpdated(old, newStake, block.number);
    }

    // optional sanity marker
    function version() external pure returns (string memory) {
        return "RoundRobinV3-stake-config";
    }
}
