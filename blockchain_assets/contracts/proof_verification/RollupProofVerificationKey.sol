// SPDX-License-Identifier: Apache-2.0 

     

    pragma solidity ^0.8.20; 

    import "./Types.sol"; 

        
    library UltraPlonkVerificationKey { 


        function getVerificationKey() internal pure returns (Types.VerificationKey memory vk) { 

            assembly { 

            // domain_size
            mstore(add(vk, 0x00), 0x2000000) 

            // num_inputs 

            mstore(add(vk, 0x20), 0x1) 

            // sigma_comms_1.x 

            mstore( 

                mload(add(vk, 0x40)), 

                0x2742e6f576208af3607f2196c7aabc16add07c86ad56a69311e2742a97c056e1 

            ) 

            // sigma_comms_1.y 

            mstore( 

                add(mload(add(vk, 0x40)), 0x20), 

                0x9405a5f5b77e4277fd8767d8262ab8a12b47efc34c17cc886ff4a870c81d606 

            ) 

            // sigma_comms_2.x 

            mstore( 

                mload(add(vk, 0x60)), 

                0x17d2de352d8c9d71c8aa547f7c2375122ce60a2e492f37a8db44e6228937437 

            ) 

            // sigma_comms_2.y 

            mstore( 

                add(mload(add(vk, 0x60)), 0x20), 

                0x19ead9572d4c2748c783af1de93ed8fe02265f1a4a298880d882ace25f986c21 

            ) 

            // sigma_comms_3.x 

            mstore( 

                mload(add(vk, 0x80)), 

                0x27cc0dc1b693072a8f1876b07b6b6305a08b7ce149b52bf87c2ea7602bf0e1de 

            ) 

            // sigma_comms_3.y 

            mstore( 

                add(mload(add(vk, 0x80)), 0x20), 

                0x2850f49aed9e35c2fda8f844dbcd1879cddc4d1c80dcd44f0726979965b323d8 

            ) 

            // sigma_comms_4.x 

            mstore( 

                mload(add(vk, 0xa0)), 

                0x4454055092d5ab8ad17878d132095e5381b45c9f1648a12b98abf808f38057a 

            ) 

            // sigma_comms_4.y 
 
            mstore( 

                add(mload(add(vk, 0xa0)), 0x20), 

                0xd3b9d567375bbbec4bedd6d74271ff85b09c90c4ee7364b12ca0519fd28f4db 

            ) 

            // sigma_comms_5.x 

            mstore( 

                mload(add(vk, 0xc0)), 

                0x29b44aa16c8ebc7ed7f8fb900887268ba4f89e47ecef5ee07156cf472307b91 

            ) 

            // sigma_comms_5.y 

            mstore( 

                add(mload(add(vk, 0xc0)), 0x20), 

                0x2fb15d2a2e0e699e83ef7b806eb749c0bf3b598c02532b2d61a9368b89527071 

            ) 

            // sigma_comms_6.x 

                mstore( 

                mload(add(vk, 0xe0)), 

                0x86eb70aaaf316430e1f16e5de9e8ee147bb2bb88e8219e8858c760a933cca21 

            ) 

            // sigma_comms_6.y 

            mstore( 

                add(mload(add(vk, 0xe0)), 0x20), 

                0x2399dcb2becedbf0fd5f5abceb9960dcdd4529d0f070c59b12f9f2232682adf1 

            ) 

            // selector_comms_1.x 

            mstore( 

                mload(add(vk, 0x100)), 
  
                0x1b860e03abacebac4d877227820354e9c7c138c59488521e693980b966fe7542 

            ) 

            // selector_comms_1.y 

            mstore( 

                add(mload(add(vk, 0x100)), 0x20), 

                0x2c631a0edd331c694bbaf84b39534e7e1fb369febc733163ea941135c7cc8cd0 

            ) 

            // selector_comms_2.x 

            mstore( 

                mload(add(vk, 0x120)), 

                0x15b9a1ee51409152bc8c48a25b1542041109516ee2622b4da089c1bd14e55a77 

            ) 

            // selector_comms_2.y 

            mstore( 

                add(mload(add(vk, 0x120)), 0x20), 

                0x240248a0fd937e30a9f77ae5aa2adc93206cdf985d6d5329e2ce3cfe9b8904ee 

            ) 

            // selector_comms_3.x 

            mstore( 

                mload(add(vk, 0x140)), 

                0x1dddb6c84b7a63c10461e4072efb473774ae86563e9c49b75df63f69747add5b 

            ) 

            // selector_comms_3.y 

            mstore( 

                add(mload(add(vk, 0x140)), 0x20), 

                0xd7d06452081ffec2336dd86c26f39aeb834b31aa019eeb3f86d5a7a91245e8e 

            ) 

            // selector_comms_4.x 

            mstore( 

                mload(add(vk, 0x160)), 

                0x1ff0daf13a7fce603594e456a847560de96e9de43f7ca64a9b478b08afd06fcb 

            ) 

            // selector_comms_4.y 

            mstore( 

                add(mload(add(vk, 0x160)), 0x20), 

                0x123f270f08d2eff8cfe21dad1f54d669785d83dafb7536e185eb1a12a02b4fbb 

            ) 

            // selector_comms_5.x 

            mstore( 

                mload(add(vk, 0x180)), 

                0x16098cd0b7496834f7cc925d9c3fe65fdb3049dcff46b7f94d2e844a4614165 

            ) 

            // selector_comms_5.y 

            mstore( 

                add(mload(add(vk, 0x180)), 0x20), 

                0x1798c6300dd1472165c806164d7c71cab8823b5d16514cc22128e426a4979172 

            ) 

            // selector_comms_6.x 

            mstore( 

                mload(add(vk, 0x1a0)), 

                0x2d71b308b9280fa89c3584499317432ac2769a522cc0fb5d5189462fd41b00de 

            ) 

            // selector_comms_6.y 

            mstore( 

                add(mload(add(vk, 0x1a0)), 0x20), 

                0xd6592d03866c942d633fe5b2e07f3b22bcc34478270988e7508009b98904f35 

            ) 

            // selector_comms_7.x 

            mstore( 

                mload(add(vk, 0x1c0)), 

                0xb8404aadcb84b62ea019cd08acb231dfcfd45058c6a7eef7c94f235cfb1770b 

            ) 

            // selector_comms_7.y 

            mstore( 

                add(mload(add(vk, 0x1c0)), 0x20), 

                0x18bf08d1a42e516663afbdca0a8890f854fc80ab909e7d4f3e60400a431d2c50 

            ) 

            // selector_comms_8.x 

            mstore( 

                mload(add(vk, 0x1e0)), 

                0x76e3b95c3f56f7d4b46ec54263fbf4db20b4c9d4711c0767ed3ed24fd1071b7 

            ) 

            // selector_comms_8.y 

            mstore( 

                add(mload(add(vk, 0x1e0)), 0x20), 

                0x2321eac3cde87c75a927ae8fb5ae8f12b48ffc823bb34cd939f3ef7f969b4db2 

            ) 

            // selector_comms_9.x 

            mstore( 

                mload(add(vk, 0x200)), 

                0xee2e6d63e1cb131dd98ea636b3ab24cb1bc765d314da2b404e67c47e5107bbe 

            ) 

            // selector_comms_9.y 

            mstore( 

                add(mload(add(vk, 0x200)), 0x20), 

                0x122b83ed765b4d8471797a5fc4fbd63dcd80c9bfd302f3b3ea7210a1e3589713 

            ) 

            // selector_comms_10.x 

            mstore( 

                mload(add(vk, 0x220)), 

                0x120b2ee0eb7ba5180f5dc4f695d6b9b7ea2f744e25a38be7bb3773fd2c45ddbc 

            ) 

            // selector_comms_10.y 

            mstore( 

                add(mload(add(vk, 0x220)), 0x20), 

                0x4ba8dd81b95213c609aac54e8aa9e2ee1da6083f5c5a5765c3c6d7c889e8a98 

            ) 

            // selector_comms_11.x 

            mstore( 

                mload(add(vk, 0x240)), 

                0x2274d92f3cc81145916f4e523c177b83345ff082c943fcd145aa3a60093149d5 

            ) 

            // selector_comms_11.y 

            mstore( 

                add(mload(add(vk, 0x240)), 0x20), 

                0x22628eb726e3e080c6a12943b6d4ba5ab648ba07c2ccae03ca579517c533d50f 

            ) 

            // selector_comms_12.x 

            mstore( 

                mload(add(vk, 0x260)), 

                0x2c6f4b531f76dc446738c6936ae55eca0f9cbe6a8c4b446c25745100edac8b4e 

            ) 

            // selector_comms_12.y 

            mstore( 

                add(mload(add(vk, 0x260)), 0x20), 

                0x1eac0a938b91c1508a89fce32233b4a029384c9f3a399a97ba1f20fc671f808b 

            ) 

            // selector_comms_13.x 

            mstore( 

                mload(add(vk, 0x280)), 

                0x2f298bb7487fe85d14e918fa1d66a13789d1854480d760a142015165fba68cc4 

            ) 

            // selector_comms_13.y 

            mstore( 

                add(mload(add(vk, 0x280)), 0x20), 

                0x24a4c6bbf7eb9b45c539ed1304574dbfaa13524e41fab4c6cbfb5e47f0b43e5f 

            ) 

            // selector_comms_14.x 

            mstore( 

                mload(add(vk, 0x2a0)), 

                0x2991df63aedcf5650b20ea3f19f3543918d4e9d39f8c984cdbeec667c006f62f 

            ) 

            // selector_comms_14.y 

            mstore( 

                add(mload(add(vk, 0x2a0)), 0x20), 

                0xff10a9dfb886da231f3cdd2fab6038cc95fd4598c3c0a8db1c37330fdf8b70a 

            ) 

            // selector_comms_15.x 

            mstore( 

                mload(add(vk, 0x2c0)), 

                0x286f77da85e1f8b9fe098a8140896e745c0e01520983e53b2539134fa48c9544 

            ) 

            // selector_comms_15.y 

            mstore( 

                add(mload(add(vk, 0x2c0)), 0x20), 

                0x20e6c84de7696a532740b7e2b8660e660761cffc259d0ddd290180d7d3da79d9 
  
            ) 

            // selector_comms_16.x 

            mstore( 

                mload(add(vk, 0x2e0)), 

                0x202baaec2865143f78ab397ec5dd51dbb111e16448f8bd44caa531c24cf522e7 

            ) 

            // selector_comms_16.y 

            mstore( 

                add(mload(add(vk, 0x2e0)), 0x20), 

                0xc840e137994486177bd08af7ad3e7705c144e8869f870bd56a0770e5c359012 

            ) 

            // selector_comms_17.x 

            mstore( 

                mload(add(vk, 0x300)), 

                0x21d34744ca66aa084f990b6754fe77cc380d70107333242583d4923ca61cc121 

            ) 

            // selector_comms_17.y 

            mstore( 

                add(mload(add(vk, 0x300)), 0x20), 

                0x21d77ecb6e913f1f817aa931dc04e887055983d6de84e591e04688b949167b06 

            ) 

            // selector_comms_18.x 

            mstore( 

                mload(add(vk, 0x320)), 

                0x25c8062b27b8c6b1a5844c5f6595ad3967a306235fa1eaa89b7ecb06137c8afc 

            ) 

            // selector_comms_18.y 

            mstore( 

                add(mload(add(vk, 0x320)), 0x20), 

                0x2b2978fb648cb096a3f50d211ae7ad6eca69c501a8b258260865bd96a913934a 

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

                0x2db1df07e52bfc8c98d179cbf77bfbc27686bff2bbf4c5d891c3b207238f4536 

            ) 

            // key_table_comm.y 

            mstore( 

                add(mload(add(vk, 0x420)), 0x20), 

                0x2141a52423d4b471e95e699e501c46a117511b877ea6f55927fa23251960898 

            ) 

            // table_dom_sep_comm.x 

            mstore( 

                mload(add(vk, 0x440)), 

                0x1c6df5aefe9c29ffe46c5616432fd5183ae66e19beffb8c8bda713c4c6787921 

            ) 

            // table_dom_sep_comm.y 

            mstore( 

                add(mload(add(vk, 0x440)), 0x20), 

                0xd0201154a22f7758ca6412800a93c4d4bc4ab7f888528ac972bdb8a41164d4b 

            ) 

            // q_dom_sep_comm.x 

            mstore( 

                mload(add(vk, 0x460)), 

                0x13cce8db24c2f4be3d3e32a2683a7125b4a512fa4d991ee6d04e29800ffc328 

            ) 

            // q_dom_sep_comm.y 

            mstore( 

                add(mload(add(vk, 0x460)), 0x20), 

                0x28fe3e1edc2878b070a0997ce5f2ad8ede2c4a8c51a272e23dcfec7df32e678d 

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

            }

            return vk; 

        }

    }