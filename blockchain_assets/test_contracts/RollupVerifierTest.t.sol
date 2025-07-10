// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../contracts/Nightfall.sol";
import "../contracts/proof_verification/RollupVerifier.sol";
import "../contracts/SanctionsListMock.sol";
import "forge-std/console2.sol";

contract NightfallVerifierTest is Test {
    Nightfall nightfall;
    RollupVerifier verifier;
    X509 x509Contract;

    function setUp() public {
        verifier = new RollupVerifier();
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
     console.log("rollup_proof:");
    console2.logBytes(rollupProof);
    console.log("rollup_proof length:", rollupProof.length);


        OnChainTransaction[] memory transactions = new OnChainTransaction[](64);
        transactions[0] = OnChainTransaction({
    fee: 1,
    commitments: [
                15392351480021048203930963145242234931291562469394646180004177305873539759707,
                18733568117644813939395418613955016050166314086467640784322196204361374209493,
                0,
                0
    ],
    nullifiers: [
                uint256(0),
                0,
                0,
                0
    ],
    public_data: [
                6621752246873859139838072872206134400407912793669465530530173505215706218417,
                2916144407822975174036791482540787921472104446237086589774849043505523554367,
                0,
                0
    ]
});

 // Zero out the rest of the transactions
    OnChainTransaction memory emptyTx = OnChainTransaction({
        fee: 0,
        commitments: [uint256(0), 0, 0, 0],
        nullifiers: [uint256(0), 0, 0, 0],
        public_data: [uint256(0), 0, 0, 0]
    });

    for (uint256 i = 1; i < 64; ++i) {
        transactions[i] = emptyTx;
    }

        Block memory blk = Block({
            commitments_root: 18439016711412260618672493514077720303865169596918994323081924644547288257451,
            nullifier_root: 5626012003977595441102792096342856268135928990590954181023475305010363075697,
            commitments_root_root: 5257425180143297814316082854138125443161646206268696786907660751879901223481,
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
        // console2.log("transaction_hashes_new[0]: ", transaction_hashes_new[0]);
        // Jiajie: todo, make this a public function in Nightfall, so we can call it directly without needing to compute the hashes again
        (bool verified, uint256 totalFee) = nightfall.verify_rollup_proof(blk, transaction_hashes[0]);
        // (bool verified, uint256 totalFee) = nightfall.verify_rollup_proof(blk, uint256(0));
        // assert that the proof is verified
        assertTrue(verified, "Proof verification failed");
        console2.log("verified: ", verified);
        console2.log("totalFee: ", totalFee);
    }
}
