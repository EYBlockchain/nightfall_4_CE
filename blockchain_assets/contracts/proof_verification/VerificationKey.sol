// SPDX-License-Identifier: Apache-2.0 

        pragma solidity >=0.8.4; 

        
        library UltraVK { 


            function getVerificationKeyHash() internal pure returns (bytes32) { 

        return 0x06a494c6bd028e3cf4e706791f817744d3788aac10378a5dd61a95751963ad65;
    } 


           function loadVerificationKey( 

             uint256 _vk, 

               uint256 _second_loc 

           ) internal pure { 

              assembly { 

                 mstore( 
                      add(_vk, 0x00), 
                      0x2000000 
                  ) // DOMAIN_SIZE 

                   mstore( 

                       add(_vk, 0x20), 

                  0x1 

                    ) // NUM_PUBLIC_INPUTS 

                    mstore(
                        add(_vk, 0x40),
                        0x202da1228f18584fb123fef1c0faca36e488d5364840737ab2e4799c04d7d66c
                    ) // SIGMA1_X 

                    mstore(
                        add(_vk, 0x60),
                        0x22e5f8ff55b2c76ef6e240369d898d50fb0b658a055543ff47378a95b6bee174
                    ) // SIGMA1_Y 

                    mstore(
                        add(_vk, 0x80),
                        0x7d868b8f7672771a97d0074175e8809730fda43ae7173e703f81e384220d195
                    ) // SIGMA2_X 

                    mstore(
                        add(_vk, 0xa0),
                        0x23eb71059046a6e4792902296b938b99927497f20aa6d354148cb5ae4f405bee
                    ) // SIGMA2_Y 

                    mstore(
                        add(_vk, 0xc0),
                        0x1fcc73a3eced57f15c26702a4a0b3418c1d69b9b176ab17b21fee812fb0ac770
                    ) // SIGMA3_X 

                    mstore(
                        add(_vk, 0xe0),
                        0xf9ec6f530adb84f0c46f849bea1cc9610e84a51f60daf598af52ed859e74328
                    ) // SIGMA3_Y 

                    mstore(
                        add(_vk, 0x100),
                        0x13e92ab43f27b6eae473be5ca5db617888abf361fe0fc5909b779e8201d76df3
                    ) // SIGMA4_X 

                    mstore(
                        add(_vk, 0x120),
                        0x2d2c01a4f26a85a323ebddc4a34f4f5e319ddc019014d6ac27d8eb8a2d5c03f
                    ) // SIGMA4_Y 

                    mstore(
                        add(_vk, 0x140),
                        0x2ff9bf2229d0b44d493545f50037e52030805ac5b9f3a7bf6700346a52b09043
                    ) // SIGMA5_X 

                    mstore(
                        add(_vk, 0x160),
                        0x1341c3e2fe21ed5935d51b6cdbcb819ebc568777556e97bc146e10fbc32fd44
                    ) // SIGMA5_Y 

                    mstore(
                        add(_vk, 0x180),
                        0x8d13bd0fc03bf64e49ec66269c84b5ba0f230383b74d324c3e80ae5144cc66d
                    ) // SIGMA6_X 

                    mstore(
                        add(_vk, 0x1a0),
                        0x1ecf7de14b7e3122a819d6b62264d7901432a961d755f6c97f2a6e2d3bd4b744
                    ) // SIGMA6_Y 

                    mstore(
                        add(_vk, 0x1c0),
                        0x2e45aac8265517ee62d83d83e1c0ca06d1929f58e80739a32f1649ebf8a3498
                    ) // QLC1_X 

                    mstore(
                        add(_vk, 0x1e0),
                        0x115da4522648b3fd3b12f5dd5c591dc57805ebaf1397be7e871b136900636a8c
                    ) // QLC1_Y 

                    mstore(
                        add(_vk, 0x200),
                        0x1d279c36d1c642a168e4cfe3c59292df2063e96d87d4e33714a3bd69fc05aec2
                    ) // QLC2_X 

                    mstore(
                        add(_vk, 0x220),
                        0xfb918e5e894ba3cd07d8ec4d8e02d6eb538e505fae318ce75e0222ed22761e1
                    ) // QLC2_Y 

                    mstore(
                        add(_vk, 0x240),
                        0x2018ae76f52cc3a0346395c4bee5ea99b7b3128ffffb4533787e33c915da81c9
                    ) // QLC3_X 

                    mstore(
                        add(_vk, 0x260),
                        0x4b6781be2aae7f8262442937f6c366d0525173056f81596d2b82ab480ea729d
                    ) // QLC3_Y 

                    mstore(
                        add(_vk, 0x280),
                        0x134fa1a3cee0f245282940801467fcd51fc6656908235d5e5722b9a11baa2f2d
                    ) // QLC4_X 

                    mstore(
                        add(_vk, 0x2a0),
                        0xa643c53e0d041e0c2b77aae67dd8cb479d37f5eca23a86bbd7ba24d0dac0abf
                    ) // QLC4_Y 

                    mstore(
                        add(_vk, 0x2c0),
                        0xdc2ba343c8d322f9a300bd418c5e6de960a6d7af41139336b64c6f5b3b0dfa7
                    ) // QMUL1_X 

                    mstore(
                        add(_vk, 0x2e0),
                        0x20acf03c1976f1b99684491c5c27d48fc4aaff9f129c27b29920370d81e2aed0
                    ) // QMUL1_Y 

                    mstore(
                        add(_vk, 0x300),
                        0x11bf7736ad78b28a61cd79ddaac898e4db685feef3244b0c9778e25955c861eb
                    ) // QMUL2_X 

                    mstore(
                        add(_vk, 0x320),
                        0x1101f975a6b3b1bed1afa56affd6a9b7fab57f1bdaa69a2e8a364f2116c51ce2
                    ) // QMUL2_Y 

                    mstore(
                        add(_vk, 0x340),
                        0x1cf1dd5f115d114068412ae3c159a1b09391643dd40eb65ab3c08284c2803f20
                    ) // QHASH1_X 

                    mstore(
                        add(_vk, 0x360),
                        0xbd54a1066954aa8d03eb00b99ebdfc9658e614d598963060c1875fa004c0a21
                    ) // QHASH1_Y 

                    mstore(
                        add(_vk, 0x380),
                        0x30375ec4b308cf93ac6c141f11917f00d75dae35a4aeb8d04b10c7eeb0201375
                    ) // QHASH2_X 

                    mstore(
                        add(_vk, 0x3a0),
                        0x152b7dac09dbcc273553df558738e85997dc3ffa5e87c53bbe8548b0d865d4c1
                    ) // QHASH2_Y 

                    mstore(
                        add(_vk, 0x3c0),
                        0xba8e62f390a03a33dee0e83fac5346dae8ac81395a59df0c448c63dbb223327
                    ) // QHASH3_X 

                    mstore(
                        add(_vk, 0x3e0),
                        0x8c0bad8c18ee6c815bdd7c67f77cae321efc8c017b7a17657965dd59c3ae737
                    ) // QHASH3_Y 

                    mstore(
                        add(_vk, 0x400),
                        0x2bf45ad37831f2e997b3ca9f743ff95c303adc27d0d4f41a3efb969dddac34c
                    ) // QHASH4_X 

                    mstore(
                        add(_vk, 0x420),
                        0xcddc97db485246b881fba2c6cd4350d4a58b5a56697984defb797cb8ee858c9
                    ) // QHASH4_Y 

                    mstore(
                        add(_vk, 0x440),
                        0xf992f738c9502b539a92d54b291a66d3f37c9fa2b91f75fb63020a63b5ad17b
                    ) // QOUT_X 

                    mstore(
                        add(_vk, 0x460),
                        0x1aa87df40ca43027f853e73bedb4ad8be5e8168faa008a2b9204f47d26683323
                    ) // QOUT_Y 

                    mstore(
                        add(_vk, 0x480),
                        0x23b6f530fba19f96f7ec3090841fad17a3e37181dda209a17cf3c73e589bab77
                    ) // QCONST_X 

                    mstore(
                        add(_vk, 0x4a0),
                        0x24f3dd6bc7f30848475927b310e1f2b21d32dfda53b38e50093f34ace49819db
                    ) // QCONST_Y 

                    mstore(
                        add(_vk, 0x4c0),
                        0xab14774268577a616ac7f571a404b33d7519ed3215b9572a242e91893b3a24b
                    ) // QECC_X 

                    mstore(
                        add(_vk, 0x4e0),
                        0x9b2efdf2e2946bfd457e29bc732c8c5b8f6f9f2a8f133297e716dfcee84d8f1
                    ) // QECC_Y 

                    mstore(
                        add(_vk, 0x500),
                        0x24a33c065da2f7f36b6ac5bba28d43ce36ac05e128ca74c7bf1b2ad9b051dd2
                    ) // QSWX1_X 

                    mstore(
                        add(_vk, 0x520),
                        0x539f1286120bb69d33231f2bf469e85e43a23ce69c1862212fa64897f485bfb
                    ) // QSWX1_Y 

                    mstore(
                        add(_vk, 0x540),
                        0x2927bdc2d1530da877b6596c251c33455377b57d33b7e1e4cce67a65f388af81
                    ) // QSWX2_X 

                    mstore(
                        add(_vk, 0x560),
                        0x1448fd858ed61248021cffe32202ef2c7efc9354d815eed06fab89a1199657ea
                    ) // QSWX2_Y 

                    mstore(
                        add(_vk, 0x580),
                        0x115b8d691cc5c05620a8614c35ec2cce4a1be9fd9f887246a1156071f2c66a1e
                    ) // QSWY1_X 

                    mstore(
                        add(_vk, 0x5a0),
                        0x25ccb8f8e2560c2c91e7acdba3140164e511b715db96472ce2c481ffacb0119b
                    ) // QSWY1_Y 

                    mstore(
                        add(_vk, 0x5c0),
                        0x22a12a3d475fa101f6235a32fbf8072bcad03ec7ebe552bf06b8e945172a38ce
                    ) // QSWY2_X 

                    mstore(
                        add(_vk, 0x5e0),
                        0x13924e1529c84da4136591ed8ccf75131c745a5ba72b52e40ec34fa652579562
                    ) // QSWY2_Y 

                    mstore(
                        add(_vk, 0x600),
                        0x25c8062b27b8c6b1a5844c5f6595ad3967a306235fa1eaa89b7ecb06137c8afc 
                    ) // QLOOKUP_X 

                    mstore(
                        add(_vk, 0x620),
                        0x2b2978fb648cb096a3f50d211ae7ad6eca69c501a8b258260865bd96a913934a
                    ) // QLOOKUP_Y 

                    mstore(add(_vk, 0x640), 0x1) // K1 

                    mstore(add(_vk, 0x660), 0x2f8dd1f1a7583c42c4e12a44e110404c73ca6c94813f85835da4fb7bb1301d4a) // K2 

                    mstore(
                        add(_vk, 0x680),
                        0x1ee678a0470a75a6eaa8fe837060498ba828a3703b311d0f77f010424afeb025
                    ) // K3 

                    mstore(
                        add(_vk, 0x6a0),
                        0x2042a587a90c187b0a087c03e29c968b950b1db26d5c82d666905a6895790c0a
                    ) // K4 

                    mstore(
                        add(_vk, 0x6c0),
                        0x2e2b91456103698adf57b799969dea1c8f739da5d8d40dd3eb9222db7c81e881
                    ) // K5 

                    mstore(
                        add(_vk, 0x6e0),
                        0x1f20f5b0adb417179d42df7ddd4410a330afdb03e5c28949665b55adf7d7922d
                    ) // K6 

                     mstore(
                        add(_vk, 0x840),
                        0x1845622f9012f44d4d9b492a254a360ab98914fda5d90031b0a78009cbae56b8
                    ) // PLRANGE_X 

                    mstore(
                        add(_vk, 0x860),
                        0x2da38747c37c85b6815a9ac706a061274eaae8f7cfb4e3c83ad4112ba8dcc18f
                    ) // PLRANGE_Y 

                    mstore(
                        add(_vk, 0x880),
                        0x2db1df07e52bfc8c98d179cbf77bfbc27686bff2bbf4c5d891c3b207238f4536
                    ) // PLKEY_X 

                    mstore(
                        add(_vk, 0x8a0),
                        0x2141a52423d4b471e95e699e501c46a117511b877ea6f55927fa23251960898
                    ) // PLKEY_Y 

                    mstore(
                        add(_vk, 0x8c0),
                        0x1c6df5aefe9c29ffe46c5616432fd5183ae66e19beffb8c8bda713c4c6787921
                    ) // PLTDS_X 

                    mstore(
                        add(_vk, 0x8e0),
                        0xd0201154a22f7758ca6412800a93c4d4bc4ab7f888528ac972bdb8a41164d4b
                    ) // PLTDS_Y 

                     mstore(
                        add(_vk, 0x900),
                        0x13cce8db24c2f4be3d3e32a2683a7125b4a512fa4d991ee6d04e29800ffc328
                    ) // QDOMSEP_X 

                     mstore(
                        add(_vk, 0x920),
                        0x28fe3e1edc2878b070a0997ce5f2ad8ede2c4a8c51a272e23dcfec7df32e678d
                    ) // QDOMSEP_Y 

                    mstore(
                        add(_vk, 0x700),
                        0x1
                    ) // OKG1_X 

                    mstore(
                        add(_vk, 0x720),
                        0x2
                    ) // OKG1_Y 

                    mstore(
                        add(_vk, 0x740),
                        0x1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed
                    ) // OKG2_X1 

                    mstore(
                        add(_vk, 0x760),
                        0x198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2
                    ) // OKG2_X2 

                    mstore(
                        add(_vk, 0x780),
                        0x12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa
                    ) // OKG2_Y1 

                    mstore(
                        add(_vk, 0x7a0),
                        0x90689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b 
                    ) // OKG2_Y2 

                    mstore(
                        add(_vk, 0x7c0),
                        0x285b1f14edd7e6632340a37dfae9005ff762edcfecfe1c732a7474c0708bef80
                    ) // OKG3_X1 

                    mstore(
                        add(_vk, 0x7e0),
                        0x17cc93077f56f654da727c1def86010339c2b4131094547285adb083e48c197b
                    ) // OKG3_X2 

                    mstore(
                        add(_vk, 0x800),
                        0x2bad9a374aec49d329ec66e8f530f68509313450580c4c17c6db5ddb9bde7fd0
                    ) // OKG3_Y1 

                    mstore(
                        add(_vk, 0x820),
                        0x219edfceee1723de674f5b2f6fdb69d9e32dd53b15844956a630d3c7cdaa6ed9
                    ) // OKG3_Y2 

                    
                    mstore(_second_loc, 0x30644e5aaf0a66b91f8030da595e7d1c6787b9b45fc54c546729acf1ff053609) // N_INV_LOCATION 

                    mstore(add(_second_loc, 0x20), 0x2a734ebb326341efa19b0361d9130cd47b26b7488dc6d26eeccd4f3eb878331a) // OMEGA_LOCATION 

                    mstore(add(_second_loc, 0x40), 0x27f035bdb21de9525bcd0d50e993ee185f43327bf6a8efc445d2f3cb9550fe47) // OMEGA_INV_LOCATION 

                }
            }
        }