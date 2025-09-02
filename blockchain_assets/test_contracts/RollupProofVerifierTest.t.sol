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
                12636195353654328471998867540792098181543535393908718011036979695792468596205,
                10638264250042184148427634169810061643228191491217528855026782186841530134827,
                21010874601036789660255638071400995349455588572429092586740919396751777434273,
                8347874306573774745479064806519662140108752051212995737745295549708583093990
            ],
            nullifiers: [
                uint256(0),
                uint256(0),
                uint256(0),
                uint256(0)
            ],
            public_data: [
                2983654255365128391349164202699154417638512746944907936151544545343364286718,
                6262441667657087776336242406216989854794223382820639315339347392518825188336,
                4600866439279334459925735189803796802166204625592488468809770166792757609427,
                4852314477403230768593765801818441196058057227458263757169659577610742936523
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
            commitments_root: 2004340799556478277714592300363008354590261464418484264923952898185058141295,
            nullifier_root: 5626012003977595441102792096342856268135928990590954181023475305010363075697,
            commitments_root_root: 2497190818530359912738216756525228448406461839246795822793641580727494963294,
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
