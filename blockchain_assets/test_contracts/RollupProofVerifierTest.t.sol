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
                0xAB5C294B5510F5994AB12BADE56FBC5B15175F9593271C7C184DF0BEB24F617,
                0x1EE718D82B59C02479A7F17ADA7AF4B13082FE1125D8883ACEE89D20456113C1,
                0x2BB9E8F47B7525F2517C3E65602F5BB5B3550D3AA9BA0169891A64A0DE0D5C0A,
                0x294DC095C5ACC33F3207E48D6BB92E970A55BF5822854C22C6ECE820BD642016
            ],
            nullifiers: [
                uint256(0),
                uint256(0),
                uint256(0),
                uint256(0)
            ],
            public_data: [
                5635457032710795443292336466412435576152054133476711170996657113935736264873,
                1620616990106169402261918942780333204210361781917366201332922125106783027011,
                224682908396721863056884040830728054169036463345531268511209115234781527699,
                786074034347489913389436862458988339887712512603487951588576506880704643320
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
            commitments_root: 1326767445992609725286332970354297787956899679694894333625853854300875696203,
            nullifier_root: 5626012003977595441102792096342856268135928990590954181023475305010363075697,
            commitments_root_root: 13230871057247600178800123234490920919802437756299757387723861982742666367714,
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
