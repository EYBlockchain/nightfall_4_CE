pragma solidity ^0.8.20;

library Types {
    // If a G1Point is infinity, for simplicity set x and y to zero
    // struct G1Point {
    //     uint256 x;
    //     uint256 y;
    // }
    struct G2Point {
        uint256 x0;
        uint256 x1;
        uint256 y0;
        uint256 y1;
    }
    struct PcsInfo {
        // the shifted point to be evaluated at
        uint256 nextEvalPoint; // 0x00
        // the polynomial evaluation value
        uint256 eval; // 0x20
        // scalars of poly comm for MSM
        uint256[] commScalars; // 0x40
        // bases of poly comm for MSM
        Types.G1Point[] commBases; // 0x60
    }
    struct Proof {
        G1Point wires_poly_comms_1; // 0x00
        G1Point wires_poly_comms_2; // 0x20
        G1Point wires_poly_comms_3; // 0x40
        G1Point wires_poly_comms_4; // 0x60
        G1Point wires_poly_comms_5; // 0x80
        G1Point wires_poly_comms_6; // 0xa0
        G1Point prod_perm_poly_comm; // 0xc0
        G1Point split_quot_poly_comms_1; // 0xe0
        G1Point split_quot_poly_comms_2; // 0x100
        G1Point split_quot_poly_comms_3; // 0x120
        G1Point split_quot_poly_comms_4; // 0x140
        G1Point split_quot_poly_comms_5; // 0x160
        G1Point split_quot_poly_comms_6; // 0x180
        G1Point h_poly_comm_1; // 0x1a0
        G1Point h_poly_comm_2; // 0x1c0
        G1Point prod_lookup_poly_comm; // 0x1e0
        uint256 wires_evals_1; // 0x200
        uint256 wires_evals_2; // 0x220
        uint256 wires_evals_3; // 0x240
        uint256 wires_evals_4; // 0x260
        uint256 wires_evals_5; // 0x280
        uint256 wires_evals_6; // 0x2a0
        uint256 wire_sigma_evals_1; // 0x2c0
        uint256 wire_sigma_evals_2; // 0x2e0
        uint256 wire_sigma_evals_3; // 0x300
        uint256 wire_sigma_evals_4; // 0x320
        uint256 wire_sigma_evals_5; // 0x340
        uint256 perm_next_eval; // 0x360
        uint256 range_table_eval; // 0x380
        uint256 key_table_eval; // 0x3a0
        uint256 table_dom_sep_eval; // 0x3c0
        uint256 q_dom_sep_eval; // 0x3e0
        uint256 h_1_eval; // 0x400
        uint256 q_lookup_eval; // 0x420
        uint256 prod_next_eval; // 0x440
        uint256 range_table_next_eval; // 0x460
        uint256 key_table_next_eval; // 0x480
        uint256 table_dom_sep_next_eval; // 0x4a0
        uint256 h_1_next_eval; // 0x4c0
        uint256 h_2_next_eval; // 0x4e0
        uint256 q_lookup_next_eval; // 0x500
        uint256 w_3_next_eval; // 0x520
        uint256 w_4_next_eval; // 0x540
        G1Point opening_proof; // 0x560
        G1Point shifted_opening_proof; // 0x580
    }

    /// @dev Plonk IOP verifier challenges.
    struct ChallengeTranscript {
        uint256 tau; // 0x00
        uint256 beta; // 0x20
        uint256 gamma; // 0x40
        uint256 alpha; // 0x60
        uint256 zeta; // 0x80
        uint256 v; // 0xa0
        uint256 u; // 0xc0
        uint256 alpha2; // 0xe0
        uint256[5] alpha_powers; // 0x100
        uint256 alpha7; // 0x100
        uint256 alpha_base;// 0x120
    }
    

    // function compress_g1_point(
    //     G1Point memory point
    // ) internal pure returns (bytes memory) {
    //     uint256 flag = 0;

    //     if (!isYNegative(point)) {
    //         flag = 0x8000000000000000000000000000000000000000000000000000000000000000;
    //     }

    //     return abi.encodePacked(Transcript.reverse_Endianness(point.x | flag));
    // }

    // function G1PointCompress(
    //     G1Point memory point
    // ) internal pure returns (bytes memory) {
    //     uint256 flag = 0;

    //     if (point.x == 0 && point.y == 0) {
    //         flag |= 0x4000000000000000000000000000000000000000000000000000000000000000;
    //     }

    //     if (!isYNegative(point)) {
    //         flag = 0x8000000000000000000000000000000000000000000000000000000000000000;
    //     }

    //     return abi.encodePacked(Transcript.reverse_Endianness(point.x | flag));
    // }

    // function isYNegative(G1Point memory point) internal pure returns (bool) {
    //     return (point.y << 1) < Bn254Crypto.p_mod;
    // }

    struct G1Point {
        uint256 x;
        uint256 y;
    }



    struct VerificationKey {
        uint256 domain_size; //0x00
        uint256 num_inputs; //0x20
        // extended perm (sigma) poly commitment
        G1Point sigma_comms_1; //0x40
        G1Point sigma_comms_2; //0x60
        G1Point sigma_comms_3; //0x80
        G1Point sigma_comms_4; //0xa0
        G1Point sigma_comms_5; //0xc0
        G1Point sigma_comms_6; // 0xe0
        //  linear combination selector q1-q4
        G1Point selector_comms_1; //0x100
        G1Point selector_comms_2; //0x120
        G1Point selector_comms_3; //0x140
        G1Point selector_comms_4; //0x160
        // multiplication selector for the first and the second wire
        // qM12
        G1Point selector_comms_5; //0x180
        // multiplication selector for the third and the fourth wire
        // qM34
        G1Point selector_comms_6; //0x1a0
        // output selector
        G1Point selector_comms_7; //0x1c0
        // constant term selector
        G1Point selector_comms_8; //0x1e0
        // rescue selector qH1 * w_ai^5
        G1Point selector_comms_9; //0x200
        // rescue selector qH2 * w_bi^5
        G1Point selector_comms_10; //0x220
        // rescue selector qH3 * w_ci^5
        G1Point selector_comms_11; //0x240
        // rescue selector qH4 * w_di^5
        G1Point selector_comms_12; //0x260
        // elliptic curve selector
        G1Point selector_comms_13; //0x280
        G1Point selector_comms_14; //0x2a0
        G1Point selector_comms_15;  //0x2c0
        G1Point selector_comms_16; //0x2e0
        G1Point selector_comms_17; //0x300
        G1Point selector_comms_18; //0x320


        // coset representatives
        // wire types == 5
        uint256 k1; //0x340
        uint256 k2; //0x360
        uint256 k3; //0x380
        uint256 k4; //0x3a0
        uint256 k5; //0x3c0
        uint256 k6; //0x3e0
        /// PlookupVerifyingKey<E>>
        //range_table_comm 
        G1Point range_table_comm; //0x400
        //key_table_comm
        G1Point key_table_comm; //0x420
        //table_dom_sep_comm
        G1Point table_dom_sep_comm; //0x440
        // q_dom_sep_comm
        G1Point q_dom_sep_comm; //0x460
        // The followings are for EvalDomain
        // sizeInv
        uint256 size_inv; //0x480
        // groupGen
        uint256 group_gen; //0x4a0
        // groupGenInv
        uint256 group_gen_inv; //0x4c0
    }
}