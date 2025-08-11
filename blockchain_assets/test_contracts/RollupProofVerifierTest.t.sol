// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../contracts/Nightfall.sol";
import "../contracts/proof_verification/RollupProofVerifier.sol";
import "../contracts/SanctionsListMock.sol";

contract RollupProofVerifierTest is Test {
    Nightfall nightfall;
    RollupProofVerifier verifier;
    X509 x509Contract;

    function setUp() public {
        verifier = new RollupProofVerifier();
        x509Contract = new X509(address(this));
        address sanctionedUser = address(0x123);
        SanctionsListMock sanctionsListMock = new SanctionsListMock(
            sanctionedUser
        );
        nightfall = new Nightfall(
            verifier,
            address(x509Contract),
            address(sanctionsListMock)
        );
    }

    function testVerifyValidProof() public {
        // Load valid proof and block fields from JSON or hardcoded (for now)
        string memory hexString = string(vm.readFile("./blockchain_assets/test_contracts/blockRollupProof.json"));
        bytes memory rollupProof = vm.parseBytes(hexString);
        OnChainTransaction[] memory transactions = new OnChainTransaction[](64);
        transactions[0] = OnChainTransaction({
            fee: uint256(0),
            commitments: [
                0x24BDCAC4C8E9DEE4D404F2B6C51DCB69AB86A5552A0334092749050FEA9BF03A,
                uint256(0),
                uint256(0),
                uint256(0)
            ],
            nullifiers: [
                uint256(0),
                uint256(0),
                uint256(0),
                uint256(0)
            ],
            public_data: [
                0x6A6800E8217051833D61E3B6A942BCD02391EBA79F98BA3A79D6BADEF211E8E,
                uint256(0),
                uint256(0),
                uint256(0)
            ]
    });

        // Zero out the rest of the transactions
        OnChainTransaction memory emptyTx = OnChainTransaction({
            fee: 0,
            commitments: [uint256(0), uint256(0), uint256(0), uint256(0)],
            nullifiers: [uint256(0), uint256(0), uint256(0), uint256(0)],
            public_data: [uint256(0), uint256(0), uint256(0), uint256(0)]
        });

        for (uint256 i = 1; i < 64; ++i) {
            transactions[i] = emptyTx;
        }

        Block memory blk = Block({
            commitments_root: 0x23AB6B16D30640DAE93F621B59871295F63F375E5BF1C218065551D2914761BC,
            nullifier_root: 0xC7035BF4A3A237A4C081307ACE7B80B5830974E2B4738D88F15C3EA19C38C71,
            commitments_root_root: 0x1EBF02320521160CB113D49DFB3D5A652651CFC6C4E9B02685828506EF8DEE98,
            transactions: transactions,
            rollup_proof: rollupProof
        });

         // Hash the transactions for the public data
        uint256 block_transactions_length = 64;
        // Hash the transactions for the public data
        uint256[] memory transaction_hashes = new uint256[](
            block_transactions_length
        );
        for (uint256 i = 0; i < block_transactions_length; ++i) {
             transaction_hashes[i] = nightfall.hash_transaction(blk.transactions[i]);
        }
        uint256[] memory transaction_hashes_new = transaction_hashes;
        for (uint256 length = block_transactions_length; length > 1; length >>= 1) {
            for (uint256 i = 0; i < (length >> 1); ++i) {
                // Directly store computed hash in the same array to save memory
                transaction_hashes_new[i] = nightfall.sha256_and_shift(
                    abi.encodePacked(transaction_hashes_new[i << 1], transaction_hashes_new[(i << 1) + 1])
                );
            }
        }
        console2.log("transaction_hashes[0]: ", transaction_hashes[0]);
        (bool verified, ) = nightfall.verify_rollup_proof(blk, transaction_hashes[0]);
        assertTrue(verified, "Proof verification failed");
    }
    function testVerifyWrongPublicInputs() public {
        // Load valid proof and block fields from JSON or hardcoded (for now)
        string memory hexString = string(vm.readFile("./blockchain_assets/test_contracts/blockRollupProof.json"));
        bytes memory rollupProof = vm.parseBytes(hexString);
        OnChainTransaction[] memory transactions = new OnChainTransaction[](64);

        // Zero out the rest of the transactions
        OnChainTransaction memory emptyTx = OnChainTransaction({
            fee: 0,
            commitments: [uint256(0), uint256(0), uint256(0), uint256(0)],
            nullifiers: [uint256(0), uint256(0), uint256(0), uint256(0)],
            public_data: [uint256(0), uint256(0), uint256(0), uint256(0)]
        });

        for (uint256 i = 0; i < 64; ++i) {
            transactions[i] = emptyTx;
        }

        Block memory blk = Block({
            commitments_root: 0x1,
            nullifier_root: 0x2,
            commitments_root_root: 0x3,
            transactions: transactions,
            rollup_proof: rollupProof
        });

        // Hash the transactions for the public data
        uint256 block_transactions_length = 64;
        // Hash the transactions for the public data
        uint256[] memory transaction_hashes = new uint256[](
            block_transactions_length
        );
        for (uint256 i = 0; i < block_transactions_length; ++i) {
             transaction_hashes[i] = nightfall.hash_transaction(blk.transactions[i]);
        }
        uint256[] memory transaction_hashes_new = transaction_hashes;
        for (uint256 length = block_transactions_length; length > 1; length >>= 1) {
            for (uint256 i = 0; i < (length >> 1); ++i) {
                // Directly store computed hash in the same array to save memory
                transaction_hashes_new[i] = nightfall.sha256_and_shift(
                    abi.encodePacked(transaction_hashes_new[i << 1], transaction_hashes_new[(i << 1) + 1])
                );
            }
        }
        (bool verified, ) = nightfall.verify_rollup_proof(blk, transaction_hashes[0]);
        assertFalse(verified, "Proof verification failed");
    }
    
}
