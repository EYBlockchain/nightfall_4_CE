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
                16961677748752541128534245196273235858253483350055570576911430034139924291314,
                589271018673332617634015085938733286658506332305114084980349817648391675512,
                21767890284655912613067251487240899553590530231118644174723075138739880012403,
                12341209983432047049663394329812089515284106999628620838977851192389746836928
            ],
            nullifiers: [
                uint256(0),
                uint256(0),
                uint256(0),
                uint256(0)
            ],
            public_data: [
                3671634412532354184577388562170605332878073207269058859177216075475641430421,
                1362455988383284647561903668500255160754175579228401397180246632415138204135,
                5552910810536177546086386907072903996339068532015456737943136533078956996024,
                5102268999071802618002673697332878120667805478388280724842015991138796661265
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
            commitments_root: 16287139121895287214274976617148885772133810504269800404207154262567465044439,
            nullifier_root: 5626012003977595441102792096342856268135928990590954181023475305010363075697,
            commitments_root_root: 5020857426584642760685255880202499052756269783161315727909830531246032171837,
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
