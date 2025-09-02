// SPDX-License-Identifier: CC0
pragma solidity ^0.8.20;

import "./proof_verification/INFVerifier.sol";
import {ERC3525, IERC721Receiver, IERC721} from "@erc-3525/contracts/ERC3525.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IERC1155} from "@openzeppelin/contracts/token/ERC1155/IERC1155.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {IERC3525} from "@erc-3525/contracts/IERC3525.sol";
import {IERC1155Receiver} from "@openzeppelin/contracts/token/ERC1155/IERC1155Receiver.sol";
import {IERC3525Receiver} from "@erc-3525/contracts/IERC3525Receiver.sol";

import "./ProposerManager.sol";
import "./X509/Certified.sol";
import "./X509/X509.sol";
import {Initializable}   from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

enum OperationType { DEPOSIT, WITHDRAW, TRANSFER }
enum TokenType { ERC20, ERC1155, ERC721, ERC3525 }

struct OnChainTransaction {
    uint256 fee;
    uint256[4] commitments;
    uint256[4] nullifiers;
    uint256[4] public_data;
}

struct DepositCommitment { uint256 nfTokenid; uint256 nfSlotId; uint256 value; uint256 secretHash; }
struct DepositFeeState { uint256 fee; uint8 escrowed; uint8 redeemed; }

struct WithdrawData {
    uint256 nf_token_id;
    address recipient_address;
    uint256 value;
    uint256 withdraw_fund_salt;
}

struct Block {
    uint256 commitments_root;
    uint256 nullifier_root;
    uint256 commitments_root_root;
    OnChainTransaction[] transactions;
    bytes rollup_proof;
}

struct TokenIdValue { address erc_address; uint256 token_id; }

error escrowFundsError();

/// @title Nightfall (UUPS-upgradeable)
/// @dev Uses `initialize()` (not constructor) and Certified’s `onlyOwner` for auth. `_authorizeUpgrade` gates upgrades.
contract Nightfall is
    Certified,
    UUPSUpgradeable,
    IERC721Receiver,
    IERC165,
    IERC1155Receiver,
    IERC3525Receiver
{
    // int256 public layer2_block_number = 0;
    event BlockProposed(int256 indexed layer2_block_number);
    event DepositEscrowed(uint256 nfSlotId, uint256 value);

    mapping(uint256 => DepositFeeState) private feeBinding;
    mapping(bytes32 => uint8) private withdrawalIncluded;
    mapping(uint256 => TokenIdValue) private tokenIdMapping;

    // uint256 private commitmentRoot = 0;
    // uint256 private nullifierRoot = 5626012003977595441102792096342856268135928990590954181023475305010363075697;
    // uint256 private historicRootsRoot = 0;

    int256 public layer2_block_number;
    uint256 private commitmentRoot;
    uint256 private nullifierRoot;        // set in initialize
    uint256 private historicRootsRoot;

    ProposerManager private proposer_manager;
    INFVerifier private verifier;
    uint256 private feeId;

    // Dummy constructor (won’t run behind proxy) – Certified has a ctor.
    // constructor()
    //     // Certified(X509Interface(address(0)), SanctionsListInterface(address(0)))
    // {
    //     _disableInitializers(); // lock the implementation; no runtime logic here
    // }

    /// @notice Proxy initializer
    function initialize(
        uint256 initialNullifierRoot,
        uint256 initialCommitmentRoot,
        uint256 initialHistoricRootsRoot,
        int256 initialLayer2BlockNumber,
        INFVerifier addr_verifier,
        address x509_address,
        address sanctionsListAddress
    ) public initializer {
        __UUPSUpgradeable_init();
        __Certified_init(msg.sender, x509_address, sanctionsListAddress);

        nullifierRoot = initialNullifierRoot;

        commitmentRoot = initialCommitmentRoot;
        historicRootsRoot = initialHistoricRootsRoot;
        layer2_block_number = initialLayer2BlockNumber;

        // Set Certified owner (its constructor won’t run on proxy)
        owner = msg.sender;

        // Wire authorities directly (avoid external self-calls)
        x509 = X509(x509_address);
        sanctionsList = SanctionsListInterface(sanctionsListAddress);

        verifier = addr_verifier;

        // feeId = keccak256(abi.encode(address(this), 0)) >> 4
        uint256 computedFeeId;
        assembly {
            let ptr := mload(0x40)
            mstore(ptr, address())
            mstore(add(ptr, 0x20), 0)
            computedFeeId := shr(4, keccak256(ptr, 0x40))
        }
        feeId = computedFeeId;
        tokenIdMapping[feeId] = TokenIdValue(address(this), 0);
    }

    function set_x509_address(address x509_address) external onlyOwner {
        x509 = X509(x509_address);
    }

    function set_sanctions_list(address sanctionsListAddress) external onlyOwner {
        sanctionsList = SanctionsListInterface(sanctionsListAddress);
    }

    function set_proposer_manager(ProposerManager proposer_manager_address) external onlyOwner {
        proposer_manager = proposer_manager_address;
    }

    function propose_block(Block calldata blk) external onlyCertified {
        require(
            proposer_manager.get_current_proposer_address() == msg.sender,
            "Only the current proposer can propose a block"
        );

        uint256 block_transactions_length;
        assembly {
            block_transactions_length := calldataload(add(blk, calldataload(add(blk, 0x60))))
        }

        uint256[] memory transaction_hashes = new uint256[](block_transactions_length);
        for (uint256 i = 0; i < block_transactions_length; ++i) {
            transaction_hashes[i] = hash_transaction(blk.transactions[i]);
        }

        uint256[] memory transaction_hashes_new = transaction_hashes;
        for (uint256 length = block_transactions_length; length > 1; length >>= 1) {
            for (uint256 i = 0; i < (length >> 1); ++i) {
                // Directly store computed hash in the same array to save memory
                transaction_hashes_new[i] = sha256_and_shift(
                    abi.encodePacked(transaction_hashes_new[i << 1], transaction_hashes_new[(i << 1) + 1])
                );
            }
        }
        // get the output of verify_rollup_proof
        (bool verified, uint256 totalFee) = verify_rollup_proof(blk, transaction_hashes[0]);
        require(verified, "Rollup proof verification failed");

        for (uint i = 0; i < block_transactions_length; i++) {
            bool is_deposit;
            assembly {
                is_deposit := iszero(calldataload(add(blk, add(add(calldataload(add(blk, 0x60)), mul(i, 0x1A0)), 0xC0))))
            }
            if (is_deposit) {
                bool is_dummy_deposit;
                assembly {
                    is_dummy_deposit := iszero(calldataload(add(blk, add(add(calldataload(add(blk, 0x60)), mul(i, 0x1A0)), 0x140))))
                }
                if (is_dummy_deposit) continue;

                uint256 localTotalFee = totalFee;
                uint256 publicData;
                for (uint j; j < 4; j++) {
                    assembly {
                        publicData := calldataload(add(blk, add(add(calldataload(add(blk, 0x60)), add(0x20, mul(i, 0x1A0))), add(0x120, mul(j, 0x20)))))
                    }
                    if (publicData == 0) continue;
                    localTotalFee += feeBinding[publicData].fee;
                    require(
                        feeBinding[publicData].escrowed == 1 && feeBinding[publicData].redeemed == 0,
                        "Deposit either has not been escrowed or has already been redeemed"
                    );
                    feeBinding[publicData].redeemed = 1;
                }
                totalFee = localTotalFee;
                continue;
            }

            bool is_withdraw;
            assembly {
                is_withdraw := and(
                    iszero(calldataload(add(blk, add(add(calldataload(add(blk, 0x60)), mul(i, 0x1A0)), 0x40)))),
                    iszero(iszero(calldataload(add(blk, add(add(calldataload(add(blk, 0x60)), mul(i, 0x1A0)), 0xC0)))))
                )
            }
            if (is_withdraw) {
                uint256 withdraw_fund_salt = blk.transactions[i].nullifiers[0];
                WithdrawData memory data = WithdrawData(
                    uint256(blk.transactions[i].public_data[0]),
                    address(uint160(uint256(blk.transactions[i].public_data[1]))),
                    blk.transactions[i].public_data[2],
                    withdraw_fund_salt
                );

                bytes32 key;
                assembly {
                    let memPtr := mload(0x40)
                    mstore(memPtr, mload(data))
                    mstore(add(memPtr, 32), mload(add(data, 32)))
                    mstore(add(memPtr, 64), mload(add(data, 64)))
                    mstore(add(memPtr, 96), mload(add(data, 96)))
                    key := keccak256(memPtr, 128)
                }

                require(withdrawalIncluded[key] == 0, "Funds have already withdrawn");
                withdrawalIncluded[key] = 1;
                continue;
            }
        }

        commitmentRoot = blk.commitments_root;
        nullifierRoot = blk.nullifier_root;
        historicRootsRoot = blk.commitments_root_root;

        address proposer = proposer_manager.get_current_proposer_address();
        (bool success, ) = proposer.call{value: totalFee}("");
        require(success, "Failed to transfer the fee to the proposer");

        emit BlockProposed(layer2_block_number++);
    }

    function supportsInterface(bytes4 interfaceId) external pure override returns (bool) {
        return
            interfaceId == type(IERC165).interfaceId ||
            interfaceId == type(IERC3525Receiver).interfaceId ||
            interfaceId == type(IERC721Receiver).interfaceId ||
            interfaceId == type(IERC1155Receiver).interfaceId;
    }

    function escrow_funds(
        uint256 fee,
        address ercAddress,
        uint256 tokenId,
        uint256 value,
        uint256 secretHash,
        TokenType token_type
    ) external payable onlyCertified {
        uint256 nfTokenId = sha256_and_shift(abi.encode(ercAddress, tokenId));
        tokenIdMapping[nfTokenId] = TokenIdValue(ercAddress, tokenId);

        uint256 nfSlotId = (token_type == TokenType.ERC3525)
            ? uint256(keccak256(abi.encode(ercAddress, IERC3525(ercAddress).slotOf(tokenId)))) >> 4
            : nfTokenId;

        DepositCommitment memory valueCommitment = DepositCommitment(nfTokenId, nfSlotId, value, secretHash);
        uint256 key = sha256_and_shift(abi.encode(valueCommitment));

        require(feeBinding[key].escrowed == 0, "Funds have already been escrowed for this Deposit");

        if (token_type == TokenType.ERC3525) {
            ERC3525(ercAddress).transferFrom(msg.sender, address(this), tokenId);
        } else if (token_type == TokenType.ERC1155) {
            IERC1155(ercAddress).safeTransferFrom(msg.sender, address(this), tokenId, value, "");
        } else if (token_type == TokenType.ERC721) {
            require(value == 0, "ERC721 tokens should have a value of zero");
            IERC721(ercAddress).safeTransferFrom(msg.sender, address(this), tokenId, "");
        } else if (token_type == TokenType.ERC20) {
            require(tokenId == 0, "ERC20 tokens should have a tokenId of 0");
            require(IERC20(ercAddress).transferFrom(msg.sender, address(this), value), "ERC20 transfer failed");
        } else {
            revert escrowFundsError();
        }

        feeBinding[key] = DepositFeeState(fee, 1, 0);
        emit DepositEscrowed(nfSlotId, value);

        if (msg.value > 2 * fee) {
            uint256 depositFee = msg.value - 2 * fee;
            DepositCommitment memory depositFeeCommitment = DepositCommitment(feeId, feeId, depositFee, secretHash);
            uint256 depositFeeKey = sha256_and_shift(abi.encode(depositFeeCommitment));
            require(feeBinding[depositFeeKey].escrowed == 0, "Funds have already been escrowed for this fee Deposit");
            feeBinding[depositFeeKey] = DepositFeeState(fee, 1, 0);
            emit DepositEscrowed(feeId, depositFee);
        }
    }

    function onERC721Received(address, address, uint256, bytes calldata) external pure override returns (bytes4) {
        return 0x150b7a02;
    }

    function onERC1155Received(address, address, uint256, uint256, bytes calldata) external pure override returns (bytes4) {
        return 0xf23a6e61;
    }

    function onERC1155BatchReceived(address, address, uint256[] calldata, uint256[] calldata, bytes calldata) external pure override returns (bytes4) {
        revert("Unsupported by Nightfall");
    }

    function onERC3525Received(address, uint256, uint256, uint256, bytes calldata) external pure override returns (bytes4) {
        return 0x009ce20b;
    }

    function getTokenInfo(uint256 nfTokenId) external view returns (address ercAddress, uint256 tokenId) {
        TokenIdValue memory tokenData = tokenIdMapping[nfTokenId];
        return (tokenData.erc_address, tokenData.token_id);
    }

    function descrow_funds(WithdrawData calldata data, TokenType token_type) external payable onlyCertified {
        bytes32 key = keccak256(abi.encode(data));
        require(withdrawalIncluded[key] == 1, "Either no funds are available to withdraw, or they are already withdrawn");

        TokenIdValue memory original = tokenIdMapping[data.nf_token_id];
        if (original.erc_address == address(0)) {
            (bool complete, ) = data.recipient_address.call{value: data.value}("");
            require(complete, "Could not withdraw fee");
            return;
        }

        withdrawalIncluded[key] = 0;
        bool success;

        if (token_type == TokenType.ERC1155) {
            IERC1155(original.erc_address).safeTransferFrom(address(this), data.recipient_address, original.token_id, data.value, "");
            success = true;
        } else if (token_type == TokenType.ERC721) {
            require(data.value == 0, "ERC721 tokens should have a value of zero");
            IERC721(original.erc_address).safeTransferFrom(address(this), data.recipient_address, original.token_id, "");
            success = true;
        } else if (token_type == TokenType.ERC20) {
            require(original.token_id == 0, "ERC20 tokens should have a tokenId of 0");
            success = IERC20(original.erc_address).transfer(data.recipient_address, data.value);
        }

        if (!success) {
            withdrawalIncluded[key] = 1;
        }
    }

    function sha256_and_shift(bytes memory inputs) public view returns (uint256 result) {
        assembly {
            let freePtr := mload(0x40)
            if iszero(staticcall(gas(), 0x02, add(inputs, 0x20), mload(inputs), freePtr, 0x20)) {
                revert(0, 0)
            }
            result := shr(4, mload(freePtr))
        }
    }

     // hashes the public data in a transaction, for use by the rollup proof
    function hash_transaction(
        OnChainTransaction memory txn
    ) public view returns (uint256) {
        bytes memory concatenatedInputs = abi.encode(
            txn.commitments[0],
            txn.commitments[1],
            txn.commitments[2],
            txn.commitments[3],
            txn.nullifiers[0],
            txn.nullifiers[1],
            txn.nullifiers[2],
            txn.nullifiers[3],
            txn.public_data[0],
            txn.public_data[1],
            txn.public_data[2],
            txn.public_data[3]
        );
        return sha256_and_shift(concatenatedInputs);
    }

    // Verifies the rollup proof
    function verify_rollup_proof(
        Block calldata blk,
        uint256 public_hash
    ) public view returns (bool, uint256) {
        // We need to split the proof into the public data and the actual proof
        // The first 32 bytes of the proof are the sum of fees
        bytes32 feeSum = abi.decode(blk.rollup_proof[:32], (bytes32));
        uint256 feeSumAsNumber = uint256(feeSum);
        bytes32[] memory publicInputs = new bytes32[](24); // we need to pass in 24 public inputs
        publicInputs[0] = feeSum;
        publicInputs[1] = bytes32(public_hash);
        publicInputs[2] = bytes32(commitmentRoot);
        publicInputs[3] = bytes32(blk.commitments_root);
        publicInputs[4] = bytes32(nullifierRoot);
        publicInputs[5] = bytes32(blk.nullifier_root);
        publicInputs[6] = bytes32(historicRootsRoot);
        publicInputs[7] = bytes32(blk.commitments_root_root);
        // These are accumulators' comm and proof
        uint256[8] memory acc_low;
        uint256[8] memory acc_high;
        (acc_low[0], acc_high[0]) = splitToLowHigh(uint256(abi.decode(blk.rollup_proof[32:64], (bytes32))));
        (acc_low[1], acc_high[1]) = splitToLowHigh(uint256(abi.decode(blk.rollup_proof[64:96], (bytes32))));
        (acc_low[2], acc_high[2]) = splitToLowHigh(uint256(abi.decode(blk.rollup_proof[96:128], (bytes32))));
        (acc_low[3], acc_high[3]) = splitToLowHigh(uint256(abi.decode(blk.rollup_proof[128:160], (bytes32))));
        (acc_low[4], acc_high[4]) = splitToLowHigh(uint256(abi.decode(blk.rollup_proof[160:192], (bytes32))));
        (acc_low[5], acc_high[5]) = splitToLowHigh(uint256(abi.decode(blk.rollup_proof[192:224], (bytes32))));
        (acc_low[6], acc_high[6]) = splitToLowHigh(uint256(abi.decode(blk.rollup_proof[224:256], (bytes32))));
        (acc_low[7], acc_high[7]) = splitToLowHigh(uint256(abi.decode(blk.rollup_proof[256:288], (bytes32))));

        // Assign to publicInputs
        for (uint i = 0; i < 8; i++) {
            publicInputs[8 + i * 2] = bytes32(acc_low[i]);
            publicInputs[9 + i * 2] = bytes32(acc_high[i]);
        }

        uint256 publicInputsBytes_computed = uint256(
             sha256_and_shift(abi.encodePacked(publicInputs))
        );
        publicInputsBytes_computed = publicInputsBytes_computed % 21888242871839275222246405745257275088548364400416034343698204186575808495617;
        bytes memory publicInputsBytes = abi.encodePacked(publicInputsBytes_computed);

        return (verifier.verify(blk.rollup_proof[32:288], blk.rollup_proof[288:], publicInputsBytes), feeSumAsNumber);
    }

    function splitToLowHigh(uint256 value) internal pure returns (uint256 low, uint256 high) {
        low = value & ((1 << 248) - 1);
        high = value >> 248;
    }

    function withdraw_processed(WithdrawData calldata data) public view returns (bool) {
        bytes32 key = keccak256(abi.encode(data));
        return withdrawalIncluded[key] == 1;
    }

    // --- UUPS guard ---
    function _authorizeUpgrade(address) internal override onlyOwner {}

    uint256[50] private __gap;
}
