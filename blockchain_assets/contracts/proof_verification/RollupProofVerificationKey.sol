// SPDX-License-Identifier: Apache-2.0 

     

    pragma solidity ^0.8.20; 

    import "./Types.sol"; 

        
    library UltraPlonkVerificationKey { 


        function getVerificationKeyHash() internal pure returns (bytes32) {  

            return 0x1c1e345bf747a1d9824b09de382109a42513b11de016709ca5011221f9abe56f; 

        } 


        function getVerificationKey() internal pure returns (Types.VerificationKey memory vk) { 

            assembly { 

            // domain_size
            mstore(add(vk, 0x00), 0x2000000) 

            // num_inputs 

            mstore(add(vk, 0x20), 0x1) 

            // sigma_comms_1.x 

            mstore( 

                mload(add(vk, 0x40)), 

                0x224aa1e86aa1f1befc63eb7dca0b5933e01f712d95f3a573d92a66efd4b31f60 

            ) 

            // sigma_comms_1.y 

            mstore( 

                add(mload(add(vk, 0x40)), 0x20), 

                0x25f81419c20cd2459e03611d1a3c9339086697876a38b56351df5cb19a7f11ee 

            ) 

            // sigma_comms_2.x 

            mstore( 

                mload(add(vk, 0x60)), 

                0xee3ddea5323d9368ed8ccbf50e47f1307ab2a8df47192678f1a9d6329d38ca7 

            ) 

            // sigma_comms_2.y 

            mstore( 

                add(mload(add(vk, 0x60)), 0x20), 

                0x1a4d3eb6cde54c6c52f103b69e3daa37eb46eca3f2a0960df9ae6f6f2c4da8c9 

            ) 

            // sigma_comms_3.x 

            mstore( 

                mload(add(vk, 0x80)), 

                0xed11832565538b32a2be9e70ce9eed6b09fd8dcd82e9af87320d2ab5ea1fe9f 

            ) 

            // sigma_comms_3.y 

            mstore( 

                add(mload(add(vk, 0x80)), 0x20), 

                0x17492bbd308b64f54f6baa250c5b523f6c651f4c22210fecbe9799dcf3088d0f 

            ) 

            // sigma_comms_4.x 

            mstore( 

                mload(add(vk, 0xa0)), 

                0x12f72ea1e0b638e16534d521629b401b42653a505ce1b65bbf682c86c0e1cf3 

            ) 

            // sigma_comms_4.y 
 
            mstore( 

                add(mload(add(vk, 0xa0)), 0x20), 

                0x1af843eb6d58f88397a60932bf485a0215f59f320bbc53d8b6f2334f4249f0ff 

            ) 

            // sigma_comms_5.x 

            mstore( 

                mload(add(vk, 0xc0)), 

                0x13842fddc8a50704d9a464ed76b0ad1e2badd8e71a362cea8642bd43f195c9d2 

            ) 

            // sigma_comms_5.y 

            mstore( 

                add(mload(add(vk, 0xc0)), 0x20), 

                0x1f7d04850eb0775cdeacc96b4d037e22379a41bbf8ce2eb44025cd3acc8f7b45 

            ) 

            // sigma_comms_6.x 

                mstore( 

                mload(add(vk, 0xe0)), 

                0xef67b9aed33ca744b467dfe54b10588e85c69dd4e1a0ccc482dac77ca6434f4 

            ) 

            // sigma_comms_6.y 

            mstore( 

                add(mload(add(vk, 0xe0)), 0x20), 

                0x135309095418997c1acc374386206df445af8046ec0213f1c36f467629860fac 

            ) 

            // selector_comms_1.x 

            mstore( 

                mload(add(vk, 0x100)), 
  
                0x21cf3183e15d395f472df27276929872fce24077b9c2ef3a2f5edc74d309fa6f 

            ) 

            // selector_comms_1.y 

            mstore( 

                add(mload(add(vk, 0x100)), 0x20), 

                0x23db370ed5fa8b8c069c4a138ee073ea75bf066c8197093841f337e3b5908d23 

            ) 

            // selector_comms_2.x 

            mstore( 

                mload(add(vk, 0x120)), 

                0x21356c841d6d4fa77c776d0592feb7be7a8a5461e64a2f1c0fbf9baa261f28cb 

            ) 

            // selector_comms_2.y 

            mstore( 

                add(mload(add(vk, 0x120)), 0x20), 

                0x22476ebc7c5f1dbe4ccdac48262e33ee3201e62876b8e0dc6146880a98cd7b19 

            ) 

            // selector_comms_3.x 

            mstore( 

                mload(add(vk, 0x140)), 

                0x213a4b0a0e2489ad2f44b7208fb4a72387dd92c68b03cf00fe6b137273fc6bf9 

            ) 

            // selector_comms_3.y 

            mstore( 

                add(mload(add(vk, 0x140)), 0x20), 

                0x1fbfc0e404f50ddc2f80a8224fa040e6ad87b653bd69692d8da8da69cc66e45e 

            ) 

            // selector_comms_4.x 

            mstore( 

                mload(add(vk, 0x160)), 

                0x26500f9857b909578887562a24e5eeb00aa6225c3abc211d6a9948c7ad6d900b 

            ) 

            // selector_comms_4.y 

            mstore( 

                add(mload(add(vk, 0x160)), 0x20), 

                0x3057a638850f561e457865b2fb18e4883b4137ddaf3866eb14c3f8645244d008 

            ) 

            // selector_comms_5.x 

            mstore( 

                mload(add(vk, 0x180)), 

                0x90c2b10408f52a49208558c48f6c9e9f1d9027a6e0714d92cd8edfeedf428b 

            ) 

            // selector_comms_5.y 

            mstore( 

                add(mload(add(vk, 0x180)), 0x20), 

                0x287202d8457a299381e4ef7fa8870f4187a4a99fe414bda0eb9eb213459b8032 

            ) 

            // selector_comms_6.x 

            mstore( 

                mload(add(vk, 0x1a0)), 

                0x6d4d75f7f9ac52cd3ff6a00dc5432fac71b81c95c10b0b6a0cf6c6ad842f82a 

            ) 

            // selector_comms_6.y 

            mstore( 

                add(mload(add(vk, 0x1a0)), 0x20), 

                0x21b1a022d8fa08c5877fc29aae4dd10a0bee624b82ef58d9ca2bd885ecaa773c 

            ) 

            // selector_comms_7.x 

            mstore( 

                mload(add(vk, 0x1c0)), 

                0x14fc4c0d862ac23877c9326b88ab6efbdef47ec62925575f07f83bcf012e2e49 

            ) 

            // selector_comms_7.y 

            mstore( 

                add(mload(add(vk, 0x1c0)), 0x20), 

                0x2a3162b250bd3f59ce9c425445447a477ab9017a1a67b056592afdd474d3693f 

            ) 

            // selector_comms_8.x 

            mstore( 

                mload(add(vk, 0x1e0)), 

                0x13e081900816842f9e7b6ec2340c6b434d05d061fc72a7613239ae3674f654ee 

            ) 

            // selector_comms_8.y 

            mstore( 

                add(mload(add(vk, 0x1e0)), 0x20), 

                0x2077c4369dbb9ad5e11f8dc5c67cf91d055954043a44ff55c887b7b332e1601a 

            ) 

            // selector_comms_9.x 

            mstore( 

                mload(add(vk, 0x200)), 

                0x8f35ec1a09b83fe7a89795eef68a71a73ea69164d17ea74e424c18a27cfd154 

            ) 

            // selector_comms_9.y 

            mstore( 

                add(mload(add(vk, 0x200)), 0x20), 

                0x2da03fa8184b74751c2cddd462510f77807619b5e0a5cb2269e0c2d34c550042 

            ) 

            // selector_comms_10.x 

            mstore( 

                mload(add(vk, 0x220)), 

                0x1c980fee742192d7461f564307a8a0cdd469203d4c239c9daf7734d3e25a57af 

            ) 

            // selector_comms_10.y 

            mstore( 

                add(mload(add(vk, 0x220)), 0x20), 

                0x2d6c75cfc5991b321572cf1a19e0bbadbcadc790101ba3d45bd7110b8b19bdf4 

            ) 

            // selector_comms_11.x 

            mstore( 

                mload(add(vk, 0x240)), 

                0x20e8848cb6a4ce2a70e912a954701aff7cc826fb815cf5aa67dce6cb04289d97 

            ) 

            // selector_comms_11.y 

            mstore( 

                add(mload(add(vk, 0x240)), 0x20), 

                0x31ada280955d6dd78af6d8d63d6dd4d9397bcc5d8ed4af4beb3c466344c9917 

            ) 

            // selector_comms_12.x 

            mstore( 

                mload(add(vk, 0x260)), 

                0x15dcbcddb5b26fb6f446047867f51a44ad686b0f68d4d10e94cad7380dcd07a3 

            ) 

            // selector_comms_12.y 

            mstore( 

                add(mload(add(vk, 0x260)), 0x20), 

                0x1235790b118ca52b2cab3a983d5c7a1ec67827f6704fb747dfb81cac918c45b4 

            ) 

            // selector_comms_13.x 

            mstore( 

                mload(add(vk, 0x280)), 

                0x2ae965fefef7a4b2df802a01cfd6d3a2cef4b4309dcd6716a65c829ec304bf52 

            ) 

            // selector_comms_13.y 

            mstore( 

                add(mload(add(vk, 0x280)), 0x20), 

                0x281f365e16247924b2bd5d2c6467b31ccc3692693a98cfc67f741c089fc5fb3 

            ) 

            // selector_comms_14.x 

            mstore( 

                mload(add(vk, 0x2a0)), 

                0x2fbe8998140323372ac4b68dc400a176f197b6abfdfac3489e410ea772e0a5fa 

            ) 

            // selector_comms_14.y 

            mstore( 

                add(mload(add(vk, 0x2a0)), 0x20), 

                0x5c1c6088a434a9925c9e290bc1fb22f05580a3b28e3fbe5a4cdf1a222525960 

            ) 

            // selector_comms_15.x 

            mstore( 

                mload(add(vk, 0x2c0)), 

                0x2937f3d6c6d90b1641899b0af223b7b4254f79dd37ea9c404a7d2e163b6861b7 

            ) 

            // selector_comms_15.y 

            mstore( 

                add(mload(add(vk, 0x2c0)), 0x20), 

                0xcd18a059ce70ff69a92c6f8f3b41e688c685a48d960080a0ff7a08c3b301ceb 
  
            ) 

            // selector_comms_16.x 

            mstore( 

                mload(add(vk, 0x2e0)), 

                0x23585175e379d41275caee601c012a4ca0183693810bde68d02a85d563a398b3 

            ) 

            // selector_comms_16.y 

            mstore( 

                add(mload(add(vk, 0x2e0)), 0x20), 

                0x1a6ea9cb335d3f4bb22fdec28497ba8048bf4d0b73351557b3412ee032cc080c 

            ) 

            // selector_comms_17.x 

            mstore( 

                mload(add(vk, 0x300)), 

                0x551412252183d89a4bc0de5fabb821a2ed129c9b49f31d148b5f1150f26e1cf 

            ) 

            // selector_comms_17.y 

            mstore( 

                add(mload(add(vk, 0x300)), 0x20), 

                0x162f5aa78e941410582b437daa750caf997c0f30b5875a4c0aec153fa691aa16 

            ) 

            // selector_comms_18.x 

            mstore( 

                mload(add(vk, 0x320)), 

                0x14fcee171c8bf03b333a70d359b2a668ca28cca1707004d67118e7174a8e0d0d 

            ) 

            // selector_comms_18.y 

            mstore( 

                add(mload(add(vk, 0x320)), 0x20), 

                0x1719b3e69e1d1d21c0ae93122a1d152fa52cb1e900642f91865219078fedf11b 

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

                0x1845622f9012f44d4d9b492a254a360ab98914fda5d90031b0a78009cbae56b8 

            ) 

            // range_table_comm.y 

            mstore( 

                add(mload(add(vk, 0x400)), 0x20), 

                0x2da38747c37c85b6815a9ac706a061274eaae8f7cfb4e3c83ad4112ba8dcc18f 

            ) 

            // key_table_comm.x 

            mstore( 

                mload(add(vk, 0x420)), 

                0x2cd3d4b52c4e81a0230fdc2afb118f488f76df3cbb474e688c21cdee5f1274ff 

            ) 

            // key_table_comm.y 

            mstore( 

                add(mload(add(vk, 0x420)), 0x20), 

                0x21889c2f2692386e405ffab0d2241794f4b4431e3265d97f42ee7e77894196db 

            ) 

            // table_dom_sep_comm.x 

            mstore( 

                mload(add(vk, 0x440)), 

                0x3735500ca623cc3fddb6a85a133599035e252fe10286c214f5ea8d1aff8dd34 

            ) 

            // table_dom_sep_comm.y 

            mstore( 

                add(mload(add(vk, 0x440)), 0x20), 

                0x1f6fff4e2027f91a1cab2cdf2ba43331f8a2bae93bba9659fbc7a6f25d87740c 

            ) 

            // q_dom_sep_comm.x 

            mstore( 

                mload(add(vk, 0x460)), 

                0x1926d85f74e52438d6ddb98c5e8c21aebdf9d351b907df73e5c3198708371281 

            ) 

            // q_dom_sep_comm.y 

            mstore( 

                add(mload(add(vk, 0x460)), 0x20), 

                0x2dfa10125652bdee92111199fcb7a4469cc346e1a2e1c970b266562e07ce377b 

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

            mstore( 

                mload(add(vk, 0x4e0)),  

                0x1 

            ) 

            // open_key_g.y 

            mstore( 

                add(mload(add(vk, 0x4e0)), 0x20),  

                0x2
            ) 

            // open_key_h 

            let h_ptr := mload(0x40) 

            mstore(add(vk, 0x500), h_ptr) 

            mstore(h_ptr, 0x198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2) // x0  

            mstore(add(h_ptr, 0x20), 0x1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed) // x1  

            mstore(add(h_ptr, 0x40), 0x90689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b) // y0  

            mstore(add(h_ptr, 0x60), 0x12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa) // y1  

            mstore(0x40, add(h_ptr, 0x80)) 

            // open_key_beta_h 

            let beta_h_ptr := mload(0x40) 

            mstore(add(vk, 0x520), beta_h_ptr) 

            mstore(beta_h_ptr, 0x17cc93077f56f654da727c1def86010339c2b4131094547285adb083e48c197b) // x0 

            mstore(add(beta_h_ptr, 0x20), 0x285b1f14edd7e6632340a37dfae9005ff762edcfecfe1c732a7474c0708bef80) // x1 

            mstore(add(beta_h_ptr, 0x40), 0x219edfceee1723de674f5b2f6fdb69d9e32dd53b15844956a630d3c7cdaa6ed9) // y0 

            mstore(add(beta_h_ptr, 0x60), 0x2bad9a374aec49d329ec66e8f530f68509313450580c4c17c6db5ddb9bde7fd0) // y1 

            mstore(0x40, add(beta_h_ptr, 0x80)) 
 
            }

            return vk; 

            
        }

    }