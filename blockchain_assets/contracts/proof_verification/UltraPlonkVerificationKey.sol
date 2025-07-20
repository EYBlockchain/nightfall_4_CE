// SPDX-License-Identifier: Apache-2.0 
pragma solidity ^0.8.20;

import "./Types.sol";

library UltraPlonkVerificationKey {
    function getVerificationKey() internal pure returns (Types.VerificationKey memory vk) {
             assembly {
            // domain_size
            mstore(add(vk, 0x00), 512)
            // num_inputs
            mstore(add(vk, 0x20), 1)
            // sigma_comms_1.x
            mstore(
                mload(add(vk, 0x40)),
                0x2F5BB5FDB1E276AB3296410B2E37950DBEC8FB71D33FD8C06FF37187867E64C2
            )
            // sigma_comms_1.y
            mstore(
                add(mload(add(vk, 0x40)), 0x20),
                0xE93AF618A1F0C36812D866E64FC1E67AAE9A38DA0B0DBB892A81B9596CA9526
            )
            // sigma_comms_2.x
            mstore(
                mload(add(vk, 0x60)),
                0x191347D5516C89251606C73F1AAEDE230315CCE8D119DBB8F85EC7ADCA58FECF
            )
            // sigma_comms_2.y
            mstore(
                add(mload(add(vk, 0x60)), 0x20),
                0xD51DA4FACEF5A9A1F66CAE614A9729AE73CBA7E910DE1E398890BFB19479E4D
            )
            // sigma_comms_3.x
            mstore(
                mload(add(vk, 0x80)),
                0x152DDE58EAB59643AC5E43697C9F224183B4FA11E3A1A2A159C775AB2123598B
            )
            // sigma_comms_3.y
            mstore(
                add(mload(add(vk, 0x80)), 0x20),
                0x27DC6260745CA0D7D34369926A2FB01A2B5F9DAD22284AFCD741DB392C136572
            )

            // sigma_comms_4.x
            mstore(
                mload(add(vk, 0xa0)),
                0x2F23DFE6B5F7375F0E0D27F7386B95472ACB6E307E6794387D43B8A41851A738
            )
            // sigma_comms_4.y
            mstore(
                add(mload(add(vk, 0xa0)), 0x20),
                0x1AA7E43E0C46B17121B3E336E2F0DDF21920247FE3F163DCB86277533E6D5368
            )

            // sigma_comms_5.x
            mstore(
                mload(add(vk, 0xc0)),
                0xC2BBD36AF72B26237A1A65C5078E16D52EE5815037B8ADCBEEAE664B12D09AB
            )
            // sigma_comms_5.y
            mstore(
                add(mload(add(vk, 0xc0)), 0x20),
                0xD6522D528F18B34B9ECA3DA372B38355441EBC1052EBD6336FBAD629DD69530
            )

            // sigma_comms_6.x
            mstore(
                mload(add(vk, 0xe0)),
                0x2B6B34697EDC9241665F312E085493A73D96C936270084A53B7E79944F5030C7
            )
            // sigma_comms_6.y
            mstore(
                add(mload(add(vk, 0xe0)), 0x20),
                0x2C2FDD8C0EF5D94FCFEFC694906A40183B7EA89EA3F57632CC4DEB833B576EB1
            )
            // // selector_comms_1
            mstore(
                mload(add(vk, 0x100)),  
                0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F
            )
            mstore(
                add(mload(add(vk, 0x100)), 0x20),
                0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287
            )
            // selector_comms_2
            mstore(
                mload(add(vk, 0x120)),
                0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F
            )
            mstore(
                add(mload(add(vk, 0x120)), 0x20),
                0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287
            )
            // selector_comms_3
            mstore(
                mload(add(vk, 0x140)),
                0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F
            )
            mstore(
                add(mload(add(vk, 0x140)), 0x20),
                0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287
            )
            // selector_comms_4
            mstore(
                mload(add(vk, 0x160)),
                0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F
            )
            mstore(
                add(mload(add(vk, 0x160)), 0x20),
                0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287
            )
            // selector_comms_5
            mstore(
                mload(add(vk, 0x180)),
                0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F
            )
            mstore(
                add(mload(add(vk, 0x180)), 0x20),
                0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287
            )
            // selector_comms_6
            mstore(
                mload(add(vk, 0x1a0)),
                0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F
            )
            mstore(
                add(mload(add(vk, 0x1a0)), 0x20),
                0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287
            )
            // selector_comms_7
            mstore(
                mload(add(vk, 0x1c0)),
                0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F
            )
            mstore(
                add(mload(add(vk, 0x1c0)), 0x20),
                0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287
            )
            // selector_comms_8
            mstore(
                mload(add(vk, 0x1e0)),
                0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F
            )
            mstore(
                add(mload(add(vk, 0x1e0)), 0x20),
                0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287
            )
            // selector_comms_9
            mstore(
                mload(add(vk, 0x200)),
                0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F
            )
            mstore(
                add(mload(add(vk, 0x200)), 0x20),
                0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287
            )
            // selector_comms_10
            mstore(
                mload(add(vk, 0x220)),
                0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F
            )
            mstore(
                add(mload(add(vk, 0x220)), 0x20),
                0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287
            )
            // selector_comms_11
            mstore(
                mload(add(vk, 0x240)),
                0xE540ED913C934CE103797CA49597514B737221427B65DE4193A24D22D35A6A0
            )
            mstore(
                add(mload(add(vk, 0x240)), 0x20),
                0x301D93C3824E94953FBB3CF62DD9C715FFF5091191A9C0746EA2344439C5F757
            )
            // selector_comms_12
            mstore(
                mload(add(vk, 0x260)),
                0xEEB0057D511309B6D4CD782C845A8F2E4C0A38ED864537A11459A31C4100F17
            )
            mstore(
                add(mload(add(vk, 0x260)), 0x20),
                0xB267413389DD11E903F58A5E319F92D81177B27D959449C67DACA851903AFC6
            )
            // selector_comms_13
            mstore(
                mload(add(vk, 0x280)),
                0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F
            )
            mstore(
                add(mload(add(vk, 0x280)), 0x20),
                0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287
            )
            // selector_comms_14  
            mstore(
                mload(add(vk, 0x2a0)),
                0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F
            )
            mstore(
                add(mload(add(vk, 0x2a0)), 0x20),
                0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287
            )
            // selector_comms_15
            mstore(
                mload(add(vk, 0x2c0)),
                0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F
            )
            mstore(
                add(mload(add(vk, 0x2c0)), 0x20),
                0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287  
            )
            // selector_comms_16
            mstore(
                mload(add(vk, 0x2e0)),
                0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F
            )
            mstore(
                add(mload(add(vk, 0x2e0)), 0x20),
                0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287
            )
            // selector_comms_17
            mstore(
                mload(add(vk, 0x300)),
                0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F
            )
            mstore(
                add(mload(add(vk, 0x300)), 0x20),
                0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287
            )
            // selector_comms_18
            mstore(
                mload(add(vk, 0x320)),
                0x14E6B4B2E0AB16AF07895F5323C577819F76AFADD3B654D05864E3A8D2569529
            )
            mstore(
                add(mload(add(vk, 0x320)), 0x20),
                0x291747F35CB51FDDCD8F87CBB0F1AD48E9274BEA9D4F788FE791D3F545872F88
            )

            // k1
            mstore(add(vk, 0x340), 1)
            // k2
            mstore(
                add(vk, 0x360),
                0x2F8DD1F1A7583C42C4E12A44E110404C73CA6C94813F85835DA4FB7BB1301D4A
            )
            // k3
            mstore(
                add(vk, 0x380),
                0x1EE678A0470A75A6EAA8FE837060498BA828A3703B311D0F77F010424AFEB025
            )
            // k4
            mstore(
                add(vk, 0x3a0),
                0x2042A587A90C187B0A087C03E29C968B950B1DB26D5C82D666905A6895790C0A
            )
            // k5
            mstore(
                add(vk, 0x3c0),
                0x2E2B91456103698ADF57B799969DEA1C8F739DA5D8D40DD3EB9222DB7C81E881
            )
            // k6
            mstore(
                add(vk, 0x3e0),
                0x1F20F5B0ADB417179D42DF7DDD4410A330AFDB03E5C28949665B55ADF7D7922D
            )
            // range_table_comm
            mstore(
                mload(add(vk, 0x400)),
                0x5CB0B9FC82EFF921CCD085F1254A3A8FBE2C82E79ABFDD99CB2F2C8353D5E8
            )
            mstore(
                add(mload(add(vk, 0x400)), 0x20),
                0x3DEAC380246DA8595AE1A8AF4B8246A53F4A3B0CA66252B34B915205A311162
            )
            // key_table_comm
            mstore(
                mload(add(vk, 0x420)),
                0x26015DD551EF9EB178392DB023F08B8AF77AA5FABDCF53CB1201ADDEBF9E1A67
            )
            mstore(
                add(mload(add(vk, 0x420)), 0x20),
                0xEB1D734631243278AD0C244DA67CBC72A8C3367F07188E22689B2D4C2F91F6A
            )
            // table_dom_sep_comm
            mstore(
                mload(add(vk, 0x440)),
                0x2960E04A2EEE47A5373512D4181ECDD9F6B7C8C1875DA9BEAE0A934CE071638D
            )
            mstore(
                add(mload(add(vk, 0x440)), 0x20),
                0x7AB73985287B41F40028D108E97686F674EA59588C20027B791F93EB7FC8AFD
            )
            // q_dom_sep_comm
            mstore(
                mload(add(vk, 0x460)),
                0x2C51031B1329ECB7043CA0B1A39BEDDC2A4C42AF8DC91062FD3ADE6EA9B62C48
            )
            mstore(
                add(mload(add(vk, 0x460)), 0x20),
                0x141587296A82B2E51B417B9D7930A22D92B0C028E16CA94CE3C808419F03560
            )
        }
        // // vk.sigma_comms_6= Types.G1Point(0x283BBA8A33ED122DA7AD7F26BA7A2D5407FACA0A0D6CFA935F647C60C914B723, 0xDBFBABB7C023EF0DA02D99D1515897019BCA7CC41C4B47161B51C0F27F52956);

        // vk.selector_comms_1 = Types.G1Point(0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F, 0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287);
        // vk.selector_comms_2 = Types.G1Point(0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F, 0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287);
        // vk.selector_comms_3 = Types.G1Point(0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F, 0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287);
        // vk.selector_comms_4 = Types.G1Point(0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F, 0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287);
        // vk.selector_comms_5 = Types.G1Point(0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F, 0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287);
        // vk.selector_comms_6 = Types.G1Point(0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F, 0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287);
        // vk.selector_comms_7 = Types.G1Point(0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F, 0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287);
        // vk.selector_comms_8 = Types.G1Point(0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F, 0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287);
        // vk.selector_comms_9 = Types.G1Point(0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F, 0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287);
        // vk.selector_comms_10 = Types.G1Point(0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F, 0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287);
        // vk.selector_comms_11 = Types.G1Point(0xE540ED913C934CE103797CA49597514B737221427B65DE4193A24D22D35A6A0, 0x301D93C3824E94953FBB3CF62DD9C715FFF5091191A9C0746EA2344439C5F757);
        // vk.selector_comms_12 = Types.G1Point(0xEEB0057D511309B6D4CD782C845A8F2E4C0A38ED864537A11459A31C4100F17, 0xB267413389DD11E903F58A5E319F92D81177B27D959449C67DACA851903AFC6);
        // vk.selector_comms_13 = Types.G1Point(0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F, 0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287);
        // vk.selector_comms_14 = Types.G1Point(0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F, 0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287);
        // vk.selector_comms_15 = Types.G1Point(0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F, 0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287);
        // vk.selector_comms_16 = Types.G1Point(0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F, 0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287);
        // vk.selector_comms_17 = Types.G1Point(0x2C070E98C8202FCC3A8CAC5DFCAB636A125C2C53F7BA36A2178C545131DB2D9F, 0x1A3298B6427790A076429AAF4FD8554A022989D298E6636F295EC239EDE33287);
        // vk.selector_comms_18 = Types.G1Point(0x14E6B4B2E0AB16AF07895F5323C577819F76AFADD3B654D05864E3A8D2569529, 0x291747F35CB51FDDCD8F87CBB0F1AD48E9274BEA9D4F788FE791D3F545872F88);

        

        // vk.range_table_comm = Types.G1Point(
        //     0x5CB0B9FC82EFF921CCD085F1254A3A8FBE2C82E79ABFDD99CB2F2C8353D5E8,
        //     0x3DEAC380246DA8595AE1A8AF4B8246A53F4A3B0CA66252B34B915205A311162
        // );
        // vk.key_table_comm = Types.G1Point(0x26015DD551EF9EB178392DB023F08B8AF77AA5FABDCF53CB1201ADDEBF9E1A67, 0xEB1D734631243278AD0C244DA67CBC72A8C3367F07188E22689B2D4C2F91F6A);
        // vk.table_dom_sep_comm = Types.G1Point(0x2960E04A2EEE47A5373512D4181ECDD9F6B7C8C1875DA9BEAE0A934CE071638D, 0x7AB73985287B41F40028D108E97686F674EA59588C20027B791F93EB7FC8AFD);
        // vk.q_dom_sep_comm = Types.G1Point(0x2C51031B1329ECB7043CA0B1A39BEDDC2A4C42AF8DC91062FD3ADE6EA9B62C48, 0x141587296A82B2E51B417B9D7930A22D92B0C028E16CA94CE3C808419F03560);
        return vk;
    }
}