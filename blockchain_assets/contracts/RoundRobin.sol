// SPDX-License-Identifier: CC0
pragma solidity ^0.8.20;

import "./ProposerManager.sol";
import "./Nightfall.sol";
import "./X509/Certified.sol";


/// @title Proposers
/// @notice An Round-Robin implementation for choosing proposers

contract RoundRobin is ProposerManager, Certified {

    mapping(address => Proposer) public proposers;
    mapping(address => uint) public pending_withdraws;
    mapping(string => bool) public proposer_urls;
    // When a proposer voluntarily deregisters:
    // Record the block number. Enforce that they cannot reregister until a certain COOLDOWN_BLOCKS window has passed.
    mapping(address => uint) public last_exit_block;
    Proposer private current;
    uint public start_l1_block;
    int public start_l2_block;
    uint public proposer_count;
    uint public immutable STAKE;
    uint public immutable DING;
    // When the current proposer voluntarily deregisters, a small but nontrivial penalty is deducted.
    uint public immutable EXIT_PENALTY;
    // This is the number of blocks that must pass before a proposer can reregister after exiting
    uint public immutable COOLDOWN_BLOCKS; 
    uint public immutable ROTATION_BlOCKS;
    uint public escrow = 0;
    uint public constant FINALIZATION_BLOCKS = 64; // number of blocks to wait before finalizing a rotation
    Nightfall private nightfall;

    // instance of the certified contract
    Certified private certified;

    modifier only_owner() {
        require(msg.sender == owner, "Only the owner can call this function");
        _;
    }

    constructor(
        address x509_address,
        address sanctionsListAddress,
        address default_proposer_address,
        string memory default_proposer_url,
        uint stake,
        uint ding,
        uint exit_penalty,
        uint cooling_blocks,
        uint rotation_blocks
    ) 
        Certified(
            X509Interface(x509_address),
            SanctionsListInterface(sanctionsListAddress)
        )
        payable 
    {
        STAKE = stake;
        DING = ding;
        EXIT_PENALTY = exit_penalty;
        COOLDOWN_BLOCKS = cooling_blocks;
        require(
            cooling_blocks > 0,
            "Cooling blocks must be greater than zero"
        );
        require(
            stake >= exit_penalty,
            "Stake must be greater than exit penalty"
        );
        require(
            ding > exit_penalty, 
            "Ding must be greater than exit penalty"
        );
        ROTATION_BlOCKS = rotation_blocks;
        require(
            msg.value == STAKE,
            "You have not paid the correct staking amount during deployment"
        );
        current = Proposer({
            stake: STAKE,
            addr: default_proposer_address,
            url: default_proposer_url,
            next_addr: default_proposer_address,
            previous_addr: default_proposer_address
        });
        escrow += STAKE;
        proposers[default_proposer_address] = current;
        proposer_urls[default_proposer_url] = true;
        proposer_count = 1;
    }

    function set_x509_address(address x509_address) external onlyOwner {
        x509 = X509(x509_address);
    }

    function set_sanctions_list(
        address sanctionsListAddress
    ) external onlyOwner {
        sanctionsList = SanctionsListInterface(sanctionsListAddress);
    }

    // we set the nightfall contract address later because we probably don't know it at the time of deployment
    function set_nightfall(address nightfall_address) external only_owner {
        nightfall = Nightfall(nightfall_address);
    }

    function rotate_proposer() external override {
        require(can_rotate(), "It is not time to rotate the proposer");
        if (nightfall.layer2_block_number() == start_l2_block)
            ding_proposer(current.addr);
        current = proposers[current.next_addr];
        start_l1_block = block.number;
        start_l2_block = nightfall.layer2_block_number();
        emit ProposerRotated(current);
    }

    function add_proposer(
        string calldata proposer_url
    ) external payable override onlyCertified{
        // Enforce cooldown only if they have previously exited
        if (last_exit_block[msg.sender] != 0) {
            require(block.number > last_exit_block[msg.sender] + COOLDOWN_BLOCKS, "Cooldown period not met");
        }
        require(
            msg.value == STAKE,
            "You have not paid the correct staking amount during registration"
        );
        require(
            proposers[msg.sender].addr == address(0),
            "You are already a proposer"
        );
        require(
            !proposer_urls[proposer_url],
            "This proposer URL is already in use"
        );
        escrow += STAKE;

        // we add the new proposer behind the current proposer, so it will be the last to be called for
        // first, insert its address in the linked list
        address current_address = current.addr;
        address previous_address = current.previous_addr;
        address next_address = current.next_addr;

        proposers[msg.sender] = Proposer({
            stake: STAKE,
            addr: msg.sender,
            url: proposer_url,
            next_addr: current_address,
            previous_addr: previous_address
        });

        // then update the proposers that are around it

        proposers[current_address].previous_addr = msg.sender;
        proposers[previous_address].next_addr = msg.sender;
        if (next_address == current_address) {
            // this is the first proposer to be added so it will also be next after the current proposer
            proposers[current_address].next_addr = msg.sender;
        }

        // and finally update metadata
        proposer_urls[proposer_url] = true;
        current = proposers[current_address];
        proposer_count++;
    }

    // an external call can only remove their own proposer
    function remove_proposer() external override {
        remove_proposer(msg.sender);
    }

    function remove_proposer(address proposer_address) private {
        require(
            proposers[proposer_address].addr == proposer_address,
            "This proposer does not exist"
        );
        require(
            proposer_address != address(0),
            "The proposer address cannot be zero"
        );
        if (msg.sender == current.addr) {
            require(
            proposer_count > 1,
            "Cannot deregister the only active proposer"
        );
            require(
            proposers[proposer_address].stake >= EXIT_PENALTY, 
            "Insufficient stake for exit"
        );

        proposers[proposer_address].stake -= EXIT_PENALTY;
        escrow -= EXIT_PENALTY;

        // Rotate to the next proposer before removing
        current = proposers[current.next_addr];
        start_l1_block = block.number;
        start_l2_block = nightfall.layer2_block_number();

        last_exit_block[proposer_address] = block.number;
        }
        Proposer storage this_proposer = proposers[proposer_address]; // don't forget these only create references
        Proposer storage next_proposer = proposers[this_proposer.next_addr];
        Proposer storage previous_proposer = proposers[
            this_proposer.previous_addr
        ];
        // break the linked list and reform it without the proposer
        next_proposer.previous_addr = this_proposer.previous_addr;
        previous_proposer.next_addr = this_proposer.next_addr;
        escrow -= this_proposer.stake;
        pending_withdraws[proposer_address] += this_proposer.stake;
        delete proposers[proposer_address]; // make sure we can't remove it again
        delete proposer_urls[this_proposer.url]; // free the URL for reuse
        proposer_count--;
        // we may now just have a single proposer. If this is the case, we need to link it to itself.
        // it must also be the current proposer so we need to update the current proposer too.
        if (proposer_count == 1) {
            proposers[current.addr].next_addr = current.addr;
            proposers[current.addr].previous_addr = current.addr;
            current = proposers[current.addr];
        }
    }

    function get_current_proposer_address()
        external
        view
        override
        returns (address)
    {
        return current.addr;
    }

    function get_proposers() external view override returns (Proposer[] memory) {
        Proposer[] memory proposer_list = new Proposer[](proposer_count);
        proposer_list[0] = current;
        for (uint i = 1; i < proposer_count; i++) {
            proposer_list[i] = proposers[proposer_list[i - 1].next_addr];
        }
        return proposer_list;
    }

    // this returns true if the current proposer has been in place for ROTATION_BLOCKS
    function can_rotate() private view returns (bool) {
        return block.number >= start_l1_block + ROTATION_BlOCKS + FINALIZATION_BLOCKS;
    }

    // function to recover the stake after removing a proposer
    function withdraw(uint amount) external {
        uint withdrawable = pending_withdraws[msg.sender];
        require(
            amount <= withdrawable,
            "You are trying to withdraw more than you have"
        );
        pending_withdraws[msg.sender] -= amount;
        payable(msg.sender).transfer(amount);
    }

    // provides a mechanism for fining a lazy proposer
    function ding_proposer(address proposer_addr) private {
        Proposer storage proposer = proposers[proposer_addr];
        int new_stake = int(proposer.stake) - int(DING);
        if (new_stake < 0) {
            remove_proposer(proposer_addr);
            return;
        }
        proposer.stake = uint(new_stake);
        escrow -= DING;
    }
}
