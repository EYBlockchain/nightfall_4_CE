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

                0x19e0282060fb3857d2b3d0cbc6d42c7cfc17a0f730de847d52cd41cef0e73046 

            ) 

            // sigma_comms_1.y 

            mstore( 

                add(mload(add(vk, 0x40)), 0x20), 

                0x56a73ea01489e4dbb0cd5e6a442d8403a04709edc2fa482441f144ea1d7e644 

            ) 

            // sigma_comms_2.x 

            mstore( 

                mload(add(vk, 0x60)), 

                0x29f7c27f23e824d479ddc274ec24170c892812d35e65ccb66d45a1fd572bbbe0 

            ) 

            // sigma_comms_2.y 

            mstore( 

                add(mload(add(vk, 0x60)), 0x20), 

                0x221c334161bf4e89264fe4cf79bc68871961ea01133a5597ecb24a175dbaeddd 

            ) 

            // sigma_comms_3.x 

            mstore( 

                mload(add(vk, 0x80)), 

                0x28c43cfc35bfd5ddced52ae4a20f0b7a3e9b3d0b2fe3d5d87e89eff499b3530f 

            ) 

            // sigma_comms_3.y 

            mstore( 

                add(mload(add(vk, 0x80)), 0x20), 

                0x289d1bcc7a53cdfd067cd34d77f0eccd61f90e6faefe4b2f6e70f060e67bd41 

            ) 

            // sigma_comms_4.x 

            mstore( 

                mload(add(vk, 0xa0)), 

                0x2aed70367456b9e99e3b2fff7210bc185ca97bbd94b3a3ab8342dff76d91a2dc 

            ) 

            // sigma_comms_4.y 

            mstore( 

                add(mload(add(vk, 0xa0)), 0x20), 

                0x2ef6f4fa0da23b219113b6741f50e07edd4c99a95a7a0b117044b9cd627deac8 

            ) 

            // sigma_comms_5.x 

            mstore( 

                mload(add(vk, 0xc0)), 

                0xa20b6e4b135f93c66016c68e58b9413bb6f045f33ae9ca8e663cb0176d156b4 

            ) 

            // sigma_comms_5.y 

            mstore( 

                add(mload(add(vk, 0xc0)), 0x20), 

                0x3f3f90b81ca3bf624e3fb6bbdb20a1faba6ad124297cea019ca63b34bfea29c 

            ) 

            // sigma_comms_6.x 

                mstore( 

                mload(add(vk, 0xe0)), 

                0x22962bda77c283dbe7fd3329c8e1d0875a2923218a378c8bc2ce698d9d37ecdf 

            ) 

            // sigma_comms_6.y 

            mstore( 

                add(mload(add(vk, 0xe0)), 0x20), 

                0x1d4e4ea4f13bdbbeeb8e8353a97c185f5e59f3c464d1a41a84d38a4f62f4737b 

            ) 

            // selector_comms_1.x 

            mstore( 

                mload(add(vk, 0x100)), 

                0x1c48a5bc3d08aa62552e1349f993b4cf5603f252f3cd4ebd3095de35db7eb678 

            ) 

            // selector_comms_1.y 

            mstore( 

                add(mload(add(vk, 0x100)), 0x20), 

                0x2f9837724fe4dc4e0ae5985eed8def81c113fdf1ab064b1c902b3f21c174f46f 

            ) 

            // selector_comms_2.x 

            mstore( 

                mload(add(vk, 0x120)), 

                0x28b933cf817ad2f71d99008a898d8c1de6a08a20d0c3215f3fa8c17236e7b8c1 

            ) 

            // selector_comms_2.y 

            mstore( 

                add(mload(add(vk, 0x120)), 0x20), 

                0x25af348bc6e56345dfc6b3eaa79f48f1b010b55bdeb67d6ae9d951bb4f719e0b 

            ) 

            // selector_comms_3.x 

            mstore( 

                mload(add(vk, 0x140)), 

                0x230db078355c9fb9b82eafa2e784d222e31aea6a11b54978daa6b7e392a3f724 

            ) 

            // selector_comms_3.y 

            mstore( 

                add(mload(add(vk, 0x140)), 0x20), 

                0x264e7bf85401171515fd7acc0355aa04f24dc36122f0a090d6d018d426b985c7 

            ) 

            // selector_comms_4.x 

            mstore( 

                mload(add(vk, 0x160)), 

                0x289aa52643c8d9b38350ce1a6e43b1c13a4e8b419addbb9bb4d7987d024ce480 

            ) 

            // selector_comms_4.y 

            mstore( 

                add(mload(add(vk, 0x160)), 0x20), 

                0x1eb8f917008b16d8a89f1ce12b3ab761175277bdde3939f2f4086c3b15ccd44 

            ) 

            // selector_comms_5.x 

            mstore( 

                mload(add(vk, 0x180)), 

                0x2e7dcdfb85d0b308acb1f2b703af7d262651edb9f1d5b0506b31a3f3eaa1c05 

            ) 

            // selector_comms_5.y 

            mstore( 

                add(mload(add(vk, 0x180)), 0x20), 

                0x1652c12c9734e88943407d8316ecb3c39d8554f2997d6eb0e12f9970304b31ca 

            ) 

            // selector_comms_6.x 

            mstore( 

                mload(add(vk, 0x1a0)), 

                0x178e18431069cf9b1eb7bf45ee08d91ed658bcdfe263b946d16908348caee2ae 

            ) 

            // selector_comms_6.y 

            mstore( 

                add(mload(add(vk, 0x1a0)), 0x20), 

                0x2149c4db462a4e1a5b2c9db1f1ca865c978a601eb1ab5da9454819eda90c853c 

            ) 

            // selector_comms_7.x 

            mstore( 

                mload(add(vk, 0x1c0)), 

                0xf112178be95aafd3890dd59c61a7753788839adb8b2fba4e80b137dd7d46fe5 

            ) 

            // selector_comms_7.y 

            mstore( 

                add(mload(add(vk, 0x1c0)), 0x20), 

                0xf61a4e7943e707ba2bfafa674e2fc4c82fbf74b74c104a72d6099385cfb58b7 

            ) 

            // selector_comms_8.x 

            mstore( 

                mload(add(vk, 0x1e0)), 

                0xd1f78835386de60c893892a8b1a90c6a467fea6c657594e213013ad681e7eba 

            ) 

            // selector_comms_8.y 

            mstore( 

                add(mload(add(vk, 0x1e0)), 0x20), 

                0xac9b49924fa895330ee753a2ecf32bf34ea7d35a8e8a8d0ab24fad0b0ce5e5 

            ) 

            // selector_comms_9.x 

            mstore( 

                mload(add(vk, 0x200)), 

                0x26a0ff65d492383e20addbd02c2873cddad72d8ae2320740d217c66be738847b 

            ) 

            // selector_comms_9.y 

            mstore( 

                add(mload(add(vk, 0x200)), 0x20), 

                0x126a189bad6ab561d2da3caaec67cc3e3fe1d8e29be19f38d9579118a68fae6f 

            ) 

            // selector_comms_10.x 

            mstore( 

                mload(add(vk, 0x220)), 

                0x2000b841c8c5a6d7a92fa411c9c8c1f3d64630757e0346c5739aecf5610d862a 

            ) 

            // selector_comms_10.y 

            mstore( 

                add(mload(add(vk, 0x220)), 0x20), 

                0x3a1240bd287c26a2f5c0d0a9b9eaa01f505b8bce4524528bcbf52c94b0c5f0 

            ) 

            // selector_comms_11.x 

            mstore( 

                mload(add(vk, 0x240)), 

                0x26a5e4d034de36fdd016d3f49b6d5fe8f4d3078feb450e2266225768134abad0 

            ) 

            // selector_comms_11.y 

            mstore( 

                add(mload(add(vk, 0x240)), 0x20), 

                0x167db88d7fae5c9cc03d88f8815035faf771612f098830038f3a5ac843d6096f 

            ) 

            // selector_comms_12.x 

            mstore( 

                mload(add(vk, 0x260)), 

                0x5f8f1ae1c0b7f3f2cabba34e8255dcbb0a1f3e6897e9628788aeefcf7f85076 

            ) 

            // selector_comms_12.y 

            mstore( 

                add(mload(add(vk, 0x260)), 0x20), 

                0x347811cb3646480f67d82b6280a9ee6bc844ddb931d78001dcdf2d044d9f4ad 

            ) 

            // selector_comms_13.x 

            mstore( 

                mload(add(vk, 0x280)), 

                0x1df3930334c5f9037acc46361c3d16429dbc9241c463576eaeebf52e7db30b68 

            ) 

            // selector_comms_13.y 

            mstore( 

                add(mload(add(vk, 0x280)), 0x20), 

                0x17a6f523f33f89ab19664bbae2468e2191624b02ca6e63b62bb77977a5eea67 

            ) 

            // selector_comms_14.x 

            mstore( 

                mload(add(vk, 0x2a0)), 

                0xaaa3abdd8e15fd452c38e7a322f8956a90047e2f8aa7f9e07cf0e2f97b238b2 

            ) 

            // selector_comms_14.y 

            mstore( 

                add(mload(add(vk, 0x2a0)), 0x20), 

                0x11988bdad3c991853b30806e5a84040b0a4090377e2c1c64df3e6ba462dd37fd 

            ) 

            // selector_comms_15.x 

            mstore( 

                mload(add(vk, 0x2c0)), 

                0xa4fcded78182d7182759c4fd87e9eabc45439d809f629dde06472493e91f446 

            ) 

            // selector_comms_15.y 

            mstore( 

                add(mload(add(vk, 0x2c0)), 0x20), 

                0x13aef74aa67fc149b5f9be4e543fca1ffa2578753bddce7c9fbbc6aabedaef63 

            ) 

            // selector_comms_16.x 

            mstore( 

                mload(add(vk, 0x2e0)), 

                0xc8862a862897875667775ba1436d66d80fd2f22523a0b718597a26f52785f4e 

            ) 

            // selector_comms_16.y 

            mstore( 

                add(mload(add(vk, 0x2e0)), 0x20), 

                0xc33a4236ddca63a4d956a077b8f0bb97c47c98d93f3db2af234d7c2169c2045 

            ) 

            // selector_comms_17.x 

            mstore( 

                mload(add(vk, 0x300)), 

                0x11430e349ed45209fb909b9676c758305ee6a27cd21e438561aa1a7053f7fc1a 

            ) 

            // selector_comms_17.y 

            mstore( 

                add(mload(add(vk, 0x300)), 0x20), 

                0x2e854bb624022cd36054efa31402d96abf600cc2bf7320ca051c67f26af94b7 

            ) 

            // selector_comms_18.x 

            mstore( 

                mload(add(vk, 0x320)), 

                0x100e5ba041d51b10bd46d8def071f4d0499479292577b98cf3fa1045ef53cdae 

            ) 

            // selector_comms_18.y 

            mstore( 

                add(mload(add(vk, 0x320)), 0x20), 

                0x1d65411efef853462f2f16d680cf10dc8acff75dd6c85e70ae2701ca74c855da 

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

                0x7bb8c2e0856b90743fa7c8934b4c9a38494f7ced78027c82183476ea4261854 

            ) 

            // key_table_comm.y 

            mstore( 

                add(mload(add(vk, 0x420)), 0x20), 

                0x1257c638fc73e31618f2dc4c1975a02abb4fd29ce5ff58e7bbc3523b833e0a2b 

            ) 

            // table_dom_sep_comm.x 

            mstore( 

                mload(add(vk, 0x440)), 

                0x1423f971d01c919216a5940633e80a6f625adaa4ff310d8c53da238b9bb129fc 

            ) 

            // table_dom_sep_comm.y 

            mstore( 

                add(mload(add(vk, 0x440)), 0x20), 

                0x56d531357c4a6d3a104cb67ee986d829dcac476bc715b33bf74fc4da24d78ce 

            ) 

            // q_dom_sep_comm.x 

            mstore( 

                mload(add(vk, 0x460)), 

                0x2b00368bee255b7636646e389ce67aec0952db520c751f327142b850ec4aa66c 

            ) 

            // q_dom_sep_comm.y 

            mstore( 

                add(mload(add(vk, 0x460)), 0x20), 

                0x1fc3064c96b764b9a1b3abccde9ebe6e80d217104cdf9f532134b6d60752a437 

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