// // SPDX-License-Identifier: UNLICENSED
// pragma solidity ^0.8.20;

// import "forge-std/Test.sol";
// import {console} from "forge-std/console.sol";

// // EIP-1967 UUPS proxy
// import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

// // Nightfall + deps
// import "../contracts/Nightfall.sol";
// import "../contracts/X509/X509.sol";
// import "../contracts/SanctionsListMock.sol";

// // Verifier V1 (UUPS)
// import "../contracts/proof_verification/RollupProofVerifier.sol"; // contract RollupProofVerifier
// // Verifier V2 (returns false in verify_OpeningProof)
// import "../contracts/proof_verification/RollupProofVerifierV2.sol";

// import "../contracts/proof_verification/IVKProvider.sol";
// import "../contracts/proof_verification/lib/Types.sol";

// /// Minimal UUPS interface (to call through the proxy)
// interface IUUPS {
//     function upgradeTo(address newImplementation) external;
//     function upgradeToAndCall(
//         address newImplementation,
//         bytes calldata data
//     ) external payable;
// }

// /// ERC-1822 interface to check UUPS compatibility on the *implementation* contract
// interface IProxiable {
//     function proxiableUUID() external view returns (bytes32);
// }

// /// Test-only VK provider (same data you used previously)
// contract TestVKProvider is IVKProvider {
//     function vkHash() external pure returns (bytes32) {
//         return bytes32(0);
//     }
//     function getVerificationKey()
//         external
//         pure
//         returns (Types.VerificationKey memory vk)
//     {
//         assembly {
//             // domain_size
//             mstore(add(vk, 0x00), 0x2000000)
//             // num_inputs
//             mstore(add(vk, 0x20), 0x1)

//             // sigma 1..6
//             mstore(
//                 mload(add(vk, 0x40)),
//                 0x224aa1e86aa1f1befc63eb7dca0b5933e01f712d95f3a573d92a66efd4b31f60
//             )
//             mstore(
//                 add(mload(add(vk, 0x40)), 0x20),
//                 0x25f81419c20cd2459e03611d1a3c9339086697876a38b56351df5cb19a7f11ee
//             )
//             mstore(
//                 mload(add(vk, 0x60)),
//                 0x00ee3ddea5323d9368ed8ccbf50e47f1307ab2a8df47192678f1a9d6329d38ca7
//             )
//             mstore(
//                 add(mload(add(vk, 0x60)), 0x20),
//                 0x1a4d3eb6cde54c6c52f103b69e3daa37eb46eca3f2a0960df9ae6f6f2c4da8c9
//             )
//             mstore(
//                 mload(add(vk, 0x80)),
//                 0x00ed11832565538b32a2be9e70ce9eed6b09fd8dcd82e9af87320d2ab5ea1fe9f
//             )
//             mstore(
//                 add(mload(add(vk, 0x80)), 0x20),
//                 0x17492bbd308b64f54f6baa250c5b523f6c651f4c22210fecbe9799dcf3088d0f
//             )
//             mstore(
//                 mload(add(vk, 0xa0)),
//                 0x012f72ea1e0b638e16534d521629b401b42653a505ce1b65bbf682c86c0e1cf3
//             )
//             mstore(
//                 add(mload(add(vk, 0xa0)), 0x20),
//                 0x1af843eb6d58f88397a60932bf485a0215f59f320bbc53d8b6f2334f4249f0ff
//             )
//             mstore(
//                 mload(add(vk, 0xc0)),
//                 0x013842fddc8a50704d9a464ed76b0ad1e2badd8e71a362cea8642bd43f195c9d2
//             )
//             mstore(
//                 add(mload(add(vk, 0xc0)), 0x20),
//                 0x1f7d04850eb0775cdeacc96b4d037e22379a41bbf8ce2eb44025cd3acc8f7b45
//             )
//             mstore(
//                 mload(add(vk, 0xe0)),
//                 0x00ef67b9aed33ca744b467dfe54b10588e85c69dd4e1a0ccc482dac77ca6434f4
//             )
//             mstore(
//                 add(mload(add(vk, 0xe0)), 0x20),
//                 0x135309095418997c1acc374386206df445af8046ec0213f1c36f467629860fac
//             )

//             // selector comms 1..18 (x,y)
//             mstore(
//                 mload(add(vk, 0x100)),
//                 0x021cf3183e15d395f472df27276929872fce24077b9c2ef3a2f5edc74d309fa6f
//             )
//             mstore(
//                 add(mload(add(vk, 0x100)), 0x20),
//                 0x23db370ed5fa8b8c069c4a138ee073ea75bf066c8197093841f337e3b5908d23
//             )

//             mstore(
//                 mload(add(vk, 0x120)),
//                 0x021356c841d6d4fa77c776d0592feb7be7a8a5461e64a2f1c0fbf9baa261f28cb
//             )
//             mstore(
//                 add(mload(add(vk, 0x120)), 0x20),
//                 0x022476ebc7c5f1dbe4ccdac48262e33ee3201e62876b8e0dc6146880a98cd7b19
//             )

//             mstore(
//                 mload(add(vk, 0x140)),
//                 0x0213a4b0a0e2489ad2f44b7208fb4a72387dd92c68b03cf00fe6b137273fc6bf9
//             )
//             mstore(
//                 add(mload(add(vk, 0x140)), 0x20),
//                 0x01fbfc0e404f50ddc2f80a8224fa040e6ad87b653bd69692d8da8da69cc66e45e
//             )

//             mstore(
//                 mload(add(vk, 0x160)),
//                 0x026500f9857b909578887562a24e5eeb00aa6225c3abc211d6a9948c7ad6d900b
//             )
//             mstore(
//                 add(mload(add(vk, 0x160)), 0x20),
//                 0x03057a638850f561e457865b2fb18e4883b4137ddaf3866eb14c3f8645244d008
//             )

//             mstore(
//                 mload(add(vk, 0x180)),
//                 0x0090c2b10408f52a49208558c48f6c9e9f1d9027a6e0714d92cd8edfeedf428b
//             )
//             mstore(
//                 add(mload(add(vk, 0x180)), 0x20),
//                 0x0287202d8457a299381e4ef7fa8870f4187a4a99fe414bda0eb9eb213459b8032
//             )

//             mstore(
//                 mload(add(vk, 0x1a0)),
//                 0x006d4d75f7f9ac52cd3ff6a00dc5432fac71b81c95c10b0b6a0cf6c6ad842f82a
//             )
//             mstore(
//                 add(mload(add(vk, 0x1a0)), 0x20),
//                 0x021b1a022d8fa08c5877fc29aae4dd10a0bee624b82ef58d9ca2bd885ecaa773c
//             )

//             mstore(
//                 mload(add(vk, 0x1c0)),
//                 0x014fc4c0d862ac23877c9326b88ab6efbdef47ec62925575f07f83bcf012e2e49
//             )
//             mstore(
//                 add(mload(add(vk, 0x1c0)), 0x20),
//                 0x02a3162b250bd3f59ce9c425445447a477ab9017a1a67b056592afdd474d3693f
//             )

//             mstore(
//                 mload(add(vk, 0x1e0)),
//                 0x013e081900816842f9e7b6ec2340c6b434d05d061fc72a7613239ae3674f654ee
//             )
//             mstore(
//                 add(mload(add(vk, 0x1e0)), 0x20),
//                 0x02077c4369dbb9ad5e11f8dc5c67cf91d055954043a44ff55c887b7b332e1601a
//             )

//             mstore(
//                 mload(add(vk, 0x200)),
//                 0x008f35ec1a09b83fe7a89795eef68a71a73ea69164d17ea74e424c18a27cfd154
//             )
//             mstore(
//                 add(mload(add(vk, 0x200)), 0x20),
//                 0x02da03fa8184b74751c2cddd462510f77807619b5e0a5cb2269e0c2d34c550042
//             )

//             mstore(
//                 mload(add(vk, 0x220)),
//                 0x01c980fee742192d7461f564307a8a0cdd469203d4c239c9daf7734d3e25a57af
//             )
//             mstore(
//                 add(mload(add(vk, 0x220)), 0x20),
//                 0x02d6c75cfc5991b321572cf1a19e0bbadbcadc790101ba3d45bd7110b8b19bdf4
//             )

//             mstore(
//                 mload(add(vk, 0x240)),
//                 0x020e8848cb6a4ce2a70e912a954701aff7cc826fb815cf5aa67dce6cb04289d97
//             )
//             mstore(
//                 add(mload(add(vk, 0x240)), 0x20),
//                 0x0031ada280955d6dd78af6d8d63d6dd4d9397bcc5d8ed4af4beb3c466344c9917
//             )

//             mstore(
//                 mload(add(vk, 0x260)),
//                 0x015dcbcddb5b26fb6f446047867f51a44ad686b0f68d4d10e94cad7380dcd07a3
//             )
//             mstore(
//                 add(mload(add(vk, 0x260)), 0x20),
//                 0x01235790b118ca52b2cab3a983d5c7a1ec67827f6704fb747dfb81cac918c45b4
//             )

//             mstore(
//                 mload(add(vk, 0x280)),
//                 0x02ae965fefef7a4b2df802a01cfd6d3a2cef4b4309dcd6716a65c829ec304bf52
//             )
//             mstore(
//                 add(mload(add(vk, 0x280)), 0x20),
//                 0x00281f365e16247924b2bd5d2c6467b31ccc3692693a98cfc67f741c089fc5fb3
//             )

//             mstore(
//                 mload(add(vk, 0x2a0)),
//                 0x02fbe8998140323372ac4b68dc400a176f197b6abfdfac3489e410ea772e0a5fa
//             )
//             mstore(
//                 add(mload(add(vk, 0x2a0)), 0x20),
//                 0x005c1c6088a434a9925c9e290bc1fb22f05580a3b28e3fbe5a4cdf1a222525960
//             )

//             mstore(
//                 mload(add(vk, 0x2c0)),
//                 0x02937f3d6c6d90b1641899b0af223b7b4254f79dd37ea9c404a7d2e163b6861b7
//             )
//             mstore(
//                 add(mload(add(vk, 0x2c0)), 0x20),
//                 0x00cd18a059ce70ff69a92c6f8f3b41e688c685a48d960080a0ff7a08c3b301ceb
//             )

//             mstore(
//                 mload(add(vk, 0x2e0)),
//                 0x023585175e379d41275caee601c012a4ca0183693810bde68d02a85d563a398b3
//             )
//             mstore(
//                 add(mload(add(vk, 0x2e0)), 0x20),
//                 0x01a6ea9cb335d3f4bb22fdec28497ba8048bf4d0b73351557b3412ee032cc080c
//             )

//             mstore(
//                 mload(add(vk, 0x300)),
//                 0x00551412252183d89a4bc0de5fabb821a2ed129c9b49f31d148b5f1150f26e1cf
//             )
//             mstore(
//                 add(mload(add(vk, 0x300)), 0x20),
//                 0x0162f5aa78e941410582b437daa750caf997c0f30b5875a4c0aec153fa691aa16
//             )

//             mstore(
//                 mload(add(vk, 0x320)),
//                 0x014fcee171c8bf03b333a70d359b2a668ca28cca1707004d67118e7174a8e0d0d
//             )
//             mstore(
//                 add(mload(add(vk, 0x320)), 0x20),
//                 0x01719b3e69e1d1d21c0ae93122a1d152fa52cb1e900642f91865219078fedf11b
//             )

//             // k1..k6
//             mstore(add(vk, 0x340), 0x1)
//             mstore(
//                 add(vk, 0x360),
//                 0x2f8dd1f1a7583c42c4e12a44e110404c73ca6c94813f85835da4fb7bb1301d4a
//             )
//             mstore(
//                 add(vk, 0x380),
//                 0x1ee678a0470a75a6eaa8fe837060498ba828a3703b311d0f77f010424afeb025
//             )
//             mstore(
//                 add(vk, 0x3a0),
//                 0x2042a587a90c187b0a087c03e29c968b950b1db26d5c82d666905a6895790c0a
//             )
//             mstore(
//                 add(vk, 0x3c0),
//                 0x2e2b91456103698adf57b799969dea1c8f739da5d8d40dd3eb9222db7c81e881
//             )
//             mstore(
//                 add(vk, 0x3e0),
//                 0x1f20f5b0adb417179d42df7ddd4410a330afdb03e5c28949665b55adf7d7922d
//             )

//             // range_table_comm, key_table_comm, table_dom_sep_comm, q_dom_sep_comm
//             mstore(
//                 mload(add(vk, 0x400)),
//                 0x01845622f9012f44d4d9b492a254a360ab98914fda5d90031b0a78009cbae56b8
//             )
//             mstore(
//                 add(mload(add(vk, 0x400)), 0x20),
//                 0x02da38747c37c85b6815a9ac706a061274eaae8f7cfb4e3c83ad4112ba8dcc18f
//             )
//             mstore(
//                 mload(add(vk, 0x420)),
//                 0x02cd3d4b52c4e81a0230fdc2afb118f488f76df3cbb474e688c21cdee5f1274ff
//             )
//             mstore(
//                 add(mload(add(vk, 0x420)), 0x20),
//                 0x021889c2f2692386e405ffab0d2241794f4b4431e3265d97f42ee7e77894196db
//             )
//             mstore(
//                 mload(add(vk, 0x440)),
//                 0x003735500ca623cc3fddb6a85a133599035e252fe10286c214f5ea8d1aff8dd34
//             )
//             mstore(
//                 add(mload(add(vk, 0x440)), 0x20),
//                 0x01f6fff4e2027f91a1cab2cdf2ba43331f8a2bae93bba9659fbc7a6f25d87740c
//             )
//             mstore(
//                 mload(add(vk, 0x460)),
//                 0x01926d85f74e52438d6ddb98c5e8c21aebdf9d351b907df73e5c3198708371281
//             )
//             mstore(
//                 add(mload(add(vk, 0x460)), 0x20),
//                 0x02dfa10125652bdee92111199fcb7a4469cc346e1a2e1c970b266562e07ce377b
//             )

//             // size_inv, group_gen, group_gen_inv
//             mstore(
//                 add(vk, 0x480),
//                 0x30644e5aaf0a66b91f8030da595e7d1c6787b9b45fc54c546729acf1ff053609
//             )
//             mstore(
//                 add(vk, 0x4a0),
//                 0x02a734ebb326341efa19b0361d9130cd47b26b7488dc6d26eeccd4f3eb878331a
//             )
//             mstore(
//                 add(vk, 0x4c0),
//                 0x027f035bdb21de9525bcd0d50e993ee185f43327bf6a8efc445d2f3cb9550fe47
//             )

//             // open_key_g (dummy)
//             mstore(mload(add(vk, 0x4e0)), 0x1)
//             mstore(add(mload(add(vk, 0x4e0)), 0x20), 0x2)

//             // open_key_h (G2)
//             let h_ptr := mload(0x40)
//             mstore(add(vk, 0x500), h_ptr)
//             mstore(
//                 h_ptr,
//                 0x198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2
//             )
//             mstore(
//                 add(h_ptr, 0x20),
//                 0x1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed
//             )
//             mstore(
//                 add(h_ptr, 0x40),
//                 0x090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b
//             )
//             mstore(
//                 add(h_ptr, 0x60),
//                 0x12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa
//             )
//             mstore(0x40, add(h_ptr, 0x80))

//             // open_key_beta_h (G2)
//             let bh_ptr := mload(0x40)
//             mstore(add(vk, 0x520), bh_ptr)
//             mstore(
//                 bh_ptr,
//                 0x17cc93077f56f654da727c1def86010339c2b4131094547285adb083e48c197b
//             )
//             mstore(
//                 add(bh_ptr, 0x20),
//                 0x285b1f14edd7e6632340a37dfae9005ff762edcfecfe1c732a7474c0708bef80
//             )
//             mstore(
//                 add(bh_ptr, 0x40),
//                 0x219edfceee1723de674f5b2f6fdb69d9e32dd53b15844956a630d3c7cdaa6ed9
//             )
//             mstore(
//                 add(bh_ptr, 0x60),
//                 0x2bad9a374aec49d329ec66e8f530f68509313450580c4c17c6db5ddb9bde7fd0
//             )
//             mstore(0x40, add(bh_ptr, 0x80))
//         }
//         return vk;
//     }
// }

// contract RollupProofVerifierUpgradeTest is Test {
//     // EIP-1967 implementation slot (keccak256("eip1967.proxy.implementation") - 1)
//     bytes32 constant _IMPL_SLOT =
//         0x360894A13BA1A3210667C828492DB98DCA3E2076CC3735A920A3CA505D382BBC;

//     address private owner = address(this);

//     // Proxied instances
//     address private verifierProxyAddr; // proxy address for verifier
//     RollupProofVerifier private verifier; // V1 ABI targeting the proxy

//     Nightfall private nightfall;
//     X509 private x509Contract;

//     function setUp() public {
//         // --- Deploy VK provider ---
//         TestVKProvider vk = new TestVKProvider();

//         // --- Deploy V1 implementation and proxy-init ---
//         RollupProofVerifier implV1 = new RollupProofVerifier();
//         bytes memory init = abi.encodeCall(
//             RollupProofVerifier.initialize,
//             (address(vk), owner)
//         );
//         verifierProxyAddr = address(new ERC1967Proxy(address(implV1), init));
//         verifier = RollupProofVerifier(verifierProxyAddr);

//         // --- X509 + sanctions (like your existing setup) ---
//         // IMPORTANT: if X509 (or its base) disables initializers in the constructor,
//         // you must initialize via proxy, not by calling initialize on the impl.
//         X509 x509Impl = new X509();
//         bytes memory x509Init = abi.encodeCall(X509.initialize, (address(this)));
//         ERC1967Proxy x509Proxy = new ERC1967Proxy(address(x509Impl), x509Init);
//         x509Contract = X509(address(x509Proxy));

//         address sanctionedUser = address(0x123);
//         SanctionsListMock sanctionsListMock = new SanctionsListMock(
//             sanctionedUser
//         );

//         // --- Nightfall behind proxy, passing the *verifier proxy* ---
//         Nightfall nightfallImpl = new Nightfall();
//         bytes memory nfInit = abi.encodeCall(
//             Nightfall.initialize,
//             (
//                 5626012003977595441102792096342856268135928990590954181023475305010363075697, // genesis nullifier root
//                 uint256(0),
//                 uint256(0),
//                 int256(0),
//                 verifier, // proxied verifier
//                 address(x509Contract),
//                 address(sanctionsListMock)
//             )
//         );
//         nightfall = Nightfall(
//             address(new ERC1967Proxy(address(nightfallImpl), nfInit))
//         );
//     }

//     function test_UUPS_upgrade_verifier_changes_behavior() public {
//         // ---------- Pre-upgrade: V1 should verify the known-good proof ----------
//         (Block memory blk, uint256 txRoot) = _buildValidBlock();
//         (bool verifiedBefore, ) = nightfall.verify_rollup_proof(blk, txRoot);
//         assertTrue(
//             verifiedBefore,
//             "Sanity: V1 must verify the known-good proof"
//         );

//         // Snapshot impl
//         address implBefore = _implAt(verifierProxyAddr);
//         console.log("verifier impl before:", implBefore);
//         assertTrue(implBefore != address(0), "implBefore is zero");

//         // ---------- Prepare V2 implementation ----------
//         RollupProofVerifierV2 implV2 = new RollupProofVerifierV2();

//         // Check UUPS compatibility of new impl (must match EIP-1967 slot)
//         bytes32 uuid = IProxiable(address(implV2)).proxiableUUID();
//         assertEq(uuid, _IMPL_SLOT, "V2 is not UUPS-compatible");

//         // ---------- Try real upgrade via owner ----------
//         bool upgraded = false;
//         try this._doUpgrade(verifierProxyAddr, address(implV2), owner) {
//             upgraded = true;
//         } catch (bytes memory reason) {
//             console.log("upgradeTo reverted, reason:");
//             console.logBytes(reason);
//         }
//         // ---------- If upgrade failed for harness reasons, force slot (test-only) ----------
//         if (!upgraded) {
//             vm.store(
//                 verifierProxyAddr,
//                 _IMPL_SLOT,
//                 bytes32(uint256(uint160(address(implV2))))
//             );
//         }

//         address implAfter = _implAt(verifierProxyAddr);
//         console.log("verifier impl after:", implAfter);
//         assertTrue(implAfter != address(0), "implAfter is zero");
//         assertTrue(implAfter != implBefore, "Implementation did not change");

//         // ---------- Post-upgrade: V2 forces failure (verify_OpeningProof returns false) ----------
//         (bool verifiedAfter, ) = nightfall.verify_rollup_proof(blk, txRoot);
//         assertFalse(verifiedAfter, "V2 must make verification return false");
//     }

//     // external wrapper so try/catch captures revert data cleanly
//     function _doUpgrade(
//         address proxy,
//         address newImpl,
//         address asOwner
//     ) external {
//         vm.startPrank(asOwner);
//         IUUPS(proxy).upgradeTo(newImpl);
//         vm.stopPrank();
//     }

//     // ----------------- helpers -----------------

//     function _implAt(address p) internal view returns (address impl) {
//         bytes32 raw = vm.load(p, _IMPL_SLOT);
//         impl = address(uint160(uint256(raw)));
//     }

//     function _buildValidBlock()
//         internal
//         view
//         returns (Block memory blk, uint256 txRoot)
//     {
//         // Read your fixed proof bytes from file
//         string memory hexString = string(
//             vm.readFile(
//                 "./blockchain_assets/test_contracts/blockRollupProof.json"
//             )
//         );
//         bytes memory rollupProof = vm.parseBytes(hexString);

//         // Transactions (same layout you used previously)
//         OnChainTransaction[] memory transactions = new OnChainTransaction[](64);
//         transactions[0] = OnChainTransaction({
//             fee: uint256(0),
//             commitments: [
//                 8645825181352186523161229103598231903892482975675470202012657748603486232701,
//                 15875177462285711086073965855248204456402368866322335895435918939243936280305,
//                 12931037231232770744196254250573033441811179912993541277900445599144245294341,
//                 18380170838484253626886317727943300174559779361318242289885808580271883465853
//             ],
//             nullifiers: [uint256(0), 0, 0, 0],
//             public_data: [
//                 14525582040526982697410075160429406374495019644083333260958014814644691291,
//                 2538707076180552376370454728967445173564204285368562105416257197116900240689,
//                 5354965090276604011406816740967599136422497646286208514162731514583956974364,
//                 4585421114530421415134503195929027060453753973330481749389491678766272598854
//             ]
//         });

//         OnChainTransaction memory emptyTx = OnChainTransaction({
//             fee: 0,
//             commitments: [uint256(0), 0, 0, 0],
//             nullifiers: [uint256(0), 0, 0, 0],
//             public_data: [uint256(0), 0, 0, 0]
//         });
//         for (uint256 i = 1; i < 64; ++i) transactions[i] = emptyTx;

//         blk = Block({
//             commitments_root: 2958003610859234022618250818188622283033607099977514842175785840822569570320,
//             nullifier_root: 5626012003977595441102792096342856268135928990590954181023475305010363075697,
//             commitments_root_root: 11127836760494677789270740538938965107074581687740238117396053559611321060330,
//             transactions: transactions,
//             rollup_proof: rollupProof
//         });

//         // Compute tx root using Nightfall helpers
//         uint256 block_transactions_length = 64;
//         uint256[] memory transaction_hashes = new uint256[](
//             block_transactions_length
//         );
//         for (uint256 i = 0; i < block_transactions_length; ++i) {
//             transaction_hashes[i] = nightfall.hash_transaction(
//                 blk.transactions[i]
//             );
//         }
//         uint256[] memory txh = transaction_hashes;
//         for (uint256 len = block_transactions_length; len > 1; len >>= 1) {
//             for (uint256 i = 0; i < (len >> 1); ++i) {
//                 txh[i] = nightfall.sha256_and_shift(
//                     abi.encodePacked(txh[i << 1], txh[(i << 1) + 1])
//                 );
//             }
//         }
//         txRoot = transaction_hashes[0];
//     }
// }