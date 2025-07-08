// SPDX-License-Identifier: CC0
pragma solidity ^0.8.20;

/// @title Proposers
/// @notice An interface for any contract that is used to choose proposers

struct Proposer {
    uint stake;
    address addr;
    string url;
    address next_addr;
    address previous_addr;
}

interface ProposerManager {
    event ProposerRotated(Proposer proposer);

    /// @notice Rotates the proposer; errors if rotation is not currently allowed
    // Emits ProposerRotated event
    function rotate_proposer() external;
    function add_proposer(string calldata proposer_url) external payable;
    function remove_proposer() external;
    function get_current_proposer_address() external view returns(address);
    function get_proposers() external view returns (Proposer[] memory);
}
