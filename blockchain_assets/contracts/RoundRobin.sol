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
    uint public DING;
    uint public EXIT_PENALTY;
    uint public COOLDOWN_BLOCKS;
    uint public ROTATION_BLOCKS;

    // -------- existing state --------
    mapping(address => Proposer) public proposers;
    mapping(address => uint)     public pending_withdraws;
    mapping(string => bool)      public proposer_urls;
    mapping(address => uint)     public last_exit_block; // for cooldown after voluntary exit

    Proposer private current;
    uint public start_l1_block;
    int  public start_l2_block;
    uint public proposer_count;
    uint public escrow;

    uint public constant FINALIZATION_BLOCKS = 64;

    Nightfall private nightfall;

    // ------------------------------------------------------------------------
    // Dummy constructor: required because `Certified` has a constructor.
    // It WILL NOT run on the proxy; real wiring happens in `initialize()`.
    // ------------------------------------------------------------------------
    // constructor()
    //     // Certified(
    //     //     X509Interface(address(0)),
    //     //     SanctionsListInterface(address(0))
    //     // )
    // {
    //     _disableInitializers(); // lock the implementation
    // }

    // ------------------------------------------------------------------------
    // Initializer (replaces constructor for proxies)
    // ------------------------------------------------------------------------
    function initialize(
        address x509_address,
        address sanctionsListAddress,
        uint stake,
        uint ding,
        uint exit_penalty,
        uint cooling_blocks,
        uint rotation_blocks
    ) public initializer {
        __UUPSUpgradeable_init();
        __Certified_init(msg.sender, x509_address, sanctionsListAddress);

        // Set Certified.owner since Certified’s constructor won’t run on the proxy
        owner = msg.sender;

        require(cooling_blocks > 0, "Cooling blocks must be > 0");
        require(stake >= exit_penalty, "Stake < exit penalty");
        require(ding  >  exit_penalty, "Ding <= exit penalty");

        STAKE           = stake;
        DING            = ding;
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
        require(msg.value == STAKE, "Wrong stake paid");
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

    function set_nightfall(address nightfall_address) external onlyOwner {
        nightfall = Nightfall(nightfall_address);
    }

    // -------- core logic --------
    function rotate_proposer() external virtual override {
        require(can_rotate(), "Not time to rotate");
        if (nightfall.layer2_block_number() == start_l2_block + int(FINALIZATION_BLOCKS)) {
            ding_proposer(current.addr);
        }
        current = proposers[current.next_addr];
        start_l1_block = block.number;
        start_l2_block = nightfall.layer2_block_number();
        emit ProposerRotated(current);
    }

    function add_proposer(
        string calldata proposer_url
    ) external payable override onlyCertified {
        if (last_exit_block[msg.sender] != 0) {
            require(
                block.number > last_exit_block[msg.sender] + COOLDOWN_BLOCKS,
                "Cooldown period not met"
            );
        }
        require(msg.value == STAKE, "Wrong stake paid");
        require(proposers[msg.sender].addr == address(0), "Already a proposer");
        require(!proposer_urls[proposer_url], "URL already in use");

        escrow += STAKE;

        // Insert behind current so it’s called last on first cycle
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
            // first addition: also becomes “next” after current
            proposers[current_address].next_addr = msg.sender;
        }

        proposer_urls[proposer_url] = true;
        current = proposers[current_address];
        proposer_count++;
    }

    function remove_proposer() external override {
        _remove_proposer(msg.sender);
    }

    function _remove_proposer(address proposer_address) private {
        require(
            proposers[proposer_address].addr == proposer_address,
            "Proposer does not exist"
        );
        require(proposer_address != address(0), "Zero address");

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

        Proposer storage this_proposer     = proposers[proposer_address];
        Proposer storage next_proposer     = proposers[this_proposer.next_addr];
        Proposer storage previous_proposer = proposers[this_proposer.previous_addr];

        next_proposer.previous_addr = this_proposer.previous_addr;
        previous_proposer.next_addr = this_proposer.next_addr;

        escrow -= this_proposer.stake;
        pending_withdraws[proposer_address] += this_proposer.stake;

        delete proposer_urls[this_proposer.url];
        delete proposers[proposer_address];

        proposer_count--;

        if (proposer_count == 1) {
            proposers[current.addr].next_addr     = current.addr;
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
        Proposer[] memory list = new Proposer[](proposer_count);
        list[0] = current;
        for (uint i = 1; i < proposer_count; i++) {
            list[i] = proposers[list[i - 1].next_addr];
        }
        return list;
    }

    function can_rotate() private view returns (bool) {
        return block.number >= start_l1_block + ROTATION_BLOCKS + FINALIZATION_BLOCKS;
    }

    function withdraw(uint amount) external {
        uint withdrawable = pending_withdraws[msg.sender];
        require(amount <= withdrawable, "Amount exceeds balance");
        pending_withdraws[msg.sender] -= amount;
        payable(msg.sender).transfer(amount);
    }

    function ding_proposer(address proposer_addr) private {
        Proposer storage p = proposers[proposer_addr];
        int new_stake = int(p.stake) - int(DING);
        if (new_stake < 0) {
            _remove_proposer(proposer_addr);
            return;
        }
        p.stake = uint(new_stake);
        escrow -= DING;
    }

    // --- UUPS guard (use Certified’s onlyOwner) ---
    function _authorizeUpgrade(address) internal override onlyOwner {}

    // Storage gap for future upgrades
    uint256[50] private __gap;
}
