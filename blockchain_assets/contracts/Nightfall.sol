// SPDX-License-Identifier: CC0

import "./proof_verification/INFVerifier.sol";
import {ERC3525, IERC721Receiver, IERC721} from  "@erc-3525/contracts/ERC3525.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IERC1155} from "@openzeppelin/contracts/token/ERC1155/IERC1155.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {IERC3525} from "@erc-3525/contracts/IERC3525.sol";
import {IERC1155Receiver} from "@openzeppelin/contracts/token/ERC1155/IERC1155Receiver.sol";
import {IERC3525Receiver} from "@erc-3525/contracts/IERC3525Receiver.sol";

import "./ProposerManager.sol";
import "./X509/Certified.sol";
import "./X509/X509.sol";
import "forge-std/console2.sol";


pragma solidity ^0.8.20;

enum OperationType {
    DEPOSIT,
    WITHDRAW,
    TRANSFER
}
// in entities.rs, we have defined
// TokenType::ERC20 => 0,
// TokenType::ERC1155 => 1,
// TokenType::ERC721 => 2,
// TokenType::ERC3525=> 3,
// So, the following enum order can't be changed
enum TokenType {
    ERC20,    // 0
    ERC1155, // 1
    ERC721, // 2
    ERC3525// 3
}

// This is the format for a transaction that has been processed by a Proposer and rolled up into a block
// Note: fee is needed here, as we don't want proposer to alter some client's fee but keep the total fee unchanged.
// Such as client_1_fee = 1, client_2_fee = 2, if proposer makes client_1_fee = 2, client_2_fee = 1 when it submits data to blockchain, (fee_sum is unchanged, but individual fee is changed), then proposer can get more fee from client_1 than it should get.
// The publicdata hash won't be the same, therefore we have to keep this `fee` in OnChainTransaction
struct OnChainTransaction {
    uint256 fee; 
    uint256[4] commitments;
    uint256[4] nullifiers;
    uint256[4] public_data; // compressed_secrets that the client submitted
}

struct DepositCommitment {
    uint256 nfTokenid;
    uint256 nfSlotId;
    uint256 value;
    uint256 secretHash;
}

struct DepositFeeState {
    uint256 fee; 
    uint8 escrowed;
    uint8 redeemed;
}

struct WithdrawData {
    uint256 nf_token_id;
    address recipient_address;
    uint256 value;
    uint256 withdraw_fund_salt;
}

// the structure of a rolled up L2 block
struct Block {
    uint256 commitments_root;
    uint256 nullifier_root;
    uint256 commitments_root_root;
    OnChainTransaction[] transactions;
    bytes rollup_proof; // this rollup_proof is public_inputs || a ultra_plonk proof
}

// A struct for remembering the base token that a Nightfall TokenId maps to
struct TokenIdValue {
    address erc_address;
    uint256 token_id;
}

error escrowFundsError();

contract Nightfall is
    Certified,
    IERC721Receiver,
    IERC165,
    IERC1155Receiver,
    IERC3525Receiver
{
    int256 public layer2_block_number = 0; // useful for checking your node is in sync, can be negative (offchain) to indicate a block that is not onchain
    event BlockProposed(int256 indexed layer2_block_number);
    event DepositEscrowed(uint256 nfSlotId, uint256 value);

    mapping(uint256 => DepositFeeState) private feeBinding; // remembers a Deposit's fee
    mapping(bytes32 => uint8) private withdrawalIncluded; // remembers whether a Withdraw can be actioned
    // withdrawalIncluded[key] == 1 means this withdraw transaction is in a Layer 2 block and it's onchain
    // withdrawalIncluded[key] == 0 means this withdraw transaction either hasn't showed on chain or there is no fund to withdraw regarding to this withdraw data
    mapping(uint256 => TokenIdValue) private tokenIdMapping; // Maps Nightfall tokenId to the original ercAddress and tokenId
    uint256 private commitmentRoot = 0;
    uint256 private nullifierRoot = 0;
    uint256 private historicRootsRoot = 0;
    // instance of the proposer_manager interface
    ProposerManager private proposer_manager;
    // instance of the verifier contract
    INFVerifier private verifier;
    // The nightfall token and slot id used for fees
    uint256 private feeId;
    // instance of the certified contract
    Certified private certified;

    modifier only_owner() {
        require(msg.sender == owner, "Only the owner can call this function");
        _;
    }

    constructor(
        INFVerifier addr_verifier,
        address x509_address,
        address sanctionsListAddress
    )
        Certified(
            X509Interface(x509_address),
            SanctionsListInterface(sanctionsListAddress)
        )
    {
        verifier = addr_verifier;
        uint256 computedFeeId;
        assembly {
            // Allocate memory pointer (free memory pointer)
            let ptr := mload(0x40)

            // Store address(this) left-padded to 32 bytes
            mstore(ptr, address())

            // Store uint256(0) just after (32 bytes later)
            mstore(add(ptr, 0x20), 0)

            // Compute keccak256 over the 64 bytes
            computedFeeId := shr(4, keccak256(ptr, 0x40))
        }
        feeId = computedFeeId;
        owner = msg.sender;
    }

    function set_x509_address(address x509_address) external onlyOwner {
        x509 = X509(x509_address);
    }

    function set_sanctions_list(
        address sanctionsListAddress
    ) external onlyOwner {
        sanctionsList = SanctionsListInterface(sanctionsListAddress);
    }

    function set_proposer_manager(
        ProposerManager proposer_manager_address
    ) external only_owner {
        proposer_manager = proposer_manager_address;
    }

    /***********************************************************************************
     * This function is called by the proposer to submit a new L2 block. It's the main  *
     * entry point to the contract.                                                     *
     ************************************************************************************/
    // Proposer will get paid the total_fee contained in all tx in this block if rollup proof is verified
   function propose_block(Block calldata blk) external onlyCertified{
        require(
            proposer_manager.get_current_proposer_address() == msg.sender,
            "Only the current proposer can propose a block"
        );
        // Hash the transactions for the public data
        uint256 block_transactions_length;
        assembly {
            block_transactions_length := calldataload(add(blk, calldataload(add(blk, 0x60))))
        }
        // Hash the transactions for the public data
        uint256[] memory transaction_hashes = new uint256[](
            block_transactions_length
        );
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

        // now we need to decode the public data for each transaction and do something with it
         for (uint i = 0; i < block_transactions_length; i++) {
            // Now we work out what transaction we're dealing with and dispatch it to an appropriate handler.
            // for deposit transaction, we need to filter out the appended dummy deposits, and only process the real deposits.   
            // a dummy deposit transaction will have compressed_secrets aka. public_data = [0,0,0,0]
            bool is_deposit;
            assembly {
                is_deposit := iszero(calldataload(add(blk, add(add(calldataload(add(blk, 0x60)), mul(i, 0x1A0)), 0xC0) )))
            }
            if (is_deposit) {
                bool is_dummy_deposit;
                assembly {
                    is_dummy_deposit := iszero(calldataload(add(blk, add(add(calldataload(add(blk, 0x60)), mul(i, 0x1A0)), 0x140) )))
                }
                if (is_dummy_deposit) {
                    continue;
                }
                uint256 localTotalFee = totalFee;
                uint256 publicData; 

                for (uint j; j < 4; j++) {
                    assembly {
                        publicData := calldataload(add(blk, add(add(calldataload(add(blk, 0x60)), add(0x20, mul(i, 0x1A0))), add(0x120, mul(j, 0x20)))))
                    }
                    if (publicData == 0) {
                        continue;
                    }
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
                is_withdraw := and(iszero(calldataload(add(blk, add(add(calldataload(add(blk, 0x60)), mul(i, 0x1A0)), 0x40) ))), iszero(iszero(calldataload(add(blk, add(add(calldataload(add(blk, 0x60)), mul(i, 0x1A0)), 0xC0) ))))) 
            }
            if (is_withdraw) {
                uint256 withdraw_fund_salt = blk.transactions[i].nullifiers[0];
                WithdrawData memory data = WithdrawData(
                    uint256(blk.transactions[i].public_data[0]),//nf_token_id
                    address(
                        uint160(uint256(blk.transactions[i].public_data[1]))
                    ),//recipient_address
                    blk.transactions[i].public_data[2],//value
                    withdraw_fund_salt
                );
                bytes32 key;
                assembly {
                    let memPtr := mload(0x40)

                    // Store token_id at memPtr
                    mstore(memPtr, mload(data)) // data.nf_token_id

                    // Store recipient_address at memPtr + 32, left-padded to 32 bytes
                    mstore(add(memPtr, 32), mload(add(data, 32)))

                    // Store value at memPtr + 64, left-padded to 32 bytes
                    mstore(add(memPtr, 64), mload(add(data, 64)))

                    // Store salt at memPtr + 96
                    mstore(add(memPtr, 96), mload(add(data, 96)))

                    // Now hash over the full 128 bytes
                    key := keccak256(memPtr, 128)
                }

                // the public data (data) here includes the recipient address. When the recipient attempts to
                // withdraw the amount they are due, they will have to provide the same public data so that the
                // same hash is created. The recipient address will therefore be successfully altered by the caller.
                // Thus, if they provide a different address, the call will fail, if not, all they will do is
                // send the funds for the rightful own, paying gas for the privilege.
                require(withdrawalIncluded[key] == 0, "Funds have already withdrawn");
                // we will give money to the recipient_address once the descrow_funds function is called
                withdrawalIncluded[key] = 1;
                continue;
            }
        }
        // Pay the proposer totalFee
        address proposer = proposer_manager.get_current_proposer_address();
        (bool success, ) = proposer.call{value: totalFee}("");
        require(success, "Failed to transfer the fee to the proposer");

        // Now we update the roots
        commitmentRoot = blk.commitments_root;
        nullifierRoot = blk.nullifier_root;
        historicRootsRoot = blk.commitments_root_root;
        emit BlockProposed(layer2_block_number++);
    }

    function supportsInterface(
        bytes4 interfaceId
    ) external pure override returns (bool) {
        return
            interfaceId == type(IERC165).interfaceId ||
            interfaceId == type(IERC3525Receiver).interfaceId ||
            interfaceId == type(IERC721Receiver).interfaceId ||
            interfaceId == type(IERC1155Receiver).interfaceId;
    }

    // Called by the client to escrow funds so that they can make Deposit transactions. 
    // Currently there is no way to un-escrow funds. This could be implemented with a timelock.
    // The deposited funds are keyed by the sha256 hash of DepositData. When data
    // is succesfully created its key is pushed into the array PendingDeposits so that
    // deposits can be processed in order.
    // Note that client can deposit extra deposit_fee, so client can pay for other transactions in the future, but this is not compulsory which means deposit_fee can be 0.
    // if msg.value - 2 * fee > 0, then client paid deposit_fee, two DepositData will be created, one for the value deposit, and one for the deposit_fee deposit. msg.value = deposit_fee + 2 * fee in this case, client needs to pay for the value deposit and deposit_fee deposit, that's why we have 2 * fee.
    // otherwise if msg.value = fee, it means client only paid for the value deposit, and no deposit_fee deposit is created.
    function escrow_funds(
        uint256 fee, 
        address ercAddress,
        uint256 tokenId,
        uint256 value,
        uint256 secretHash,
        TokenType token_type
) external payable onlyCertified{
        
        uint256 nfTokenId = sha256_and_shift(abi.encode(ercAddress, tokenId));
        tokenIdMapping[nfTokenId] = TokenIdValue(ercAddress, tokenId);

        uint256 nfSlotId = (token_type == TokenType.ERC3525)
        ? uint256(keccak256(abi.encode(ercAddress, IERC3525(ercAddress).slotOf(tokenId)))) >> 4
        : nfTokenId;

        DepositCommitment memory valueCommitment = DepositCommitment(
            nfTokenId,
            nfSlotId,
            value,
            secretHash
        );
        
        uint256 key = sha256_and_shift(abi.encode(valueCommitment));
        require(
            feeBinding[key].escrowed == 0,
            "Funds have already been escrowed for this Deposit"
        );
        if (token_type == TokenType.ERC3525) {
            ERC3525(ercAddress).transferFrom(
                msg.sender,
                address(this),
                tokenId
            );
        } else if (token_type == TokenType.ERC1155) {
            IERC1155(ercAddress).safeTransferFrom(
                msg.sender,
                address(this),
                tokenId,
                value,
                ""
            );
        } else if (token_type == TokenType.ERC721) {
            require(value == 0, "ERC721 tokens should have a value of zero");
            IERC721(ercAddress).safeTransferFrom(
                msg.sender,
                address(this),
                tokenId,
                ""
            );
        } else if (token_type == TokenType.ERC20) {
            require(tokenId == 0, "ERC20 tokens should have a tokenId of 0");
            require(
            IERC20(ercAddress).transferFrom(msg.sender, address(this), value),
            "ERC20 transfer failed"
            );
        }   else {
            revert escrowFundsError();
        }
            feeBinding[key] = DepositFeeState(fee, 1, 0);
            emit DepositEscrowed(nfSlotId, value);
        // Now we also check to see if client made deposit_fee deposit.
        if (msg.value > 2 * fee ) {
            // deposited fee which can be used for future transaction(s)
            uint256 depositFee = msg.value - 2 * fee; 
            
            DepositCommitment memory depositFeeCommitment = DepositCommitment(
                feeId,
                feeId,
                depositFee,
                secretHash
            );
            uint256 depositFeeKey = sha256_and_shift(abi.encode(depositFeeCommitment));
            require(
                feeBinding[depositFeeKey].escrowed == 0,
                "Funds have already been escrowed for this fee Deposit"
            );
            feeBinding[depositFeeKey] = DepositFeeState(fee, 1, 0);
            // nfTokenId for fee commitmemt is keccak256(abi.encode(address(this), 0))
            tokenIdMapping[nfTokenId] = TokenIdValue(address(this), 0);
            emit DepositEscrowed(feeId, depositFee);
        }
    }

    function onERC721Received(
        address,
        address,
        uint256,
        bytes calldata
    ) external pure override returns (bytes4) {
        return 0x150b7a02;
    }

    function onERC1155Received(
        address,
        address,
        uint256,
        uint256,
        bytes calldata
    ) external pure override returns (bytes4) {
        return 0xf23a6e61;
    }

    function onERC1155BatchReceived(
        address,
        address,
        uint256[] calldata,
        uint256[] calldata,
        bytes calldata
    ) external pure override returns (bytes4) {
        revert("Unsupport by Nightfall");
    }

    function onERC3525Received(
        address,
        uint256,
        uint256,
        uint256,
        bytes calldata
    ) external pure returns (bytes4) {
        return 0x009ce20b;
    }

    // Function to the the ercAddress and tokenId of a token if the only information you have is the nfTokenId
    // This is useful if someone transfers a Nightfall token to you and you want to know what the underlying token is.
    function getTokenInfo(uint256 nfTokenId) external view returns (address ercAddress, uint256 tokenId) {
        TokenIdValue memory tokenData = tokenIdMapping[nfTokenId];
        console2.log("in nightfall contract getTokenInfo called with nfTokenId:", nfTokenId);
        console2.log("ercAddress:", tokenData.erc_address);
        console2.log("tokenId:", tokenData.token_id);
        return (tokenData.erc_address, tokenData.token_id);
    }

    // Called by the client to remove their funds from escrow, once they've proved they're entitled to them
    // by submitting a Withdraw transaction that is then proved in a block. We used the compressed_secrets,
    // not because they're really required to prove ownership, but because they are different for every commitment
    // and therefore ensure that the public_data_hash is unique.
    function descrow_funds(
        WithdrawData calldata data,
        TokenType token_type
    ) external payable onlyCertified {
        bytes32 key = keccak256(abi.encode(data));
        require(
            withdrawalIncluded[key] == 1,
            "Either no funds are available to withdraw, or they are already withdrawn"
        );
        // Now that we know the withdraw is present we get the actual erc-address and tokenId from our mapping.
        TokenIdValue memory original = tokenIdMapping[data.nf_token_id];

        if (original.erc_address == address(0)) {
            (bool complete, ) = data.recipient_address.call{value: data.value}(
                ""
            );

            require(complete, "Could not withdraw fee");

            return;
        }

        bool success;
        if (token_type == TokenType.ERC1155) {
            IERC1155(original.erc_address).safeTransferFrom(
                address(this),
                data.recipient_address,
                original.token_id,
                data.value,
                ""
            );
            success = true;
        } else if (token_type == TokenType.ERC721) {
            require(
                data.value == 0,
                "ERC721 tokens should have a value of zero"
            );
            IERC721(original.erc_address).safeTransferFrom(
                address(this),
                data.recipient_address,
                original.token_id,
                ""
            );
            success = true;
        } else if (token_type == TokenType.ERC20) {
            require(
                original.token_id == 0,
                "ERC20 tokens should have a tokenId of 0"
            );
            success = IERC20(original.erc_address).transfer(
                data.recipient_address,
                data.value
            );
        }

        if (success) {
            withdrawalIncluded[key] = 0;
        }
    }

    /********************************************************************************
     * Other functions                                                               *
     *********************************************************************************/

    // Hashes a string of bytes and right-shifts the output by 4
    function sha256_and_shift(
        bytes memory inputs
    ) private view returns (uint256 result) {
        assembly {
            // Allocate memory for hash result (32 bytes)
            let freePtr := mload(0x40)
        
            // Perform SHA-256 hash on inputs
            if iszero(staticcall(gas(), 0x02, add(inputs, 0x20), mload(inputs), freePtr, 0x20)) {
                revert(0, 0)
            }
        
            // Load the hashed value and shift right by 4 bits
            result := shr(4, mload(freePtr))
            
        }
    }

    // hashes the public data in a transaction, for use by the rollup proof
    function hash_transaction(
        OnChainTransaction memory txn
    ) private view returns (uint256) {
        uint256 lastData = txn.public_data[3] & ((1 << 255) - 1);
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
            lastData
        );
        return sha256_and_shift(concatenatedInputs);
    }

    // Verifies the rollup proof
    function verify_rollup_proof(
        Block calldata blk,
        uint256 public_hash
    ) private view returns (bool, uint256) {
        // We need to split the proof into the public data and the actual proof
        // The first 32 bytes of the proof are the sum of fees
        bytes32 feeSum = abi.decode(blk.rollup_proof[:32], (bytes32));
        // convert feeSum to uint256
        uint256 feeSumAsNumber = uint256(feeSum);
        // Then its instance1 x, instance 1y, proof1 x, proof1y, instance2 x, instance2 y, proof2 x, proof2 y
        bytes32 instance1_x = abi.decode(blk.rollup_proof[32:64], (bytes32));
        bytes32 instance1_y = abi.decode(blk.rollup_proof[64:96], (bytes32));
        bytes32 proof1_x = abi.decode(blk.rollup_proof[96:128], (bytes32));
        bytes32 proof1_y = abi.decode(blk.rollup_proof[128:160], (bytes32));
        bytes32 instance2_x = abi.decode(blk.rollup_proof[160:192], (bytes32));
        bytes32 instance2_y = abi.decode(blk.rollup_proof[192:224], (bytes32));
        bytes32 proof2_x = abi.decode(blk.rollup_proof[224:256], (bytes32));
        bytes32 proof2_y = abi.decode(blk.rollup_proof[256:288], (bytes32));

        bytes32[] memory publicInputs = new bytes32[](16); // we need to pass in 16 public inputs
        publicInputs[0] = feeSum;
        publicInputs[1] = bytes32(public_hash);
        publicInputs[2] = bytes32(commitmentRoot);
        publicInputs[3] = bytes32(blk.commitments_root);
        publicInputs[4] = bytes32(nullifierRoot);
        publicInputs[5] = bytes32(blk.nullifier_root);
        publicInputs[6] = bytes32(historicRootsRoot);
        publicInputs[7] = bytes32(blk.commitments_root_root);
        publicInputs[8] = instance1_x;
        publicInputs[9] = instance1_y;
        publicInputs[10] = proof1_x;
        publicInputs[11] = proof1_y;
        publicInputs[12] = instance2_x;
        publicInputs[13] = instance2_y;
        publicInputs[14] = proof2_x;
        publicInputs[15] = proof2_y;

        // we also need to deserialize the transaction public data bytes into fields - but that's easy in Solidity
        bytes memory proof = blk.rollup_proof[288:];
        return (verifier.verify(proof, publicInputs), feeSumAsNumber);
    }

    /// Function that can be called to see if funds are able to be de-escrowed following a withdraw transaction.
    function withdraw_processed(
        WithdrawData calldata data
    ) public view returns (bool) {   
        bytes32 key = keccak256(abi.encode(data));
        return withdrawalIncluded[key] == 1;
    }
}
