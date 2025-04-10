// SPDX-License-Identifier: Apache-2.0 

        pragma solidity >=0.8.4; 

        
        library TestVK { 


            function getVerificationKeyHash() internal pure returns (bytes32) { 

        return 0x2bd1145cd2bc6bbccfe698bf3415882964e7f3dfebef3a2389d9f111c9d67868;
    } 


           function loadVerificationKey( 

             uint256 _vk, 

               uint256 _second_loc 

           ) internal pure { 

              assembly { 

                 mstore( 
                      add(_vk, 0x00), 
                      0x40 
                  ) // DOMAIN_SIZE 

                   mstore( 

                       add(_vk, 0x20), 

                  0x4 

                    ) // NUM_PUBLIC_INPUTS 

                    mstore(
                        add(_vk, 0x40),
                        0x25dd5ebea47039d4c6125242c1ed8fffd8d4e772a8d3b9909620a7256e6f2b30
                    ) // SIGMA1_X 

                    mstore(
                        add(_vk, 0x60),
                        0xe02da9e32d56966c698c403f94a4d6966fa332feb1a401185fa9f851aabef74
                    ) // SIGMA1_Y 

                    mstore(
                        add(_vk, 0x80),
                        0x27ed54734211f3657d45bdcffacb03d79cea52ce7aa79f56cb03d6e0b4d90c82
                    ) // SIGMA2_X 

                    mstore(
                        add(_vk, 0xa0),
                        0x2348bd4787e369ffc6a37407005b129741d2b6b5e0a7fa1138729141ceb3341f
                    ) // SIGMA2_Y 

                    mstore(
                        add(_vk, 0xc0),
                        0x24c0bfe1830a57ae57f0b1f14184d568a0b4156eb28e16dd52d0a941407df0d2
                    ) // SIGMA3_X 

                    mstore(
                        add(_vk, 0xe0),
                        0x2be2d0b3d0a2b799fa3be283cc272c04b545e2e7c8930347bea1cb0a969fb1f
                    ) // SIGMA3_Y 

                    mstore(
                        add(_vk, 0x100),
                        0x19865b7b84705c326954e7fde2076607ade318b2c11cfda2435f1c5e60fd770f
                    ) // SIGMA4_X 

                    mstore(
                        add(_vk, 0x120),
                        0xa6ba045c98a5ab98a26dc00f36dae166af5731a54518b64f670e081ea5cb9c0
                    ) // SIGMA4_Y 

                    mstore(
                        add(_vk, 0x140),
                        0x98c1262d3e4b45be13eb606c439ef41ba0a9652f6ac2a16b10b9e784270c6e2
                    ) // SIGMA5_X 

                    mstore(
                        add(_vk, 0x160),
                        0x280e399a534131ae9b779c2f01856c1caf407e960f274421532249c2fed9c4c0
                    ) // SIGMA5_Y 

                    mstore(
                        add(_vk, 0x180),
                        0x11d72e76f673a1a3a74ae813a8e59a53023f0c27d6a653e8f1a9be321e6310c
                    ) // SIGMA6_X 

                    mstore(
                        add(_vk, 0x1a0),
                        0x2d4838c5ecd8ab87b77af691cb2198c912101af4f7a00abbf5abfd39a4408e03
                    ) // SIGMA6_Y 

                    mstore(
                        add(_vk, 0x1c0),
                        0x10746f00300888acd92244afdb8a9aab21f47286afb551da9921edf2d19e4deb
                    ) // QLC1_X 

                    mstore(
                        add(_vk, 0x1e0),
                        0x156e92d2cef81cf0d1a373c758ec10def69a21a3fda7371992329f65b2f14a9b
                    ) // QLC1_Y 

                    mstore(
                        add(_vk, 0x200),
                        0x1bf9687824ef29ee722b5335256530d24ec44c38e1cc35b6f37ae821bc97ce82
                    ) // QLC2_X 

                    mstore(
                        add(_vk, 0x220),
                        0x1627b15980763db4f066a440c51f8de083f16086115fba1528aa0537150fe67f
                    ) // QLC2_Y 

                    mstore(
                        add(_vk, 0x240),
                        0xe7cfb67665f0121b9ad20294f911f77f8e71916f173fe5550e978f0a90e93e3
                    ) // QLC3_X 

                    mstore(
                        add(_vk, 0x260),
                        0x2955dcbb56f48f082c3c02b22a968fb7224df414627629e3b57a126744a534e8
                    ) // QLC3_Y 

                    mstore(
                        add(_vk, 0x280),
                        0xe7cfb67665f0121b9ad20294f911f77f8e71916f173fe5550e978f0a90e93e3
                    ) // QLC4_X 

                    mstore(
                        add(_vk, 0x2a0),
                        0x2955dcbb56f48f082c3c02b22a968fb7224df414627629e3b57a126744a534e8
                    ) // QLC4_Y 

                    mstore(
                        add(_vk, 0x2c0),
                        0x292c2a63e706c8855d4571ad1346c7e8ac95392046bb4b365ee4ecded6342c79
                    ) // QMUL1_X 

                    mstore(
                        add(_vk, 0x2e0),
                        0x9ee1f584acf156b0c95ef7fd8eb038d2e156d7d8e89ee90fa75307ba6adfff5
                    ) // QMUL1_Y 

                    mstore(
                        add(_vk, 0x300),
                        0xe7cfb67665f0121b9ad20294f911f77f8e71916f173fe5550e978f0a90e93e3
                    ) // QMUL2_X 

                    mstore(
                        add(_vk, 0x320),
                        0x2955dcbb56f48f082c3c02b22a968fb7224df414627629e3b57a126744a534e8
                    ) // QMUL2_Y 

                    mstore(
                        add(_vk, 0x340),
                        0xe7cfb67665f0121b9ad20294f911f77f8e71916f173fe5550e978f0a90e93e3
                    ) // QHASH1_X 

                    mstore(
                        add(_vk, 0x360),
                        0x2955dcbb56f48f082c3c02b22a968fb7224df414627629e3b57a126744a534e8
                    ) // QHASH1_Y 

                    mstore(
                        add(_vk, 0x380),
                        0xe7cfb67665f0121b9ad20294f911f77f8e71916f173fe5550e978f0a90e93e3
                    ) // QHASH2_X 

                    mstore(
                        add(_vk, 0x3a0),
                        0x2955dcbb56f48f082c3c02b22a968fb7224df414627629e3b57a126744a534e8
                    ) // QHASH2_Y 

                    mstore(
                        add(_vk, 0x3c0),
                        0xe7cfb67665f0121b9ad20294f911f77f8e71916f173fe5550e978f0a90e93e3
                    ) // QHASH3_X 

                    mstore(
                        add(_vk, 0x3e0),
                        0x2955dcbb56f48f082c3c02b22a968fb7224df414627629e3b57a126744a534e8
                    ) // QHASH3_Y 

                    mstore(
                        add(_vk, 0x400),
                        0xe7cfb67665f0121b9ad20294f911f77f8e71916f173fe5550e978f0a90e93e3
                    ) // QHASH4_X 

                    mstore(
                        add(_vk, 0x420),
                        0x2955dcbb56f48f082c3c02b22a968fb7224df414627629e3b57a126744a534e8
                    ) // QHASH4_Y 

                    mstore(
                        add(_vk, 0x440),
                        0x1821a958a1e74b7ad91a0cf1cf49ba3ef7473d3a8942be8707afa2eb4c624bb1
                    ) // QOUT_X 

                    mstore(
                        add(_vk, 0x460),
                        0x2b580d7e9c36ec6923fd8992f1ef5c2ee6549cb5bca59f11ccdc9ac85ed50ded
                    ) // QOUT_Y 

                    mstore(
                        add(_vk, 0x480),
                        0x2fd1c2ed85918b0f9ac3f77afa16a8a8b3ee97373fc4a54f17b55d394c8bb6e0
                    ) // QCONST_X 

                    mstore(
                        add(_vk, 0x4a0),
                        0x1c1070f8a63d9c591b4d992b831a88a0d5f59f3eeaf405c18101c875df10194c
                    ) // QCONST_Y 

                    mstore(
                        add(_vk, 0x4c0),
                        0xe7cfb67665f0121b9ad20294f911f77f8e71916f173fe5550e978f0a90e93e3
                    ) // QECC_X 

                    mstore(
                        add(_vk, 0x4e0),
                        0x2955dcbb56f48f082c3c02b22a968fb7224df414627629e3b57a126744a534e8
                    ) // QECC_Y 

                    mstore(
                        add(_vk, 0x500),
                        0xe7cfb67665f0121b9ad20294f911f77f8e71916f173fe5550e978f0a90e93e3
                    ) // QSWX1_X 

                    mstore(
                        add(_vk, 0x520),
                        0x2955dcbb56f48f082c3c02b22a968fb7224df414627629e3b57a126744a534e8
                    ) // QSWX1_Y 

                    mstore(
                        add(_vk, 0x540),
                        0xe7cfb67665f0121b9ad20294f911f77f8e71916f173fe5550e978f0a90e93e3
                    ) // QSWX2_X 

                    mstore(
                        add(_vk, 0x560),
                        0x2955dcbb56f48f082c3c02b22a968fb7224df414627629e3b57a126744a534e8
                    ) // QSWX2_Y 

                    mstore(
                        add(_vk, 0x580),
                        0xe7cfb67665f0121b9ad20294f911f77f8e71916f173fe5550e978f0a90e93e3
                    ) // QSWY1_X 

                    mstore(
                        add(_vk, 0x5a0),
                        0x2955dcbb56f48f082c3c02b22a968fb7224df414627629e3b57a126744a534e8
                    ) // QSWY1_Y 

                    mstore(
                        add(_vk, 0x5c0),
                        0xe7cfb67665f0121b9ad20294f911f77f8e71916f173fe5550e978f0a90e93e3
                    ) // QSWY2_X 

                    mstore(
                        add(_vk, 0x5e0),
                        0x2955dcbb56f48f082c3c02b22a968fb7224df414627629e3b57a126744a534e8
                    ) // QSWY2_Y 

                    mstore(
                        add(_vk, 0x600),
                        0x1c9d85d2eb2b8e1955e453fabc5ae6d2223ca5157df5fe87d3bd2b0ef7b78552 
                    ) // QLOOKUP_X 

                    mstore(
                        add(_vk, 0x620),
                        0x2700920f1d9f71da3efed1905a81525d16c5dea745fedda0ce33c9e9b782e463
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
                        0x2b7e7faacbc2d6b82e3979284ac2faab8900317b6be86d10c67f10a42d641019
                    ) // PLRANGE_X 

                    mstore(
                        add(_vk, 0x860),
                        0xdb181859e349e4e33d3a7a2e9e9fc90b211f2acdcc29cabde2c06c2138e738b
                    ) // PLRANGE_Y 

                    mstore(
                        add(_vk, 0x880),
                        0x11bc5e02fb2123200692127c11cc52265fe4aa72dbbe2bffcf125a901b7f8dad
                    ) // PLKEY_X 

                    mstore(
                        add(_vk, 0x8a0),
                        0x2df3464dbb2f4032c56f1073e7e53ce3579307b2484ceb45bfa6d98df1dadc2
                    ) // PLKEY_Y 

                    mstore(
                        add(_vk, 0x8c0),
                        0x2fb870077e6f4f64deff0d463ac6164ac36d50258b2cff0970490c0c66c417a3
                    ) // PLTDS_X 

                    mstore(
                        add(_vk, 0x8e0),
                        0x260efd3800d2850be44395eea4ee8275e3ce8c27ce1d251f36367731c022aa8
                    ) // PLTDS_Y 

                     mstore(
                        add(_vk, 0x900),
                        0x1b64f549d07963a9171d4ff28c3fbe59cc0b5850fa4e4cb16ce8cad44d4d0eb
                    ) // QDOMSEP_X 

                     mstore(
                        add(_vk, 0x920),
                        0x236ed45677917df3054774346cbf24c8314ba62aa89208a42cf95bda34e1d5aa
                    ) // QDOMSEP_Y 

                    mstore(
                        add(_vk, 0x700),
                        0x10ff4ab61e6109f64b103c45770a0c2ec238622df2dddbf4aee2683203a35b7f
                    ) // OKG1_X 

                    mstore(
                        add(_vk, 0x720),
                        0x10ff352f0d2acf65fcd3a0517bb3dba40d462da489a9041ffd6e02a1f1ef6100
                    ) // OKG1_Y 

                    mstore(
                        add(_vk, 0x740),
                        0x29fd9755178e22781e448030a456ebfbdbb449335804c09e815f0b7a8437b94d
                    ) // OKG2_X1 

                    mstore(
                        add(_vk, 0x760),
                        0x1eb826b8b6e3ebbb3a1607ebf19d2758c162d2c915188d49191d80a4f88e5306
                    ) // OKG2_X2 

                    mstore(
                        add(_vk, 0x780),
                        0x182766815b58475e731a8afcff419c841a4ed2f571e3f9d4b1af94bc5e7d5777
                    ) // OKG2_Y1 

                    mstore(
                        add(_vk, 0x7a0),
                        0x29eca2bd88436dab9a7bd738d120579d2dbcc9abc980d32aba2449d68b0a9457 
                    ) // OKG2_Y2 

                    mstore(
                        add(_vk, 0x7c0),
                        0x1d9b573a9b30ead10dcf030d1ab3c9ec81dc3da2aac764597280370a6b29baab
                    ) // OKG3_X1 

                    mstore(
                        add(_vk, 0x7e0),
                        0x1dd13357222eab4fb810d5c89b5af426816cd0492532f7f181bb44e39cbc2be4
                    ) // OKG3_X2 

                    mstore(
                        add(_vk, 0x800),
                        0x2b9ee7fd0e19d5ec504255b3090e52ab453425e7b43c170022f6f862f7cc2291
                    ) // OKG3_Y1 

                    mstore(
                        add(_vk, 0x820),
                        0x20ca4b8da283890ea4ab8ac17f07102e0e3bcd102998e3bb16349b6005b02de4
                    ) // OKG3_Y2 

                    
                    mstore(_second_loc, 0x2fa2bd3915acd9a9116f049fa77b52fbb39318a757d28acefed26dbda0400001) // N_INV_LOCATION 

                    mstore(add(_second_loc, 0x20), 0x1418144d5b080fcac24cdb7649bdadf246a6cb2426e324bedb94fb05118f023a) // OMEGA_LOCATION 

                    mstore(add(_second_loc, 0x40), 0x26177cf2b2a13d3a035cdc7567a8a676d80396ec1d3213ee78ce6a0b763d698f) // OMEGA_INV_LOCATION 

                }
            }
        }