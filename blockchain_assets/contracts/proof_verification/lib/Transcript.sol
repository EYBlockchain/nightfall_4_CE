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
                0xFF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00) >> 8) |
            ((v &
                0x00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF) << 8);
        v =
            ((v &
                0xFFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000) >> 16) |
            ((v &
                0x0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF) << 16);
        v =
            ((v &
                0xFFFFFFFF00000000FFFFFFFF00000000FFFFFFFF00000000FFFFFFFF00000000) >> 32) |
            ((v &
                0x00000000FFFFFFFF00000000FFFFFFFF00000000FFFFFFFF00000000FFFFFFFF) << 32);
        v =
            ((v &
                0xFFFFFFFFFFFFFFFF0000000000000000FFFFFFFFFFFFFFFF0000000000000000) >> 64) |
            ((v &
                0x0000000000000000FFFFFFFFFFFFFFFF0000000000000000FFFFFFFFFFFFFFFF) << 64);
        v = (v >> 128) | (v << 128);
    }

    function compute_vk_hash(
        Types.VerificationKey memory vk
    ) internal pure returns (uint256 vk_hash) {
        assembly {
            let ptr := mload(0x40)
            let offset := ptr

            // sigma_comms_1..6 (offsets start at 0x40 in your VK layout)
            for { let i := 0 } lt(i, 6) { i := add(i, 1) } {
                let g1ptr := mload(add(vk, add(0x40, mul(i, 0x20))))
                mstore(offset, mload(g1ptr))
                mstore(add(offset, 0x20), mload(add(g1ptr, 0x20)))
                offset := add(offset, 0x40)
            }

            // selector_comms_1..18 (start 0x100)
            for { let i := 0 } lt(i, 18) { i := add(i, 1) } {
                let g1ptr := mload(add(vk, add(0x100, mul(i, 0x20))))
                mstore(offset, mload(g1ptr))
                mstore(add(offset, 0x20), mload(add(g1ptr, 0x20)))
                offset := add(offset, 0x40)
            }

            // range_table_comm, key_table_comm, table_dom_sep_comm, q_dom_sep_comm (start 0x400)
            for { let i := 0 } lt(i, 4) { i := add(i, 1) } {
                let g1ptr := mload(add(vk, add(0x400, mul(i, 0x20))))
                mstore(offset, mload(g1ptr))
                mstore(add(offset, 0x20), mload(add(g1ptr, 0x20)))
                offset := add(offset, 0x40)
            }

            let len := sub(offset, ptr)
            vk_hash := shr(0, keccak256(ptr, len))
            vk_hash := mod(vk_hash, 21888242871839275222246405745257275088548364400416034343698204186575808495617)
        }
    }

    function compute_challengs(
        TranscriptData memory self,
        Types.VerificationKey memory vk,
        Types.Proof memory proof,
        uint256 public_inputs_hash
    ) internal pure {
        uint256 vk_hash = compute_vk_hash(vk);
        append_field_element(self, vk_hash);
        append_field_element(self, public_inputs_hash);

        append_G1_element(self, proof.wires_poly_comms_1);
        append_G1_element(self, proof.wires_poly_comms_2);
        append_G1_element(self, proof.wires_poly_comms_3);
        append_G1_element(self, proof.wires_poly_comms_4);
        append_G1_element(self, proof.wires_poly_comms_5);
        append_G1_element(self, proof.wires_poly_comms_6);

        generate_tau_challenge(self);
        generate_beta_challenges(self, proof);
        self.challenges.gamma = generate_gamma_challenges(self);
        self.challenges.alpha = generate_alpha_challenges(self, proof);
        self.challenges.zeta = generate_zeta_challenges(self, proof);
        self.challenges.v = generate_v_challenges(self, proof);
        self.challenges.u = generate_u_challenges(self, proof);
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

    function append_K1_K6(TranscriptData memory self, Types.VerificationKey memory vk) internal pure {
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
            for { let i := 0 } lt(i, public_input_byte_length) { i := add(i, 32) } {
                let a := add(ptr, add(32, i))
                let b := add(a, 31)
                for { let j := 0 } lt(j, 16) { j := add(j, 1) } {
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

    function removeLeadingZeros(bytes memory x) internal pure returns (bytes memory x1) {
        uint256 i; uint256 length = x.length;
        assembly {
            let leadingZeros := 0
            for { } lt(i, length) { i := add(i, 32) } {
                leadingZeros := add(leadingZeros, iszero(mload(add(x, i))))
            }
            x1 := mload(0x40)
            mstore(x1, sub(length, leadingZeros))
            for { } lt(i, length) { i := add(i, 32) } {
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
            for { let i := 0 } lt(i, length) { i := add(i, 1) } {
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
        return Bn254Crypto.fromBeBytesModOrder(BytesLib.slice(abi.encodePacked(buf), 0, 32));
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
        return Bn254Crypto.fromBeBytesModOrder(BytesLib.slice(abi.encodePacked(buf), 0, 32));
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
        return Bn254Crypto.fromBeBytesModOrder(BytesLib.slice(abi.encodePacked(buf), 0, 32));
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
        return Bn254Crypto.fromBeBytesModOrder(BytesLib.slice(abi.encodePacked(buf), 0, 32));
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
        return Bn254Crypto.fromBeBytesModOrder(BytesLib.slice(abi.encodePacked(buf), 0, 32));
    }
}
