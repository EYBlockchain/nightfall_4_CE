// SPDX-License-Identifier: Apache-2.0 

    // This contract is made to test create_vk_contract function.
 

    pragma solidity ^0.8.20; 

    import "./Types.sol"; 

        
    library UltraPlonkVerificationKey { 


        function getVerificationKeyHash() internal pure returns (bytes32) {  

            return 0x1466f0da1c60df4dfef29ccab3f988bd9eaca22a792dd82d306c2ff29707775a; 

        } 


        function getVerificationKey() internal pure returns (Types.VerificationKey memory vk) { 

            assembly { 

            // domain_size
            mstore(add(vk, 0x00), 0x200) 

            // num_inputs 

            mstore(add(vk, 0x20), 0x1) 

            // sigma_comms_1.x 

            mstore( 

                mload(add(vk, 0x40)), 

                0x2f5bb5fdb1e276ab3296410b2e37950dbec8fb71d33fd8c06ff37187867e64c2 

            ) 

            // sigma_comms_1.y 

            mstore( 

                add(mload(add(vk, 0x40)), 0x20), 

                0xe93af618a1f0c36812d866e64fc1e67aae9a38da0b0dbb892a81b9596ca9526 

            ) 

            // sigma_comms_2.x 

            mstore( 

                mload(add(vk, 0x60)), 

                0x191347d5516c89251606c73f1aaede230315cce8d119dbb8f85ec7adca58fecf 

            ) 

            // sigma_comms_2.y 

            mstore( 

                add(mload(add(vk, 0x60)), 0x20), 

                0xd51da4facef5a9a1f66cae614a9729ae73cba7e910de1e398890bfb19479e4d 

            ) 

            // sigma_comms_3.x 

            mstore( 

                mload(add(vk, 0x80)), 

                0x152dde58eab59643ac5e43697c9f224183b4fa11e3a1a2a159c775ab2123598b 

            ) 

            // sigma_comms_3.y 

            mstore( 

                add(mload(add(vk, 0x80)), 0x20), 

                0x27dc6260745ca0d7d34369926a2fb01a2b5f9dad22284afcd741db392c136572 

            ) 

            // sigma_comms_4.x 

            mstore( 

                mload(add(vk, 0xa0)), 

                0x2f23dfe6b5f7375f0e0d27f7386b95472acb6e307e6794387d43b8a41851a738 

            ) 

            // sigma_comms_4.y 
 
            mstore( 

                add(mload(add(vk, 0xa0)), 0x20), 

                0x1aa7e43e0c46b17121b3e336e2f0ddf21920247fe3f163dcb86277533e6d5368 

            ) 

            // sigma_comms_5.x 

            mstore( 

                mload(add(vk, 0xc0)), 

                0xc2bbd36af72b26237a1a65c5078e16d52ee5815037b8adcbeeae664b12d09ab 

            ) 

            // sigma_comms_5.y 

            mstore( 

                add(mload(add(vk, 0xc0)), 0x20), 

                0xd6522d528f18b34b9eca3da372b38355441ebc1052ebd6336fbad629dd69530 

            ) 

            // sigma_comms_6.x 

                mstore( 

                mload(add(vk, 0xe0)), 

                0x2b6b34697edc9241665f312e085493a73d96c936270084a53b7e79944f5030c7 

            ) 

            // sigma_comms_6.y 

            mstore( 

                add(mload(add(vk, 0xe0)), 0x20), 

                0x2c2fdd8c0ef5d94fcfefc694906a40183b7ea89ea3f57632cc4deb833b576eb1 

            ) 

            // selector_comms_1.x 

            mstore( 

                mload(add(vk, 0x100)), 
  
                0x2c070e98c8202fcc3a8cac5dfcab636a125c2c53f7ba36a2178c545131db2d9f 

            ) 

            // selector_comms_1.y 

            mstore( 

                add(mload(add(vk, 0x100)), 0x20), 

                0x1a3298b6427790a076429aaf4fd8554a022989d298e6636f295ec239ede33287 

            ) 

            // selector_comms_2.x 

            mstore( 

                mload(add(vk, 0x120)), 

                0x2c070e98c8202fcc3a8cac5dfcab636a125c2c53f7ba36a2178c545131db2d9f 

            ) 

            // selector_comms_2.y 

            mstore( 

                add(mload(add(vk, 0x120)), 0x20), 

                0x1a3298b6427790a076429aaf4fd8554a022989d298e6636f295ec239ede33287 

            ) 

            // selector_comms_3.x 

            mstore( 

                mload(add(vk, 0x140)), 

                0x2c070e98c8202fcc3a8cac5dfcab636a125c2c53f7ba36a2178c545131db2d9f 

            ) 

            // selector_comms_3.y 

            mstore( 

                add(mload(add(vk, 0x140)), 0x20), 

                0x1a3298b6427790a076429aaf4fd8554a022989d298e6636f295ec239ede33287 

            ) 

            // selector_comms_4.x 

            mstore( 

                mload(add(vk, 0x160)), 

                0x2c070e98c8202fcc3a8cac5dfcab636a125c2c53f7ba36a2178c545131db2d9f 

            ) 

            // selector_comms_4.y 

            mstore( 

                add(mload(add(vk, 0x160)), 0x20), 

                0x1a3298b6427790a076429aaf4fd8554a022989d298e6636f295ec239ede33287 

            ) 

            // selector_comms_5.x 

            mstore( 

                mload(add(vk, 0x180)), 

                0x2c070e98c8202fcc3a8cac5dfcab636a125c2c53f7ba36a2178c545131db2d9f 

            ) 

            // selector_comms_5.y 

            mstore( 

                add(mload(add(vk, 0x180)), 0x20), 

                0x1a3298b6427790a076429aaf4fd8554a022989d298e6636f295ec239ede33287 

            ) 

            // selector_comms_6.x 

            mstore( 

                mload(add(vk, 0x1a0)), 

                0x2c070e98c8202fcc3a8cac5dfcab636a125c2c53f7ba36a2178c545131db2d9f 

            ) 

            // selector_comms_6.y 

            mstore( 

                add(mload(add(vk, 0x1a0)), 0x20), 

                0x1a3298b6427790a076429aaf4fd8554a022989d298e6636f295ec239ede33287 

            ) 

            // selector_comms_7.x 

            mstore( 

                mload(add(vk, 0x1c0)), 

                0x2c070e98c8202fcc3a8cac5dfcab636a125c2c53f7ba36a2178c545131db2d9f 

            ) 

            // selector_comms_7.y 

            mstore( 

                add(mload(add(vk, 0x1c0)), 0x20), 

                0x1a3298b6427790a076429aaf4fd8554a022989d298e6636f295ec239ede33287 

            ) 

            // selector_comms_8.x 

            mstore( 

                mload(add(vk, 0x1e0)), 

                0x2c070e98c8202fcc3a8cac5dfcab636a125c2c53f7ba36a2178c545131db2d9f 

            ) 

            // selector_comms_8.y 

            mstore( 

                add(mload(add(vk, 0x1e0)), 0x20), 

                0x1a3298b6427790a076429aaf4fd8554a022989d298e6636f295ec239ede33287 

            ) 

            // selector_comms_9.x 

            mstore( 

                mload(add(vk, 0x200)), 

                0x2c070e98c8202fcc3a8cac5dfcab636a125c2c53f7ba36a2178c545131db2d9f 

            ) 

            // selector_comms_9.y 

            mstore( 

                add(mload(add(vk, 0x200)), 0x20), 

                0x1a3298b6427790a076429aaf4fd8554a022989d298e6636f295ec239ede33287 

            ) 

            // selector_comms_10.x 

            mstore( 

                mload(add(vk, 0x220)), 

                0x2c070e98c8202fcc3a8cac5dfcab636a125c2c53f7ba36a2178c545131db2d9f 

            ) 

            // selector_comms_10.y 

            mstore( 

                add(mload(add(vk, 0x220)), 0x20), 

                0x1a3298b6427790a076429aaf4fd8554a022989d298e6636f295ec239ede33287 

            ) 

            // selector_comms_11.x 

            mstore( 

                mload(add(vk, 0x240)), 

                0xe540ed913c934ce103797ca49597514b737221427b65de4193a24d22d35a6a0 

            ) 

            // selector_comms_11.y 

            mstore( 

                add(mload(add(vk, 0x240)), 0x20), 

                0x301d93c3824e94953fbb3cf62dd9c715fff5091191a9c0746ea2344439c5f757 

            ) 

            // selector_comms_12.x 

            mstore( 

                mload(add(vk, 0x260)), 

                0xeeb0057d511309b6d4cd782c845a8f2e4c0a38ed864537a11459a31c4100f17 

            ) 

            // selector_comms_12.y 

            mstore( 

                add(mload(add(vk, 0x260)), 0x20), 

                0xb267413389dd11e903f58a5e319f92d81177b27d959449c67daca851903afc6 

            ) 

            // selector_comms_13.x 

            mstore( 

                mload(add(vk, 0x280)), 

                0x2c070e98c8202fcc3a8cac5dfcab636a125c2c53f7ba36a2178c545131db2d9f 

            ) 

            // selector_comms_13.y 

            mstore( 

                add(mload(add(vk, 0x280)), 0x20), 

                0x1a3298b6427790a076429aaf4fd8554a022989d298e6636f295ec239ede33287 

            ) 

            // selector_comms_14.x 

            mstore( 

                mload(add(vk, 0x2a0)), 

                0x2c070e98c8202fcc3a8cac5dfcab636a125c2c53f7ba36a2178c545131db2d9f 

            ) 

            // selector_comms_14.y 

            mstore( 

                add(mload(add(vk, 0x2a0)), 0x20), 

                0x1a3298b6427790a076429aaf4fd8554a022989d298e6636f295ec239ede33287 

            ) 

            // selector_comms_15.x 

            mstore( 

                mload(add(vk, 0x2c0)), 

                0x2c070e98c8202fcc3a8cac5dfcab636a125c2c53f7ba36a2178c545131db2d9f 

            ) 

            // selector_comms_15.y 

            mstore( 

                add(mload(add(vk, 0x2c0)), 0x20), 

                0x1a3298b6427790a076429aaf4fd8554a022989d298e6636f295ec239ede33287 
  
            ) 

            // selector_comms_16.x 

            mstore( 

                mload(add(vk, 0x2e0)), 

                0x2c070e98c8202fcc3a8cac5dfcab636a125c2c53f7ba36a2178c545131db2d9f 

            ) 

            // selector_comms_16.y 

            mstore( 

                add(mload(add(vk, 0x2e0)), 0x20), 

                0x1a3298b6427790a076429aaf4fd8554a022989d298e6636f295ec239ede33287 

            ) 

            // selector_comms_17.x 

            mstore( 

                mload(add(vk, 0x300)), 

                0x2c070e98c8202fcc3a8cac5dfcab636a125c2c53f7ba36a2178c545131db2d9f 

            ) 

            // selector_comms_17.y 

            mstore( 

                add(mload(add(vk, 0x300)), 0x20), 

                0x1a3298b6427790a076429aaf4fd8554a022989d298e6636f295ec239ede33287 

            ) 

            // selector_comms_18.x 

            mstore( 

                mload(add(vk, 0x320)), 

                0x14e6b4b2e0ab16af07895f5323c577819f76afadd3b654d05864e3a8d2569529 

            ) 

            // selector_comms_18.y 

            mstore( 

                add(mload(add(vk, 0x320)), 0x20), 

                0x291747f35cb51fddcd8f87cbb0f1ad48e9274bea9d4f788fe791d3f545872f88 

            ) 

            // k1 

            mstore( 

                add(vk, 0x340), 

                0x1) 

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

                0x5cb0b9fc82eff921ccd085f1254a3a8fbe2c82e79abfdd99cb2f2c8353d5e8 

            ) 

            // range_table_comm.y 

            mstore( 

                add(mload(add(vk, 0x400)), 0x20), 

                0x3deac380246da8595ae1a8af4b8246a53f4a3b0ca66252b34b915205a311162 

            ) 

            // key_table_comm.x 

            mstore( 

                mload(add(vk, 0x420)), 

                0x26015dd551ef9eb178392db023f08b8af77aa5fabdcf53cb1201addebf9e1a67 

            ) 

            // key_table_comm.y 

            mstore( 

                add(mload(add(vk, 0x420)), 0x20), 

                0xeb1d734631243278ad0c244da67cbc72a8c3367f07188e22689b2d4c2f91f6a 

            ) 

            // table_dom_sep_comm.x 

            mstore( 

                mload(add(vk, 0x440)), 

                0x2960e04a2eee47a5373512d4181ecdd9f6b7c8c1875da9beae0a934ce071638d 

            ) 

            // table_dom_sep_comm.y 

            mstore( 

                add(mload(add(vk, 0x440)), 0x20), 

                0x7ab73985287b41f40028d108e97686f674ea59588c20027b791f93eb7fc8afd 

            ) 

            // q_dom_sep_comm.x 

            mstore( 

                mload(add(vk, 0x460)), 

                0x2c51031b1329ecb7043ca0b1a39beddc2a4c42af8dc91062fd3ade6ea9b62c48 

            ) 

            // q_dom_sep_comm.y 

            mstore( 

                add(mload(add(vk, 0x460)), 0x20), 

                0x141587296a82b2e51b417b9d7930a22d92b0c028e16ca94ce3c808419f03560 

            ) 

            // size_inv 

            mstore( 

                add(vk, 0x480), 

                0x304c1c4ba7c10759a3741d93a64097b0f99fce54557c93d8fb40049926080001 

            ) 

            // group_gen 

            mstore( 

                add(vk, 0x4a0), 

                0xf1ded1ef6e72f5bffc02c0edd9b0675e8302a41fc782d75893a7fa1470157ce 

            ) 

            // group_gen_inv 

            mstore( 

                add(vk, 0x4c0), 

                0x9d8f821aa9995b3546875d5e4fcfcab4c277a07f0bcc0c852f26c0faf6b3e4e 

            ) 

            // open_key_g.x 

            mstore( 

                mload(add(vk, 0x4e0)),  

                0x10ff4ab61e6109f64b103c45770a0c2ec238622df2dddbf4aee2683203a35b7f 

            ) 

            // open_key_g.y 

            mstore( 

                add(mload(add(vk, 0x4e0)), 0x20),  

                0x10ff352f0d2acf65fcd3a0517bb3dba40d462da489a9041ffd6e02a1f1ef6100
            ) 

            // open_key_h 

            let h_ptr := mload(0x40) 

            mstore(add(vk, 0x500), h_ptr) 

            mstore(h_ptr, 0x1eb826b8b6e3ebbb3a1607ebf19d2758c162d2c915188d49191d80a4f88e5306) // x0  

            mstore(add(h_ptr, 0x20), 0x29fd9755178e22781e448030a456ebfbdbb449335804c09e815f0b7a8437b94d) // x1  

            mstore(add(h_ptr, 0x40), 0x29eca2bd88436dab9a7bd738d120579d2dbcc9abc980d32aba2449d68b0a9457) // y0  

            mstore(add(h_ptr, 0x60), 0x182766815b58475e731a8afcff419c841a4ed2f571e3f9d4b1af94bc5e7d5777) // y1  

            mstore(0x40, add(h_ptr, 0x80)) 

            // open_key_beta_h 

            let beta_h_ptr := mload(0x40) 

            mstore(add(vk, 0x520), beta_h_ptr) 

            mstore(beta_h_ptr, 0x1dd13357222eab4fb810d5c89b5af426816cd0492532f7f181bb44e39cbc2be4) // x0 

            mstore(add(beta_h_ptr, 0x20), 0x1d9b573a9b30ead10dcf030d1ab3c9ec81dc3da2aac764597280370a6b29baab) // x1 

            mstore(add(beta_h_ptr, 0x40), 0x20ca4b8da283890ea4ab8ac17f07102e0e3bcd102998e3bb16349b6005b02de4) // y0 

            mstore(add(beta_h_ptr, 0x60), 0x2b9ee7fd0e19d5ec504255b3090e52ab453425e7b43c170022f6f862f7cc2291) // y1 

            mstore(0x40, add(beta_h_ptr, 0x80)) 
 
            }

            return vk; 

            
        }

    }