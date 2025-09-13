// SPDX-License-Identifier: CC0
pragma solidity ^0.8.20;

import "./ProposerManager.sol";
import "./Nightfall.sol";
import "./X509/Certified.sol";

import {Initializable}   from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "forge-std/console.sol";

/// @title Proposers (UUPS-upgradeable)
/// @notice Round-robin proposer manager with staking, cooldowns, and penalties.
/// @dev Key points:
///  - No OwnableUpgradeable: Certified already defines `owner` + `onlyOwner`, so we use that to avoid clashes.
///  - No constructor state: proxies ignore constructors; use `initialize()`.
///  - No `immutable` fields: store them and set in `initialize()`.
///  - Payable seeding split into `bootstrapDefaultProposer{value:...}()` so we can cleanly fund the first stake.
///  - UUPS: `_authorizeUpgrade` guarded by Certified’s `onlyOwner`.
contract RoundRobin is ProposerManager, Certified, UUPSUpgradeable
{
    // -------- config that used to be `immutable` (can’t be immutable in proxies) --------
    uint public STAKE;
    uint public LAZY_PENALTY;
    uint public EXIT_PENALTY;
    uint public COOLDOWN_BLOCKS;
    uint public ROTATION_BLOCKS;

    // -------- existing state --------
    mapping(address => Proposer) public proposers;
    mapping(address => uint)     public pending_withdraws;
    mapping(string => bool)      public proposer_urls;
    // When a proposer voluntarily deregisters:
    // Record the block number. Enforce that they cannot reregister until a certain COOLDOWN_BLOCKS window has passed.
    mapping(address => uint)     public last_exit_block; // for cooldown after voluntary exit

    Proposer private current;
    uint public start_l1_block;
    int  public start_l2_block;
    uint public proposer_count;
    uint public escrow;

    // number of blocks to wait before finalizing a rotation
    uint public constant FINALIZATION_BLOCKS = 64;

    Nightfall private nightfall;

    // ------------------------------------------------------------------------
    // Initializer (replaces constructor for proxies)
    // ------------------------------------------------------------------------
    function initialize(
        address x509_address,
        address sanctionsListAddress,
        uint stake, // Proposer needs to stake this much to join the ring
        uint lazy_penalty, // If a proposer fails to propose when it is their turn, this amount is deducted from their stake
        uint exit_penalty, // When the current proposer voluntarily deregisters, a small but nontrivial penalty is deducted.
        uint cooling_blocks, // This is the number of blocks that must pass before a proposer can reregister after exiting
        uint rotation_blocks
    ) public initializer {
        __UUPSUpgradeable_init();
        __Certified_init(msg.sender, x509_address, sanctionsListAddress);

        // Set Certified.owner since Certified’s constructor won’t run on the proxy
        owner = msg.sender;

        require(cooling_blocks > 0, "Cooling blocks must be > 0");
        require(stake >= exit_penalty, "Stake < exit penalty");
        require(lazy_penalty > exit_penalty, "LazyPenalty <= exit penalty");

        STAKE           = stake;
        LAZY_PENALTY    = lazy_penalty;
        EXIT_PENALTY    = exit_penalty;
        COOLDOWN_BLOCKS = cooling_blocks;
        ROTATION_BLOCKS = rotation_blocks;

        // Wire external dependencies (don’t rely on Certified’s constructor)
        set_x509_address(x509_address);
        set_sanctions_list(sanctionsListAddress);

        // Ring will be seeded later (payable)
        escrow = 0;
    }

    // ------------------------------------------------------------------------
    // Payable bootstrap step (seed the first proposer + deposit initial stake)
    // ------------------------------------------------------------------------
    function bootstrapDefaultProposer(
        address default_proposer_address,
        string calldata default_proposer_url,
        address nightfall_address
    ) external payable onlyOwner {
        console.log("Bootstrapping default proposer...");
        console.log("defaultProposerAddress: ", default_proposer_address);
        console.log("defaultProposerUrl: ", default_proposer_url);
        console.log("nightfallAddress: ", nightfall_address);
        require(proposer_count == 0, "Already bootstrapped");
        require(msg.value == STAKE, "You have not paid the correct staking amount during deployment");
        require(!proposer_urls[default_proposer_url], "URL already in use");

        nightfall = Nightfall(nightfall_address);

        current = Proposer({
            stake: STAKE,
            addr:  default_proposer_address,
            url:   default_proposer_url,
            next_addr:     default_proposer_address,
            previous_addr: default_proposer_address
        });

        escrow += STAKE;
        proposers[default_proposer_address] = current;
        proposer_urls[default_proposer_url] = true;
        proposer_count = 1;

        start_l1_block = block.number;
        start_l2_block = nightfall.layer2_block_number();

        emit ProposerRotated(current);
    }

    // -------- admin wiring (Certified already provides onlyOwner) --------
    function set_x509_address(address x509_address) public onlyOwner {
        x509 = X509(x509_address);
    }

    function set_sanctions_list(address sanctionsListAddress) public onlyOwner {
        sanctionsList = SanctionsListInterface(sanctionsListAddress);
    }

    // we set the nightfall contract address later because we probably don't know it at the time of deployment
    function set_nightfall(address nightfall_address) external onlyOwner {
        nightfall = Nightfall(nightfall_address);
    }

    // -------- core logic --------
    function rotate_proposer() external virtual override {
        require(can_rotate(), "It is not time to rotate the proposer");
        if (nightfall.layer2_block_number() == start_l2_block) {
            lazy_penalize_proposer(current.addr);
        }
        current = proposers[current.next_addr];
        start_l1_block = block.number;
        start_l2_block = nightfall.layer2_block_number();
        emit ProposerRotated(current);
    }

    function add_proposer(
        string calldata proposer_url
    ) external payable override onlyCertified {
        // Enforce cooldown only if they have previously exited
        if (last_exit_block[msg.sender] != 0) {
            require(
                block.number > last_exit_block[msg.sender] + COOLDOWN_BLOCKS,
                "Cooldown period not met"
            );
        }
        require(msg.value == STAKE, "You have not paid the correct staking amount during registration");
        require(proposers[msg.sender].addr == address(0), "You are already a proposer");
        require(!proposer_urls[proposer_url], "This proposer URL is already in use");

        escrow += STAKE;

        // we add the new proposer behind the current proposer, so it will be the last to be called for	        // Insert behind current so it’s called last on first cycle
        // first, insert its address in the linked list
        address current_address  = current.addr;
        address previous_address = current.previous_addr;
        address next_address     = current.next_addr;

        proposers[msg.sender] = Proposer({
            stake: STAKE,
            addr:  msg.sender,
            url:   proposer_url,
            next_addr:     current_address,
            previous_addr: previous_address
        });

        proposers[current_address].previous_addr = msg.sender;
        proposers[previous_address].next_addr    = msg.sender;

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
        _remove_proposer(msg.sender);
    }

    function _remove_proposer(address proposer_address) private {
        require(
            proposers[proposer_address].addr == proposer_address,
            "Proposer does not exist"
        );
        require(proposer_address != address(0), "The proposer address cannot be the zero address");

        if (proposer_address == current.addr) {
            require(proposer_count > 1, "Cannot deregister the only active proposer");
            require(
                proposers[proposer_address].stake >= EXIT_PENALTY,
                "Insufficient stake for exit"
            );

            proposers[proposer_address].stake -= EXIT_PENALTY;
            escrow -= EXIT_PENALTY;

            current = proposers[current.next_addr];
            start_l1_block = block.number;
            start_l2_block = nightfall.layer2_block_number();

            last_exit_block[proposer_address] = block.number;
        }

        // don't forget these only create references
        Proposer storage this_proposer = proposers[proposer_address];
        Proposer storage next_proposer = proposers[this_proposer.next_addr];
        Proposer storage previous_proposer = proposers[this_proposer.previous_addr];

        // break the linked list and reform it without the proposer
        next_proposer.previous_addr = this_proposer.previous_addr;
        previous_proposer.next_addr = this_proposer.next_addr;

        escrow -= this_proposer.stake;
        pending_withdraws[proposer_address] += this_proposer.stake;

        // make sure we can't remove it again
        delete proposer_urls[this_proposer.url];
        // free the URL for reuse
        delete proposers[proposer_address];

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
        return block.number >= start_l1_block + ROTATION_BLOCKS + FINALIZATION_BLOCKS;
    }

    // function to recover the stake after removing a proposer
    function withdraw(uint amount) external {
        uint withdrawable = pending_withdraws[msg.sender];
        require(amount <= withdrawable, "Amount exceeds balance. You are trying to withdraw more than you have");
        pending_withdraws[msg.sender] -= amount;
        payable(msg.sender).transfer(amount);
    }

    // provides a mechanism for fining a lazy proposer
    function lazy_penalize_proposer(address proposer_addr) private {
        Proposer storage proposer = proposers[proposer_addr];
        if (proposer.stake < LAZY_PENALTY) {
            _remove_proposer(proposer_addr);
            return;
        }
        proposer.stake -= LAZY_PENALTY;
        require(escrow >= LAZY_PENALTY, "escrow underflow");
        escrow -= LAZY_PENALTY;
    }

    // --- UUPS guard (use Certified’s onlyOwner) ---
    function _authorizeUpgrade(address) internal override onlyOwner {}

    // Storage gap for future upgrades
    uint256[50] private __gap;
}
