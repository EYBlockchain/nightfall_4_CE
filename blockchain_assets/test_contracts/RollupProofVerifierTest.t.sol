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
                9198439142478967502697636509931965151634727417146896125353819678293330536347,
                16286838217560047089080811460758219457160299666854794689508553263322489797281,
                20713223312993921584199628400250790051437555902158345286503557532519393720660,
                18749504252154654491947701778442957059911974934978965838792124933030463339212
            ],
            nullifiers: [
                uint256(0),
                uint256(0),
                uint256(0),
                uint256(0)
            ],
            public_data: [
                633519080992023385748464646036436608555128945835282356737684777027132405623,
                452980860958441247012245198514945511831844269137834234855514766087565554596,
                6132476519940381476074851771766465401407066379106513934962088888291192621495,
                1268235300461599465541360796234029347530012672395351765212346595679099287210
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
            commitments_root: 19845722556425664244107200632550782513058278546086852058616449534809158322184,
            nullifier_root: 5626012003977595441102792096342856268135928990590954181023475305010363075697,
            commitments_root_root: 9933439041481830185178143501093826543136835451097741505674578970921115107864,
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
    // function testVerifyWrongPublicInputs() public {
    //     // Load valid proof and block fields from JSON or hardcoded (for now)
    //     string memory hexString = string(vm.readFile("./blockchain_assets/test_contracts/blockRollupProof.json"));
    //     bytes memory rollupProof = vm.parseBytes(hexString);
    //     OnChainTransaction[] memory transactions = new OnChainTransaction[](64);

    //     // Zero out the rest of the transactions
    //     OnChainTransaction memory emptyTx = OnChainTransaction({
    //         fee: 0,
    //         commitments: [uint256(0), uint256(0), uint256(0), uint256(0)],
    //         nullifiers: [uint256(0), uint256(0), uint256(0), uint256(0)],
    //         public_data: [uint256(0), uint256(0), uint256(0), uint256(0)]
    //     });

    //     for (uint256 i = 0; i < 64; ++i) {
    //         transactions[i] = emptyTx;
    //     }

    //     Block memory blk = Block({
    //         commitments_root: 0x1,
    //         nullifier_root: 0x2,
    //         commitments_root_root: 0x3,
    //         transactions: transactions,
    //         rollup_proof: rollupProof
    //     });

    //     // Hash the transactions for the public data
    //     uint256 block_transactions_length = 64;
    //     // Hash the transactions for the public data
    //     uint256[] memory transaction_hashes = new uint256[](
    //         block_transactions_length
    //     );
    //     for (uint256 i = 0; i < block_transactions_length; ++i) {
    //          transaction_hashes[i] = nightfall.hash_transaction(blk.transactions[i]);
    //     }
    //     uint256[] memory transaction_hashes_new = transaction_hashes;
    //     for (uint256 length = block_transactions_length; length > 1; length >>= 1) {
    //         for (uint256 i = 0; i < (length >> 1); ++i) {
    //             // Directly store computed hash in the same array to save memory
    //             transaction_hashes_new[i] = nightfall.sha256_and_shift(
    //                 abi.encodePacked(transaction_hashes_new[i << 1], transaction_hashes_new[(i << 1) + 1])
    //             );
    //         }
    //     }
    //     (bool verified, ) = nightfall.verify_rollup_proof(blk, transaction_hashes[0]);
    //     assertFalse(verified, "Proof verification failed");
    // }
    
}
