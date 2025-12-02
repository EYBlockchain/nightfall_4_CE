// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../contracts/Nightfall.sol";
import "../contracts/X509/X509.sol";
import "../contracts/SanctionsListMock.sol";

// Verifier + VK interface/types
import "../contracts/proof_verification/RollupProofVerifier.sol";
import "../contracts/proof_verification/IVKProvider.sol";
import "../contracts/proof_verification/lib/Types.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

/// @dev Test-only VK provider that returns a fixed VK.
contract TestVKProvider is IVKProvider {
    function vkHash() external pure returns (bytes32) {
        return bytes32(0);
    }
    function getVerificationKey()
        external
        pure
        returns (Types.VerificationKey memory vk)
    {
        assembly {
            // domain_size
            mstore(add(vk, 0x00), 0x2000000)

            // num_inputs

            mstore(add(vk, 0x20), 0x1)

            // sigma_comms_1.x

            mstore(
                mload(add(vk, 0x40)),
                0x16e4a93603ca05c343034436dd29a416846d105b6f18a5a90741614b45e669e8
            )

            // sigma_comms_1.y

            mstore(
                add(mload(add(vk, 0x40)), 0x20),
                0x23ebfad2b8b12897c10e4e7298b132db5240734fef91fa574ca924d9d5dda8ea
            )

            // sigma_comms_2.x

            mstore(
                mload(add(vk, 0x60)),
                0x1edb804e14fe17e3ddca73ab285dd6f0254df1594560d13c92c00dfcf2f56d26
            )

            // sigma_comms_2.y

            mstore(
                add(mload(add(vk, 0x60)), 0x20),
                0x2eea878b321ce17b366825435187bc598701659f321b2e96df77cf7995df0fa1
            )

            // sigma_comms_3.x

            mstore(
                mload(add(vk, 0x80)),
                0x57e6ef0f494e2f1b6bdc71928c8938148ae7e422302d463412286a4825ed00
            )

            // sigma_comms_3.y

            mstore(
                add(mload(add(vk, 0x80)), 0x20),
                0x894f2e4b166fdddfedc4427f6a55c9a18606156a01026def6b87f9a4ccf13f2
            )

            // sigma_comms_4.x

            mstore(
                mload(add(vk, 0xa0)),
                0x1a20edda5dd9378ea6976c2d528e908d0d8cbfc537b138db4f7b8e2437145b57
            )

            // sigma_comms_4.y

            mstore(
                add(mload(add(vk, 0xa0)), 0x20),
                0x30a3779bf47489678cec962d8779bfa6cfedc7b41a97d41ee58c6a45d4c4466
            )

            // sigma_comms_5.x

            mstore(
                mload(add(vk, 0xc0)),
                0x18de0b99ab31f2e254fceeed21aaff5a5afb74211056e80950f779eafafee6aa
            )

            // sigma_comms_5.y

            mstore(
                add(mload(add(vk, 0xc0)), 0x20),
                0x16271aef84194b2ed9973b5cb392efe25cff6f88a52bb159c8c9f9969bbaed6e
            )

            // sigma_comms_6.x

            mstore(
                mload(add(vk, 0xe0)),
                0x22612893d1ba3be7fa0c8b53cb1d3a998e35a5d53015b36eff86b981df508321
            )

            // sigma_comms_6.y

            mstore(
                add(mload(add(vk, 0xe0)), 0x20),
                0xdd8ac2ba291eff940e50fed7c66818da542f44917ae09cf00bc8a042c4c48c3
            )

            // selector_comms_1.x

            mstore(
                mload(add(vk, 0x100)),
                0x71503f8cf18715b80ad1bcd43d3f1ee4280bed3d8d6efeef7a5c4acabf13f4f
            )

            // selector_comms_1.y

            mstore(
                add(mload(add(vk, 0x100)), 0x20),
                0x29e25c4941f0230b454badd29e54b5819ed5c5f3bdb261bebe773e63b66be87f
            )

            // selector_comms_2.x

            mstore(
                mload(add(vk, 0x120)),
                0x9a7b3eb0f034cbfe9064b01da0163bde525aeb628991274664885e9b51c7647
            )

            // selector_comms_2.y

            mstore(
                add(mload(add(vk, 0x120)), 0x20),
                0xd88091f391262c8b0ce75e1e733192f00f73d97fd8c50b4527e3903537b03d8
            )

            // selector_comms_3.x

            mstore(
                mload(add(vk, 0x140)),
                0x151710444f739bace835034464fe44d0d22bec5397fad1ebfd60a77f5e04b605
            )

            // selector_comms_3.y

            mstore(
                add(mload(add(vk, 0x140)), 0x20),
                0x23facd746814dec41ba10b9ecd45098dbb237b0154086bd0a2bea2d15c7e66b
            )

            // selector_comms_4.x

            mstore(
                mload(add(vk, 0x160)),
                0x219e7d5fdbbbe79321771b768553b03a095f67ab600d2d932b88855d0f4db6c4
            )

            // selector_comms_4.y

            mstore(
                add(mload(add(vk, 0x160)), 0x20),
                0xa164fea0645347c166bd9a339b2551e208de1e3a113d016ad861d3df9987359
            )

            // selector_comms_5.x

            mstore(
                mload(add(vk, 0x180)),
                0x26356dc4fd7463d256b8bd7b8992e7468c47e165bf4965815216212ad94e015c
            )

            // selector_comms_5.y

            mstore(
                add(mload(add(vk, 0x180)), 0x20),
                0x16cb74366214b433abc896be497508390bfe10df9088d377587554a9cedc8397
            )

            // selector_comms_6.x

            mstore(
                mload(add(vk, 0x1a0)),
                0x160439dd16ebadfe6471a6ce1bedccd295a88df58c2e3271bf36992a92ca671c
            )

            // selector_comms_6.y

            mstore(
                add(mload(add(vk, 0x1a0)), 0x20),
                0x105769d2038e54a6e31f2a7a56d16fb90f5d311f7ecd0a7a67db2e578361b62e
            )

            // selector_comms_7.x

            mstore(
                mload(add(vk, 0x1c0)),
                0x1d1cd14eb33670fb7589b25745ed4e3aa7019e04eec938203e04f88645e5bf5e
            )

            // selector_comms_7.y

            mstore(
                add(mload(add(vk, 0x1c0)), 0x20),
                0xfc0d67c73abfa922ce68ae317658f811d4f20d290a5afe2d855c97d1f54c078
            )

            // selector_comms_8.x

            mstore(
                mload(add(vk, 0x1e0)),
                0x23cd9c7f04d0a916cf2d94eff97681bdffb6648cead9d46aba2cb93084cfdd29
            )

            // selector_comms_8.y

            mstore(
                add(mload(add(vk, 0x1e0)), 0x20),
                0x2ccbb1b22dfd1ac4d6be76bd528013e32810ff8012b2cb65f8b9fddc42b8fdc6
            )

            // selector_comms_9.x

            mstore(
                mload(add(vk, 0x200)),
                0x29b34babd56e2a2e94645276d60a1290a9536ffccf2cd5ab95e9971db1992fba
            )

            // selector_comms_9.y

            mstore(
                add(mload(add(vk, 0x200)), 0x20),
                0xe25c69e7fa0215f4196ed9402ff97baa156be305798ab4ddc8e63c942e2f85d
            )

            // selector_comms_10.x

            mstore(
                mload(add(vk, 0x220)),
                0x21b0a5dfc37ce228e73b93d0350d502dae2ffec708bf61ab3b0c6c47c47c7c02
            )

            // selector_comms_10.y

            mstore(
                add(mload(add(vk, 0x220)), 0x20),
                0x265a459356b6aa5849f7af90243f64c2cb90ea96f6c8119b065e1765aafaf71c
            )

            // selector_comms_11.x

            mstore(
                mload(add(vk, 0x240)),
                0x2ef2edbc75ce6c8fa8c8b3d24b55582ff5bcaec6c31907c09ada767aa9a323c1
            )

            // selector_comms_11.y

            mstore(
                add(mload(add(vk, 0x240)), 0x20),
                0x243356f176f6286c2a4cda20ef13e31b074ca54799dec7038e28aa7042e024d9
            )

            // selector_comms_12.x

            mstore(
                mload(add(vk, 0x260)),
                0x38fe217b3a711fe54844ee333dcf5e53aff3d907e2ec8534e9f236c0d73eca5
            )

            // selector_comms_12.y

            mstore(
                add(mload(add(vk, 0x260)), 0x20),
                0x2537d05ccb4cfb4fc2063fb94c49ef44e858fe227793beeb5c1950d59b487e82
            )

            // selector_comms_13.x

            mstore(
                mload(add(vk, 0x280)),
                0x28ac9f1946edebd1d328dffad65d1d3eb5a93b416b4dcd702cdb14a59a507e65
            )

            // selector_comms_13.y

            mstore(
                add(mload(add(vk, 0x280)), 0x20),
                0x221b90bfb30131245364bf73b7b321a69f8b1d4ce2701ae3f0529c19c48f4a3f
            )

            // selector_comms_14.x

            mstore(
                mload(add(vk, 0x2a0)),
                0x1830e835607f38920104fe6afa847790824ecd672ae59ffd0ed5b9f17ba5b55e
            )

            // selector_comms_14.y

            mstore(
                add(mload(add(vk, 0x2a0)), 0x20),
                0x20dbf1b5ea6afcb41eeb509300902c1a355c8a12e3f88fb54e1d9cd8b2497595
            )

            // selector_comms_15.x

            mstore(
                mload(add(vk, 0x2c0)),
                0x72ecd6e60f9925b0c85dc5c4ebd6de4fe6aec300a0e4b47b445f63b1320bbe0
            )

            // selector_comms_15.y

            mstore(
                add(mload(add(vk, 0x2c0)), 0x20),
                0x2a494f1540c1cabaf41d08f8168bdcb146ac4e6b257fb593afc0895ffb797930
            )

            // selector_comms_16.x

            mstore(
                mload(add(vk, 0x2e0)),
                0xdbc6bebf1fec9882346d397c153fb1abc7367fda6c3b6269ddb9d15f5c8b162
            )

            // selector_comms_16.y

            mstore(
                add(mload(add(vk, 0x2e0)), 0x20),
                0x1e54728c33242415ef9a8e8bc5d5bcd380e240505eec550942a0439fd2b6316a
            )

            // selector_comms_17.x

            mstore(
                mload(add(vk, 0x300)),
                0x274a44b2389bca62b1a935cc939d3e41172ecd1f853c81d138acdc2252452927
            )

            // selector_comms_17.y

            mstore(
                add(mload(add(vk, 0x300)), 0x20),
                0x1fefbe2d0c0ce4c3888bdba3504913272f06fed1ea308f69f1e43cffbb92732b
            )

            // selector_comms_18.x

            mstore(
                mload(add(vk, 0x320)),
                0x2eb70c2ae0ed6bd2f379ca47a911735f043310476e723086d4b2e4e456f4f2fe
            )

            // selector_comms_18.y

            mstore(
                add(mload(add(vk, 0x320)), 0x20),
                0xcb4a9dec9c7bbd01c5d037b1e3a984409e381dbb0fd14f35ffa8f80484464da
            )

            // k1

            mstore(add(vk, 0x340), 0x1)

            // k2

            mstore(
                add(vk, 0x360),
                0x2f8dd1f1a7583c42c4e12a44e110404c73ca6c94813f85835da4fb7bb1301d4a
            )

            // k3

            mstore(
                add(vk, 0x380),
                0x1ee678a0470a75a6eaa8fe837060498ba828a3703b311d0f77f010424afeb025
            )

            // k4

            mstore(
                add(vk, 0x3a0),
                0x2042a587a90c187b0a087c03e29c968b950b1db26d5c82d666905a6895790c0a
            )

            // k5

            mstore(
                add(vk, 0x3c0),
                0x2e2b91456103698adf57b799969dea1c8f739da5d8d40dd3eb9222db7c81e881
            )

            // k6

            mstore(
                add(vk, 0x3e0),
                0x1f20f5b0adb417179d42df7ddd4410a330afdb03e5c28949665b55adf7d7922d
            )

            // range_table_comm.x

            mstore(
                mload(add(vk, 0x400)),
                0x2f7145aa125d58c2f53f71837f32f4a137bb6e73cd8094677f8d7fec879a088c
            )

            // range_table_comm.y

            mstore(
                add(mload(add(vk, 0x400)), 0x20),
                0x3ebf56a079ca2a9757ef30ea6d62274dc698d9c1de3d1df8050cf58330a8090
            )

            // key_table_comm.x

            mstore(
                mload(add(vk, 0x420)),
                0xf97fb5961b31071ddd4f0276c058922f187410345f36b4dceb9a2b3488d71a5
            )

            // key_table_comm.y

            mstore(
                add(mload(add(vk, 0x420)), 0x20),
                0xcfbd9ad7cf245463d32c3c1c405fd8e728b6306540c43d3a74c40aedaa13e86
            )

            // table_dom_sep_comm.x

            mstore(
                mload(add(vk, 0x440)),
                0x4061fb0f66819bba6461ef43fdc16359989aab1e44e190873bec7cdc888f03d
            )

            // table_dom_sep_comm.y

            mstore(
                add(mload(add(vk, 0x440)), 0x20),
                0x24f34771d16d52aa4ced17c7cee512e6ee44cf22004787bd5c397bc702a0b97
            )

            // q_dom_sep_comm.x

            mstore(
                mload(add(vk, 0x460)),
                0x10aca5984f1913b5fc612b69aed7974d2ca89b62b85c524bd5d5833a0bf509ea
            )

            // q_dom_sep_comm.y

            mstore(
                add(mload(add(vk, 0x460)), 0x20),
                0x271215aec155258a920482965d97d614401e640906dfe75cd29f4db1d874c4da
            )

            // size_inv

            mstore(
                add(vk, 0x480),
                0x30644e5aaf0a66b91f8030da595e7d1c6787b9b45fc54c546729acf1ff053609
            )

            // group_gen

            mstore(
                add(vk, 0x4a0),
                0x2a734ebb326341efa19b0361d9130cd47b26b7488dc6d26eeccd4f3eb878331a
            )

            // group_gen_inv

            mstore(
                add(vk, 0x4c0),
                0x27f035bdb21de9525bcd0d50e993ee185f43327bf6a8efc445d2f3cb9550fe47
            )

            // open_key_g.x

            mstore(mload(add(vk, 0x4e0)), 0x1)

            // open_key_g.y

            mstore(add(mload(add(vk, 0x4e0)), 0x20), 0x2)

            // open_key_h

            let h_ptr := mload(0x40)

            mstore(add(vk, 0x500), h_ptr)

            mstore(
                h_ptr,
                0x198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2
            ) // x0

            mstore(
                add(h_ptr, 0x20),
                0x1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed
            ) // x1

            mstore(
                add(h_ptr, 0x40),
                0x90689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b
            ) // y0

            mstore(
                add(h_ptr, 0x60),
                0x12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa
            ) // y1

            mstore(0x40, add(h_ptr, 0x80))

            // open_key_beta_h

            let beta_h_ptr := mload(0x40)

            mstore(add(vk, 0x520), beta_h_ptr)

            mstore(
                beta_h_ptr,
                0x17cc93077f56f654da727c1def86010339c2b4131094547285adb083e48c197b
            ) // x0

            mstore(
                add(beta_h_ptr, 0x20),
                0x285b1f14edd7e6632340a37dfae9005ff762edcfecfe1c732a7474c0708bef80
            ) // x1

            mstore(
                add(beta_h_ptr, 0x40),
                0x219edfceee1723de674f5b2f6fdb69d9e32dd53b15844956a630d3c7cdaa6ed9
            ) // y0

            mstore(
                add(beta_h_ptr, 0x60),
                0x2bad9a374aec49d329ec66e8f530f68509313450580c4c17c6db5ddb9bde7fd0
            ) // y1

            mstore(0x40, add(beta_h_ptr, 0x80))
        }

        return vk;
    }
}

contract RollupProofVerifierTest is Test {
    Nightfall nightfall;
    RollupProofVerifier verifier;
    X509 x509Contract;

    function setUp() public {
        // Use the test VK provider (full VK in assembly)
        TestVKProvider vk = new TestVKProvider();

        // -------------------------------
        // Deploy RollupProofVerifier UUPS
        // -------------------------------
        RollupProofVerifier impl = new RollupProofVerifier();
        bytes memory initData = abi.encodeCall(
            RollupProofVerifier.initialize,
            (address(vk), address(this)) // vkProvider, initialOwner
        );
        ERC1967Proxy verifierProxy = new ERC1967Proxy(address(impl), initData);
        verifier = RollupProofVerifier(address(verifierProxy));

        // -------------------------------
        // X509 + sanctions
        // -------------------------------
        // IMPORTANT: since the implementation has `constructor(){ _disableInitializers(); }`
        // we must initialize THROUGH THE PROXY, not by calling initialize on the impl.
        X509 x509Impl = new X509();
        bytes memory x509Init = abi.encodeCall(X509.initialize, (address(this)));
        x509Contract = X509(address(new ERC1967Proxy(address(x509Impl), x509Init)));
        
        address sanctionedUser = address(0x123);
        SanctionsListMock sanctionsListMock = new SanctionsListMock(
            sanctionedUser
        );

        // -------------------------------
        // Deploy Nightfall via UUPS proxy
        // -------------------------------
        Nightfall nightfallImpl = new Nightfall();

        bytes memory nfInit = abi.encodeCall(
            Nightfall.initialize,
            (
                5626012003977595441102792096342856268135928990590954181023475305010363075697,
                uint256(0),
                uint256(0),
                int256(0),
                verifier,
                address(x509Contract),
                address(sanctionsListMock)
            )
        );

        ERC1967Proxy nightfallProxy = new ERC1967Proxy(
            address(nightfallImpl),
            nfInit
        );
        nightfall = Nightfall(address(nightfallProxy));
    }

    function testVerifyValidProof() public view {
        string memory hexString = string(
            vm.readFile(
                "./blockchain_assets/test_contracts/blockRollupProof.json"
            )
        );
        bytes memory rollupProof = vm.parseBytes(hexString);

        OnChainTransaction[] memory transactions = new OnChainTransaction[](64);
        transactions[0] = OnChainTransaction({
            fee: uint256(0),
            commitments: [
                635771042160038461983245573025283601683188835677146008410923635038861845890,
                11910083944179089998473492713443941602904639881214727010068586592475323025851,
                18627729925507202786221717168940546377645442951023838256304812722732885377204,
                7330696190229194800820974063519189669714778009207841954152145747952659629758
            ],
            nullifiers: [uint256(0), uint256(0), uint256(0), uint256(0)],
            public_data: [
                6016815775618917255679223309897369488253286026931867663656599130930740985504,
                1618779482463562751484081652214273305875473506436476699834655266856788494340,
                6701453537052539182256371633833579257024574047346530281329659601844695857712,
                5160500547943615058857711535161345356024216369425658536724127636259170578086
            ]
        });

        OnChainTransaction memory emptyTx = OnChainTransaction({
            fee: 0,
            commitments: [uint256(0), 0, 0, 0],
            nullifiers: [uint256(0), 0, 0, 0],
            public_data: [uint256(0), 0, 0, 0]
        });
        for (uint256 i = 1; i < 64; ++i) transactions[i] = emptyTx;

        Block memory blk = Block({
            commitments_root: 8790568928363206804394297340946966097561557610656478367610362967145599462702,
            nullifier_root: 5626012003977595441102792096342856268135928990590954181023475305010363075697,
            commitments_root_root: 9685336808687621011152651517596383829693417568113234202546079283402275385696,
            transactions: transactions,
            rollup_proof: rollupProof
        });

        uint256 block_transactions_length = 64;
        uint256[] memory transaction_hashes = new uint256[](
            block_transactions_length
        );
        for (uint256 i = 0; i < block_transactions_length; ++i) {
            transaction_hashes[i] = nightfall.hash_transaction(
                blk.transactions[i]
            );
        }
        uint256[] memory txh = transaction_hashes;
        for (uint256 len = block_transactions_length; len > 1; len >>= 1) {
            for (uint256 i = 0; i < (len >> 1); ++i) {
                txh[i] = nightfall.sha256_and_shift(
                    abi.encodePacked(txh[i << 1], txh[(i << 1) + 1])
                );
            }
        }
        (bool verified, ) = nightfall.verify_rollup_proof(
            blk,
            transaction_hashes[0]
        );
        assertTrue(verified, "Proof verification failed");
    }

    function testVerifyWrongPublicInputs() public view {
        string memory hexString = string(
            vm.readFile(
                "./blockchain_assets/test_contracts/blockRollupProof.json"
            )
        );
        bytes memory rollupProof = vm.parseBytes(hexString);

        OnChainTransaction[] memory transactions = new OnChainTransaction[](64);
        OnChainTransaction memory emptyTx = OnChainTransaction({
            fee: 0,
            commitments: [uint256(0), 0, 0, 0],
            nullifiers: [uint256(0), 0, 0, 0],
            public_data: [uint256(0), 0, 0, 0]
        });
        for (uint256 i = 0; i < 64; ++i) transactions[i] = emptyTx;

        Block memory blk = Block({
            commitments_root: 0x1,
            nullifier_root: 0x2,
            commitments_root_root: 0x3,
            transactions: transactions,
            rollup_proof: rollupProof
        });

        uint256 block_transactions_length = 64;
        uint256[] memory transaction_hashes = new uint256[](
            block_transactions_length
        );
        for (uint256 i = 0; i < block_transactions_length; ++i) {
            transaction_hashes[i] = nightfall.hash_transaction(
                blk.transactions[i]
            );
        }
        uint256[] memory txh = transaction_hashes;
        for (uint256 len = block_transactions_length; len > 1; len >>= 1) {
            for (uint256 i = 0; i < (len >> 1); ++i) {
                txh[i] = nightfall.sha256_and_shift(
                    abi.encodePacked(txh[i << 1], txh[(i << 1) + 1])
                );
            }
        }

        (bool verified, ) = nightfall.verify_rollup_proof(
            blk,
            transaction_hashes[0]
        );
        assertFalse(
            verified,
            "Expected verification to fail with wrong public inputs"
        );
    }
}
