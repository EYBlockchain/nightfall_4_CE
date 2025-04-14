// SPDX-License-Identifier: Apache-2.0 

        pragma solidity >=0.8.4; 

        
        library UltraVK { 


            function getVerificationKeyHash() internal pure returns (bytes32) { 

        return 0x036c98aa534dd7b62a829e0c89d2579a025098b11139ce8aed7a6010cff902ff;
    } 


           function loadVerificationKey( 

             uint256 _vk, 

               uint256 _second_loc 

           ) internal pure { 

              assembly { 

                 mstore( 
                      add(_vk, 0x00), 
                      0x1000000 
                  ) // DOMAIN_SIZE 

                   mstore( 

                       add(_vk, 0x20), 

                  0x1 

                    ) // NUM_PUBLIC_INPUTS 

                    mstore(
                        add(_vk, 0x40),
                        0x11847f78338c5fc6744fc81bc8af4f2cf7eeadb90564cbf7a838a3f60db7f5bb
                    ) // SIGMA1_X 

                    mstore(
                        add(_vk, 0x60),
                        0x71343968552b4a06a7532ae390fd582fa88e9494483e1367dbb7a4a50dd1f23
                    ) // SIGMA1_Y 

                    mstore(
                        add(_vk, 0x80),
                        0x14de3c07d0f9375f04672b132570fcaa7430465ad366b09f8672a48e03cd29ea
                    ) // SIGMA2_X 

                    mstore(
                        add(_vk, 0xa0),
                        0x27e1dcf3b60552be1fb9dfa1e6a631cfc68e126ba5227f505a5dca65469d2bdc
                    ) // SIGMA2_Y 

                    mstore(
                        add(_vk, 0xc0),
                        0x10768a4b5f5d62d36e9e0016fa667c6e73b34935d62d03fd9bd3ff167c66d50d
                    ) // SIGMA3_X 

                    mstore(
                        add(_vk, 0xe0),
                        0x8b1b80326dc36233e9d2be509e23b58e2e91f38a3a87a284fc5240d0a0d3826
                    ) // SIGMA3_Y 

                    mstore(
                        add(_vk, 0x100),
                        0x78b23b740d207983e2485af2500b7b37cd98c6d85ccc32d2b7cbc09d4bbb28f
                    ) // SIGMA4_X 

                    mstore(
                        add(_vk, 0x120),
                        0xe3b51cb0a51b799ab1729a063370a6212a5a570ee27ca24562cfa8c42e4aa2b
                    ) // SIGMA4_Y 

                    mstore(
                        add(_vk, 0x140),
                        0x58d736a5ebc114c98fbd86522171d600296ffe3d7bccd5c0bfe8ab4e2222be2
                    ) // SIGMA5_X 

                    mstore(
                        add(_vk, 0x160),
                        0x105f1a8c903dc944c83f031e9aff349d25d4de14de52fe7e6cdeae3a1f60abe9
                    ) // SIGMA5_Y 

                    mstore(
                        add(_vk, 0x180),
                        0x6052db148bcbdcf65ad2ed194497f817f0c523f892e4fa8b7f477d2aba209e0
                    ) // SIGMA6_X 

                    mstore(
                        add(_vk, 0x1a0),
                        0x256e79c3f5b839bf4eec7f5a0973adf0bb2d4b800f01c7670b5e61ca489d36a8
                    ) // SIGMA6_Y 

                    mstore(
                        add(_vk, 0x1c0),
                        0xab82ba4ba120c544bd44d733deffb3ed86967c5708b73455688619ccc82a86d
                    ) // QLC1_X 

                    mstore(
                        add(_vk, 0x1e0),
                        0x2091ac7a95623cd087ae326709d45920c165fed62d0663d9386c71132c60e177
                    ) // QLC1_Y 

                    mstore(
                        add(_vk, 0x200),
                        0x11b0b9c9e297989005ae83ee68b3dc2927eda65d7b99838b630b8f5e43151f46
                    ) // QLC2_X 

                    mstore(
                        add(_vk, 0x220),
                        0x1e42c001a29b699379f6f65f76aab872f7db37893fdeef10e1e47288a8291141
                    ) // QLC2_Y 

                    mstore(
                        add(_vk, 0x240),
                        0x2ae10f6778f5006cdf08e9e503d9f8a4e709f3fbeb154b7d75b6aafde1ddcd78
                    ) // QLC3_X 

                    mstore(
                        add(_vk, 0x260),
                        0x29bbce539e6636e1584aa776495c5e655396688a5312d54918cfa6cb578fb86a
                    ) // QLC3_Y 

                    mstore(
                        add(_vk, 0x280),
                        0x2c7ecb0ef6738f96e9935e04790a450ebfb6e4811770b07a5ffac637c02f7180
                    ) // QLC4_X 

                    mstore(
                        add(_vk, 0x2a0),
                        0x1d0682bafc7f30b92d87921599c877a18886cd81d05d39dad764cdb49a8f2a6a
                    ) // QLC4_Y 

                    mstore(
                        add(_vk, 0x2c0),
                        0x1206b2ec4ff795712a3902397f1cac4deff891624dec7a3e8fdb55c8aa5e1bb0
                    ) // QMUL1_X 

                    mstore(
                        add(_vk, 0x2e0),
                        0x238a2a153028d73ebae2bdbc80b142bb70c8655e039443585845a431a484b6aa
                    ) // QMUL1_Y 

                    mstore(
                        add(_vk, 0x300),
                        0x1199a315f8e95791b966bfbe8420e5e247f42888fbef8f012180539e70323acd
                    ) // QMUL2_X 

                    mstore(
                        add(_vk, 0x320),
                        0x1fe470c6b49916974398258862d2864c33be37c8595fec7434bcad43f3d2d6cd
                    ) // QMUL2_Y 

                    mstore(
                        add(_vk, 0x340),
                        0x22531a8837332ff0c9182756d022e3baa242568b900ea074a9ff2c3ce34ac4ac
                    ) // QHASH1_X 

                    mstore(
                        add(_vk, 0x360),
                        0x1934bb0cd04cb48c975ea5f894f5d40aa06903bbddbf931db0e542b317975a18
                    ) // QHASH1_Y 

                    mstore(
                        add(_vk, 0x380),
                        0x13ae5c7f9396e5c838ca080e82d21aedd96d7487d7b5bee29e41b56665ac629e
                    ) // QHASH2_X 

                    mstore(
                        add(_vk, 0x3a0),
                        0x624bbc8f4be3f21f53a176df4f157cd95b25e9da0a7bc5918c4d1d0a352ddaf
                    ) // QHASH2_Y 

                    mstore(
                        add(_vk, 0x3c0),
                        0x18bed3c27b7b394893234d5189349bbcc93c3ff7e46b631eead936c27743ad84
                    ) // QHASH3_X 

                    mstore(
                        add(_vk, 0x3e0),
                        0x10b705fc4f23d92fcc79a7c495bbf25041e363f1ab10e6ac1ed8d96d03e94d2e
                    ) // QHASH3_Y 

                    mstore(
                        add(_vk, 0x400),
                        0x18345ac64163dc3080d737b589b60a6bf0fa3fbb7c0a03b2433986d9dcf0a502
                    ) // QHASH4_X 

                    mstore(
                        add(_vk, 0x420),
                        0xd18d680eecb2abf43bc4a55d00ac15e98d76634c4127b39d77d496b914a19be
                    ) // QHASH4_Y 

                    mstore(
                        add(_vk, 0x440),
                        0x91d57f6c0da8f0394595ea6218190d61fa35bd552ff682803fd8edda22588b5
                    ) // QOUT_X 

                    mstore(
                        add(_vk, 0x460),
                        0x2a3bf63c7d5d300e0690e4b841475088cd02b55cc58657302aea497d4c03a7c5
                    ) // QOUT_Y 

                    mstore(
                        add(_vk, 0x480),
                        0x2c5ebba128c309968d9231db3607233c463af54ec4f9f998e05ded219e5b3435
                    ) // QCONST_X 

                    mstore(
                        add(_vk, 0x4a0),
                        0x27fc47add6837f16001478821b62e60741d267f40b77cd7afe2b5db4d1d3cb21
                    ) // QCONST_Y 

                    mstore(
                        add(_vk, 0x4c0),
                        0xfce45aef9cf1ba27ffea0a4d6334121e092cbbf921decd84f9a4c8e254c28ec
                    ) // QECC_X 

                    mstore(
                        add(_vk, 0x4e0),
                        0x279d10d8a5f1921b62fac827c9dae168384a537da7519a170663493899acb7da
                    ) // QECC_Y 

                    mstore(
                        add(_vk, 0x500),
                        0x212a2ad098798aea063527c3344bfbcfe9dc01fe266506dd5e147a5353ba304f
                    ) // QSWX1_X 

                    mstore(
                        add(_vk, 0x520),
                        0x155767229bb5bc32ab8f9211a6c70cc2f5fc1d5aa8e13f7fef6e2e78c39693bb
                    ) // QSWX1_Y 

                    mstore(
                        add(_vk, 0x540),
                        0x22351632c43839eca3c21b319751ba663afe2344f499e57ae6a5d059facdcee
                    ) // QSWX2_X 

                    mstore(
                        add(_vk, 0x560),
                        0x27fd90bfbec06c4067d7566886a27c9824b9a243decc80debcb4def5d5f44204
                    ) // QSWX2_Y 

                    mstore(
                        add(_vk, 0x580),
                        0x12bd139631370e41861068a8686c35c78f84c5c6cda641d88099f72a7666e5e9
                    ) // QSWY1_X 

                    mstore(
                        add(_vk, 0x5a0),
                        0x2fd617a85e263c73e3c6078f4b209756a25af8afdbb202fed9dcb37e02570a50
                    ) // QSWY1_Y 

                    mstore(
                        add(_vk, 0x5c0),
                        0x2e09ef09bdc7d84a1d69b5429a5512385cd7b89198dc48c228a15e7d2fd8580b
                    ) // QSWY2_X 

                    mstore(
                        add(_vk, 0x5e0),
                        0x1aca801510ef5f59f03e4f1cc637b573a6dd3f9efb83dc99a2fb4d806e6246f7
                    ) // QSWY2_Y 

                    mstore(
                        add(_vk, 0x600),
                        0x26a52929d191140f549dda36f835db9454972bf1ffe689ca8fbbe9aafc379cc3 
                    ) // QLOOKUP_X 

                    mstore(
                        add(_vk, 0x620),
                        0x100f59c2011766d0c703417d855dc5841ae8a6442bd112800ae1a4d2d372132c
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
                        0x21cfd2471450f10a247629878bea8d59fb97c85cc47582abbe464906069abe9
                    ) // PLRANGE_X 

                    mstore(
                        add(_vk, 0x860),
                        0x257d7c231b9593f596fe4a8ab093375cda10bd379389b41f3f70af783a21a359
                    ) // PLRANGE_Y 

                    mstore(
                        add(_vk, 0x880),
                        0x1ef3e52d588b7bca236653ec530021560c6782abe090bc37b43dfcea7baa86e1
                    ) // PLKEY_X 

                    mstore(
                        add(_vk, 0x8a0),
                        0xe40611368aa4e2ec8ff70b49e77559ca5b3258e1390f1e02f45e5772581bf7f
                    ) // PLKEY_Y 

                    mstore(
                        add(_vk, 0x8c0),
                        0x253b6637714d33cbc6b6cf813ddd38fb152d2a6827639d584ac96121757faeb6
                    ) // PLTDS_X 

                    mstore(
                        add(_vk, 0x8e0),
                        0x1cc6666c8892b9283bee5d5ad3fc466245def5d5f9d159a440155af29d523d18
                    ) // PLTDS_Y 

                     mstore(
                        add(_vk, 0x900),
                        0x27203ddcf603c3a64a7537c77ca7e757e6bd8728061bcb035e056231b51a61d3
                    ) // QDOMSEP_X 

                     mstore(
                        add(_vk, 0x920),
                        0x50146fa7605b67e3297042cec34cded1fc6425dc22fbb1b276036babb00e832
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
                        0x2fb108fa1410110197702abf48a2a4082ac69797fefabd875eae2441ac01cb97
                    ) // OKG2_X1 

                    mstore(
                        add(_vk, 0x760),
                        0xe8027dd7270d8140147261462b96bb2cb41df5614d9958765ef6fefafa4c971
                    ) // OKG2_X2 

                    mstore(
                        add(_vk, 0x780),
                        0x26273f16484917c81b833ad7ef4336a3ed59ab3dd1a2e1b34cb1992072f341d8
                    ) // OKG2_Y1 

                    mstore(
                        add(_vk, 0x7a0),
                        0x92ea9592c4ab2f4b6a63e16eb8e5e6ce0dbfbcdbbb90c690c7ac7f8a60f76c6 
                    ) // OKG2_Y2 

                    mstore(
                        add(_vk, 0x7c0),
                        0x4620580e72f066d8c9bb495edb136c66e943b96bfdd6add73f87e2961b0bbb5
                    ) // OKG3_X1 

                    mstore(
                        add(_vk, 0x7e0),
                        0xa30ec13879cde0806c1cabba85675a58ea0f47a2bc9a7cebe898111a9a3ca2c
                    ) // OKG3_X2 

                    mstore(
                        add(_vk, 0x800),
                        0x1539eba7f2e61fe4f91d1e79c2825e08564c851ac904085e3f29e6b246de2e50
                    ) // OKG3_Y1 

                    mstore(
                        add(_vk, 0x820),
                        0x18bc34689dcbd6b67b98b268dda679bc91f99178fd44345899a8ff5a0ad3fd3e
                    ) // OKG3_Y2 

                    
                    mstore(_second_loc, 0x30644e427ce32d4886b01bfe313ba1dba6db8b2045d128178a7164500e0a6c11) // N_INV_LOCATION 

                    mstore(add(_second_loc, 0x20), 0xc9fabc7845d50d2852e2a0371c6441f145e0db82e8326961c25f1e3e32b045b) // OMEGA_LOCATION 

                    mstore(add(_second_loc, 0x40), 0x2710c370db50e9cda334d3179cd061637be1488db323a16402e1d4d1110b737b) // OMEGA_INV_LOCATION 

                }
            }
        }