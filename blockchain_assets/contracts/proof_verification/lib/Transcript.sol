// SPDX-License-Identifier: GPL-2.0-only
pragma solidity ^0.8.20;

import "../lib/Types.sol";
import "../lib/BytesLib.sol";
import "./Bn254Crypto.sol";

library Transcript {
    struct TranscriptData {
        bytes32[2] state;
        bytes transcript;
        Types.ChallengeTranscript challenges;
    }

    function append_G1_element(
        TranscriptData memory self,
        Types.G1Point memory point
    ) internal pure {
        append_field_element(self, point.x);
        append_field_element(self, point.y);
    }

    function append_field_element(
        TranscriptData memory self,
        uint256 fieldElement
    ) internal pure {
        appendMessage(self, abi.encodePacked(fieldElement));
    }

    function appendMessage(
        TranscriptData memory self,
        bytes memory message
    ) internal pure {
        self.transcript = abi.encodePacked(self.transcript, message);
    }

    function reverse_Endianness(
        uint256 input
    ) internal pure returns (uint256 v) {
        v = input;
        v =
            ((v &
                0xFF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00) >>
                8) |
            ((v &
                0x00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF) <<
                8);
        v =
            ((v &
                0xFFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000) >>
                16) |
            ((v &
                0x0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF) <<
                16);
        v =
            ((v &
                0xFFFFFFFF00000000FFFFFFFF00000000FFFFFFFF00000000FFFFFFFF00000000) >>
                32) |
            ((v &
                0x00000000FFFFFFFF00000000FFFFFFFF00000000FFFFFFFF00000000FFFFFFFF) <<
                32);
        v =
            ((v &
                0xFFFFFFFFFFFFFFFF0000000000000000FFFFFFFFFFFFFFFF0000000000000000) >>
                64) |
            ((v &
                0x0000000000000000FFFFFFFFFFFFFFFF0000000000000000FFFFFFFFFFFFFFFF) <<
                64);
        v = (v >> 128) | (v << 128);
    }

    function compute_vk_hash(
        Types.VerificationKey memory vk
    ) internal pure returns (uint256 vk_hash) {
        // vk_hash = keccak256(
        //    vk.domain_size,
        //    vk.sigma_comms_1..6,
        //    vk.selector_comms_1..18,
        //    vk.k1..k6,
        //    vk.range_table_comm,
        //    vk.key_table_comm,
        //    vk.table_dom_sep_comm,
        //    vk.q_dom_sep_comm
        // ) mod p

        assembly {
            // Free memory pointer & write cursor
            let ptr := mload(0x40)
            let offset := ptr
            // 1) domain_size (uint256 at vk + 0x00)
            {
                let ds := mload(vk) // uint256 in Solidity
                // write bytes 24..31 (most-significant to least-significant)
                mstore8(offset, byte(24, ds))
                mstore8(add(offset, 1), byte(25, ds))
                mstore8(add(offset, 2), byte(26, ds))
                mstore8(add(offset, 3), byte(27, ds))
                mstore8(add(offset, 4), byte(28, ds))
                mstore8(add(offset, 5), byte(29, ds))
                mstore8(add(offset, 6), byte(30, ds))
                mstore8(add(offset, 7), byte(31, ds))
                offset := add(offset, 8)
            }

            // 2) sigma_comms_1..6
            // Each field in parent struct holds a pointer to a G1Point (x,y).
            // G1Point memory layout: [x (32 bytes), y (32 bytes)]
            // Parent VK layout offsets: 0x40, 0x60, 0x80, 0xa0, 0xc0, 0xe0
            {
                let base := 0x40
                for {
                    let i := 0
                } lt(i, 6) {
                    i := add(i, 1)
                } {
                    let g1ptr := mload(add(vk, add(base, mul(i, 0x20))))
                    mstore(offset, mload(g1ptr)) // x
                    mstore(add(offset, 0x20), mload(add(g1ptr, 0x20))) // y
                    offset := add(offset, 0x40)
                }
            }

            // 3) selector_comms_1..18
            // Parent VK layout starts at 0x100, step 0x20 for each pointer
            {
                let base := 0x100
                for {
                    let i := 0
                } lt(i, 18) {
                    i := add(i, 1)
                } {
                    let g1ptr := mload(add(vk, add(base, mul(i, 0x20))))
                    mstore(offset, mload(g1ptr)) // x
                    mstore(add(offset, 0x20), mload(add(g1ptr, 0x20))) // y
                    offset := add(offset, 0x40)
                }
            }

            // 4) ks: k1..k6 (uint256s)
            // Parent VK layout: 0x340, 0x360, 0x380, 0x3a0, 0x3c0, 0x3e0
            {
                let base := 0x340
                for {
                    let i := 0
                } lt(i, 6) {
                    i := add(i, 1)
                } {
                    mstore(offset, mload(add(vk, add(base, mul(i, 0x20)))))
                    offset := add(offset, 0x20)
                }
            }

            // 5) range_table_comm, key_table_comm, table_dom_sep_comm, q_dom_sep_comm
            // Parent VK layout: pointers at 0x400, 0x420, 0x440, 0x460
            {
                let base := 0x400
                for {
                    let i := 0
                } lt(i, 4) {
                    i := add(i, 1)
                } {
                    let g1ptr := mload(add(vk, add(base, mul(i, 0x20))))
                    mstore(offset, mload(g1ptr)) // x
                    mstore(add(offset, 0x20), mload(add(g1ptr, 0x20))) // y
                    offset := add(offset, 0x40)
                }
            }

            // Hash the contiguous region [ptr .. offset)
            let len := sub(offset, ptr)
            let h := keccak256(ptr, len)

            // advance free memory pointer
            mstore(0x40, add(ptr, len))

            // Reduce mod BN254 scalar field prime
            // p = 21888242871839275222246405745257275088548364400416034343698204186575808495617
            vk_hash := h
        }
    }

    function _beU32(uint256 x) internal pure returns (bytes memory out) {
        out = new bytes(4);
        assembly {
            mstore(add(out, 32), shl(224, x))
        } // write 4 BE bytes
    }

    // TLV encoding for the metadata hased in the transcript initialisation message.
    // We prepend the label and values by their length for parsing.
    function _tlv(
        string memory label,
        bytes memory value
    ) internal pure returns (bytes memory) {
        bytes memory lab = bytes(label);
        return
            abi.encodePacked(
                _beU32(lab.length),
                lab,
                _beU32(value.length),
                value
            );
    }

    function ilog2(uint256 x) internal pure returns (uint32 r) {
        require(x > 0, "ilog2(0)");
        while (x > 1) {
            x >>= 1;
            r++;
        }
    }

    function decider_depth(
        uint256 prove_inputs
    ) internal pure returns (uint32) {
        // Require power-of-two batch sizes (64, 256, ...)
        require(
            prove_inputs > 1 && (prove_inputs & (prove_inputs - 1)) == 0,
            "non pow2"
        );
        uint32 d = ilog2(prove_inputs);
        require(d > 0, "depth underflow");
        return d - 1;
    }

    struct ChallengeInputs {
        Types.VerificationKey vk;
        Types.Proof proof;
        uint256 public_inputs_hash;
        bytes32 srs_digest;
        uint256 rollup_tx_batch_size;
        string version;
    }

    function compute_challengs(
        TranscriptData memory self,
        ChallengeInputs memory challenges
    ) internal pure {
        uint256 vk_hash = compute_vk_hash(challenges.vk);

        bytes memory hdr = _tlv("app_id", bytes("nightfish.pcd"));
        hdr = abi.encodePacked(hdr, _tlv("proto", bytes("plonk-recursion")));
        hdr = abi.encodePacked(hdr, _tlv("role", bytes("rollup_prover")));
        hdr = abi.encodePacked(hdr, _tlv("layer", bytes("decider")));
        hdr = abi.encodePacked(
            hdr,
            _tlv("vk_digest", abi.encodePacked(vk_hash))
        );
        hdr = abi.encodePacked(
            hdr,
            _tlv("srs_digest", abi.encodePacked(challenges.srs_digest))
        );
        hdr = abi.encodePacked(
            hdr,
            _tlv(
                "recursion_depth",
                _beU32(uint256(decider_depth(challenges.rollup_tx_batch_size)))
            )
        );
        hdr = abi.encodePacked(
            hdr,
            _tlv("rollup_size", _beU32(challenges.rollup_tx_batch_size))
        );

        appendMessage(self, hdr);

        append_field_element(self, challenges.public_inputs_hash);

        append_G1_element(self, challenges.proof.wires_poly_comms_1);
        append_G1_element(self, challenges.proof.wires_poly_comms_2);
        append_G1_element(self, challenges.proof.wires_poly_comms_3);
        append_G1_element(self, challenges.proof.wires_poly_comms_4);
        append_G1_element(self, challenges.proof.wires_poly_comms_5);
        append_G1_element(self, challenges.proof.wires_poly_comms_6);

        generate_tau_challenge(self);
        generate_beta_challenges(self, challenges.proof);
        self.challenges.gamma = generate_gamma_challenges(self);
        self.challenges.alpha = generate_alpha_challenges(
            self,
            challenges.proof
        );
        self.challenges.zeta = generate_zeta_challenges(self, challenges.proof);
        self.challenges.v = generate_v_challenges(self, challenges.proof);
        self.challenges.u = generate_u_challenges(self, challenges.proof);
    }

    function generate_initial_transcript(
        TranscriptData memory self,
        Types.VerificationKey memory vk
    ) internal pure {
        bytes memory transcript_con;
        assembly {
            let ptr := mload(0x40)
            mstore(ptr, 80)
            mstore(add(ptr, 36), 0)
            mstore(add(ptr, 68), 0)
            let vk_ptr := vk
            mstore8(add(ptr, 96), mload(vk_ptr))
            mstore8(add(ptr, 97), shr(8, mload(vk_ptr)))
            mstore8(add(ptr, 98), shr(16, mload(vk_ptr)))
            mstore8(add(ptr, 99), shr(24, mload(vk_ptr)))
            mstore8(add(ptr, 100), shr(32, mload(vk_ptr)))
            mstore8(add(ptr, 101), shr(40, mload(vk_ptr)))
            mstore8(add(ptr, 102), shr(48, mload(vk_ptr)))
            mstore8(add(ptr, 103), shr(56, mload(vk_ptr)))
            mstore8(add(ptr, 104), mload(add(vk_ptr, 0x20)))
            mstore8(add(ptr, 105), shr(8, mload(add(vk_ptr, 0x20))))
            mstore8(add(ptr, 106), shr(16, mload(add(vk_ptr, 0x20))))
            mstore8(add(ptr, 107), shr(24, mload(add(vk_ptr, 0x20))))
            mstore8(add(ptr, 108), shr(32, mload(add(vk_ptr, 0x20))))
            mstore8(add(ptr, 109), shr(40, mload(add(vk_ptr, 0x20))))
            mstore8(add(ptr, 110), shr(48, mload(add(vk_ptr, 0x20))))
            mstore(0x40, add(ptr, 112))
            mstore8(add(ptr, 111), shr(56, mload(add(vk_ptr, 0x20))))
            transcript_con := ptr
        }
        self.transcript = transcript_con;
    }

    function append_K1_K6(
        TranscriptData memory self,
        Types.VerificationKey memory vk
    ) internal pure {
        append_field_element(self, vk.k1);
        append_field_element(self, vk.k2);
        append_field_element(self, vk.k3);
        append_field_element(self, vk.k4);
        append_field_element(self, vk.k5);
        append_field_element(self, vk.k6);
    }

    function append_selector_comms_1_18(
        TranscriptData memory self,
        Types.VerificationKey memory vk
    ) internal pure {
        append_G1_element(self, vk.selector_comms_1);
        append_G1_element(self, vk.selector_comms_2);
        append_G1_element(self, vk.selector_comms_3);
        append_G1_element(self, vk.selector_comms_4);
        append_G1_element(self, vk.selector_comms_5);
        append_G1_element(self, vk.selector_comms_6);
        append_G1_element(self, vk.selector_comms_7);
        append_G1_element(self, vk.selector_comms_8);
        append_G1_element(self, vk.selector_comms_9);
        append_G1_element(self, vk.selector_comms_10);
        append_G1_element(self, vk.selector_comms_11);
        append_G1_element(self, vk.selector_comms_12);
        append_G1_element(self, vk.selector_comms_13);
        append_G1_element(self, vk.selector_comms_14);
        append_G1_element(self, vk.selector_comms_15);
        append_G1_element(self, vk.selector_comms_16);
        append_G1_element(self, vk.selector_comms_17);
        append_G1_element(self, vk.selector_comms_18);
    }

    function append_sigma_comms_1_6(
        TranscriptData memory self,
        Types.VerificationKey memory vk
    ) internal pure {
        append_G1_element(self, vk.sigma_comms_1);
        append_G1_element(self, vk.sigma_comms_2);
        append_G1_element(self, vk.sigma_comms_3);
        append_G1_element(self, vk.sigma_comms_4);
        append_G1_element(self, vk.sigma_comms_5);
        append_G1_element(self, vk.sigma_comms_6);
    }

    function append_public_inputs(
        TranscriptData memory self,
        Types.VerificationKey memory vk
    ) internal pure {
        bytes memory transcript_con;
        assembly {
            let ptr := mload(0x40)
            let vk_ptr := vk
            let public_input_byte_length := mul(mload(add(vk_ptr, 0x20)), 0x20)
            mstore(ptr, public_input_byte_length)
            let total := add(public_input_byte_length, 32)
            mstore(0x40, add(ptr, total))
            calldatacopy(add(ptr, 32), 68, public_input_byte_length)
            for {
                let i := 0
            } lt(i, public_input_byte_length) {
                i := add(i, 32)
            } {
                let a := add(ptr, add(32, i))
                let b := add(a, 31)
                for {
                    let j := 0
                } lt(j, 16) {
                    j := add(j, 1)
                } {
                    let tmp := mload(a)
                    mstore(a, mload(b))
                    mstore(b, tmp)
                    a := add(a, 1)
                    b := sub(b, 1)
                }
            }
            transcript_con := ptr
        }
        appendMessage(self, transcript_con);
    }

    function append_wires_poly_comms_1_6(
        TranscriptData memory self,
        Types.Proof memory proof
    ) internal pure {
        append_field_element(self, proof.wires_poly_comms_1.x);
        append_field_element(self, proof.wires_poly_comms_1.y);
        append_field_element(self, proof.wires_poly_comms_2.x);
        append_field_element(self, proof.wires_poly_comms_2.y);
        append_field_element(self, proof.wires_poly_comms_3.x);
        append_field_element(self, proof.wires_poly_comms_3.y);
        append_field_element(self, proof.wires_poly_comms_4.x);
        append_field_element(self, proof.wires_poly_comms_4.y);
        append_field_element(self, proof.wires_poly_comms_5.x);
        append_field_element(self, proof.wires_poly_comms_5.y);
        append_field_element(self, proof.wires_poly_comms_6.x);
        append_field_element(self, proof.wires_poly_comms_6.y);
    }

    function generate_tau_challenge(TranscriptData memory self) internal pure {
        bytes memory input = abi.encodePacked(self.state[0], self.transcript);
        bytes32 buf = keccak256(input);
        self.state[0] = buf;
        self.transcript = "";
        self.challenges.tau = Bn254Crypto.fromBeBytesModOrder(
            BytesLib.slice(abi.encodePacked(buf), 0, 32)
        );
    }

    function removeLeadingZeros(
        bytes memory x
    ) internal pure returns (bytes memory x1) {
        uint256 i;
        uint256 length = x.length;
        assembly {
            let leadingZeros := 0
            for {} lt(i, length) {
                i := add(i, 32)
            } {
                leadingZeros := add(leadingZeros, iszero(mload(add(x, i))))
            }
            x1 := mload(0x40)
            mstore(x1, sub(length, leadingZeros))
            for {} lt(i, length) {
                i := add(i, 32)
            } {
                mstore(add(x1, add(32, i)), mload(add(x, add(32, i))))
            }
            mstore(x1, add(32, mload(x1)))
        }
    }

    function slice_from_index(
        bytes memory data,
        uint256 index
    ) internal pure returns (bytes memory) {
        require(index < data.length, "Invalid index");
        uint256 length = data.length - index;
        bytes memory result = new bytes(length);
        assembly {
            let src := add(data, 0x20)
            let dst := add(result, 0x20)
            for {
                let i := 0
            } lt(i, length) {
                i := add(i, 1)
            } {
                let temp := mload(add(src, add(i, index)))
                mstore(add(dst, i), temp)
            }
        }
        return result;
    }

    function generate_beta_challenges(
        TranscriptData memory self,
        Types.Proof memory proof
    ) internal pure {
        append_G1_element(self, proof.h_poly_comm_1);
        append_G1_element(self, proof.h_poly_comm_2);
        bytes memory input = abi.encodePacked(self.state[0], self.transcript);
        bytes32 buf = keccak256(input);
        self.state[0] = buf;
        self.transcript = "";
        self.challenges.beta = Bn254Crypto.fromBeBytesModOrder(
            BytesLib.slice(abi.encodePacked(buf), 0, 32)
        );
    }

    function generate_gamma_challenges(
        TranscriptData memory self
    ) internal pure returns (uint256) {
        bytes memory input = abi.encodePacked(self.state[0], self.transcript);
        bytes32 buf = keccak256(input);
        self.state[0] = buf;
        self.transcript = "";
        return
            Bn254Crypto.fromBeBytesModOrder(
                BytesLib.slice(abi.encodePacked(buf), 0, 32)
            );
    }

    function generate_alpha_challenges(
        TranscriptData memory self,
        Types.Proof memory proof
    ) internal pure returns (uint256) {
        append_G1_element(self, proof.prod_perm_poly_comm);
        append_G1_element(self, proof.prod_lookup_poly_comm);
        bytes memory input = abi.encodePacked(self.state[0], self.transcript);
        bytes32 buf = keccak256(input);
        self.state[0] = buf;
        self.transcript = "";
        return
            Bn254Crypto.fromBeBytesModOrder(
                BytesLib.slice(abi.encodePacked(buf), 0, 32)
            );
    }

    function generate_zeta_challenges(
        TranscriptData memory self,
        Types.Proof memory proof
    ) internal pure returns (uint256) {
        append_G1_element(self, proof.split_quot_poly_comms_1);
        append_G1_element(self, proof.split_quot_poly_comms_2);
        append_G1_element(self, proof.split_quot_poly_comms_3);
        append_G1_element(self, proof.split_quot_poly_comms_4);
        append_G1_element(self, proof.split_quot_poly_comms_5);
        append_G1_element(self, proof.split_quot_poly_comms_6);
        bytes memory input = abi.encodePacked(self.state[0], self.transcript);
        bytes32 buf = keccak256(input);
        self.state[0] = buf;
        self.transcript = "";
        return
            Bn254Crypto.fromBeBytesModOrder(
                BytesLib.slice(abi.encodePacked(buf), 0, 32)
            );
    }

    function generate_v_challenges(
        TranscriptData memory self,
        Types.Proof memory proof
    ) internal pure returns (uint256) {
        append_field_element(self, proof.wires_evals_1);
        append_field_element(self, proof.wires_evals_2);
        append_field_element(self, proof.wires_evals_3);
        append_field_element(self, proof.wires_evals_4);
        append_field_element(self, proof.wires_evals_5);
        append_field_element(self, proof.wires_evals_6);
        append_field_element(self, proof.wire_sigma_evals_1);
        append_field_element(self, proof.wire_sigma_evals_2);
        append_field_element(self, proof.wire_sigma_evals_3);
        append_field_element(self, proof.wire_sigma_evals_4);
        append_field_element(self, proof.wire_sigma_evals_5);
        append_field_element(self, proof.perm_next_eval);

        append_field_element(self, proof.key_table_eval);
        append_field_element(self, proof.table_dom_sep_eval);
        append_field_element(self, proof.range_table_eval);
        append_field_element(self, proof.q_dom_sep_eval);
        append_field_element(self, proof.h_1_eval);
        append_field_element(self, proof.q_lookup_eval);
        append_field_element(self, proof.prod_next_eval);
        append_field_element(self, proof.range_table_next_eval);
        append_field_element(self, proof.key_table_next_eval);
        append_field_element(self, proof.table_dom_sep_next_eval);
        append_field_element(self, proof.h_1_next_eval);
        append_field_element(self, proof.h_2_next_eval);
        append_field_element(self, proof.q_lookup_next_eval);
        append_field_element(self, proof.w_3_next_eval);
        append_field_element(self, proof.w_4_next_eval);

        bytes memory input = abi.encodePacked(self.state[0], self.transcript);
        bytes32 buf = keccak256(input);
        self.state[0] = buf;
        self.transcript = "";
        return
            Bn254Crypto.fromBeBytesModOrder(
                BytesLib.slice(abi.encodePacked(buf), 0, 32)
            );
    }

    function generate_u_challenges(
        TranscriptData memory self,
        Types.Proof memory proof
    ) internal pure returns (uint256) {
        append_G1_element(self, proof.opening_proof);
        append_G1_element(self, proof.shifted_opening_proof);
        bytes memory input = abi.encodePacked(self.state[0], self.transcript);
        bytes32 buf = keccak256(input);
        self.state[0] = buf;
        self.transcript = "";
        return
            Bn254Crypto.fromBeBytesModOrder(
                BytesLib.slice(abi.encodePacked(buf), 0, 32)
            );
    }
}
