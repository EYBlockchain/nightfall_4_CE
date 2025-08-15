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
                17035205440293355959999152765141543448633724136297475469325716275904799076219,
                9065520758278556095546448031269229233552505442865369773428764815280584507013,
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
                3608046996008026082361488024200288048824670182271115325485728651639269704184,
                3959754566005603064667970065349921387862618986405911504119028649217049673017,
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
            commitments_root: 623948621222247331753330434215219841882414700521245897215773651023767114521,
            nullifier_root: 5626012003977595441102792096342856268135928990590954181023475305010363075697,
            commitments_root_root: 5279554228441733217215028804109502950787603540364801728902128666082823786352,
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
