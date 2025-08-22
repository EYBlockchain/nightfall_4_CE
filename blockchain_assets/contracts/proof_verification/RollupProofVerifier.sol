// SPDX-License-Identifier: GPL-2.0-only

pragma solidity >=0.6.0;
pragma experimental ABIEncoderV2;
import "./BytesLib.sol";
import "./Types.sol";
import "./RollupProofVerificationKey.sol";
import {INFVerifier} from "./INFVerifier.sol";
import "forge-std/console.sol";


/**
@title RollupProofVerifier
@dev Verifier Implementation for Nightfish Ultra plonk proof verification
*/

contract RollupProofVerifier is INFVerifier{
    /**
        Calldata formatting:
        0x00 - 0x04 : function signature
        0x04 - 0x24 : proof_data pointer (location in calldata that contains the proof_data array)
        0x44 - 0x64 : length of `proof_data` array
        0x64 - ???? : array containing our zk proof data
    **/

    // Define a global p here for the mod operation
    // Jiajie: You can find that when we use line assembly code in a function,
    // I define a p again, this is because line assembly code can only get local parameter
    uint256 public p;
    constructor() {
        p = Bn254Crypto.r_mod;
    }

    // A struct for compute_buffer_v_and_uv_basis_2() input parameters to avoid stack too deep error
    // compute_buffer_v_and_uv_basis() is devided into two functions to avoid stack too deep error
    struct compute_buffer_v_and_uv_basis_2_parameters {
        uint256[] buffer_v_and_uv_basis;
        uint256 start_index;
        Types.VerificationKey verifyingKey;
        Types.ChallengeTranscript chal;
        uint256[] commScalars;
        Types.G1Point[] commBases;
        uint256 v_base;
    }

 struct compute_buffer_v_and_uv_basis_3_parameters {
         Types.ChallengeTranscript  chal;
        Types.VerificationKey  vk;
        Types.Proof proof;
        uint256 start_index;
        uint256[]  buffer_v_and_uv_basis;
        uint256 v_base;
        uint256 uv_base;
        uint256[] commScalars;
        Types.G1Point[]  commBases;
    }
    // A struct for add_splitted_quotient_commitments() input parameters to avoid stack too deep error
    struct add_splitted_quotient_commitments_parameter {
        uint256 index;
        uint256 challenge_zeta;
        uint256 evalData_vanish_eval;
        Types.G1Point[] bases;
        uint256[] scalars;
        Types.Proof proof;
    }

    // A struct for add_selector_polynomial_commitments() input parameters to avoid stack too deep error
    struct add_selector_polynomial_commitments_parameters {
        Types.VerificationKey verifyingKey;
        Types.G1Point[] bases;
        uint256[] scalars;
        Types.Proof proof;
    }

    // Define beta*G2 generator
    // This value is from Jellyfish proof
    // open_key.beta_h:(QuadExtField(
    // 13391535935907980906767851257946662220338053336387334424319244660799129107115 +
    // 13486697983632270454518154085647451274272822224810404073606160143201764912100 * u),
    // QuadExtField(19730215451073055081484308140724634066486551345743718054014672266688134390417 +
    // 14831435711192949190034188156049387261611890338548161586954205995029247307236 * u))
    // Swap the x0 and x1, y0 and y1
    // Becase:
    // In ethereum precompile paring check smart contract (https://github.com/ethereum/EIPs/blob/master/EIPS/eip-197.md)
    // we have
    // P2 = (
    //   11559732032986387107991004021392285783925812861821192530917403151452391805634 * i +
    //   10857046999023057135944570762232829481370756359578518086990519993285655852781,
    //   4082367875863433681332203403145435568316851327593401208105741076214120093531 * i +
    //   8495653923123431417604973247489272438418190587263600148770280649306958101930
    // )
    // In Jellyfish, we have
    // open_key.h:
    // (QuadExtField(18992883557077338751676740118177043722015109780430174049283693347921085315405 +
    // 13894752563669177437972774094368710232791813444255892635223873293938781475590 * u),
    // QuadExtField(10925122867104977337323190443464523418359723997008819246567631783324624443255 +
    // 18962925891889930889570493569926001403492439380023366507779121004511310287959 * u))

    // Jellyfish doesn't use the original generator
    // they used random value during the SRS generating and set the generator as follows:
    // let g = E::G1::rand(rng);
    // let h = E::G2::rand(rng);
    // This value needs to be changed for different proofs.
    // Types.G2Point private beta_h =
    //     Types.G2Point({
    //         // x0: 0x1DD13357222EAB4FB810D5C89B5AF426816CD0492532F7F181BB44E39CBC2BE4,
    //         // x1: 0x1D9B573A9B30EAD10DCF030D1AB3C9EC81DC3DA2AAC764597280370A6B29BAAB,
    //         // y0: 0x20CA4B8DA283890EA4AB8AC17F07102E0E3BCD102998E3BB16349B6005B02DE4,
    //         // y1: 0x2B9EE7FD0E19D5EC504255B3090E52AB453425E7B43C170022F6F862F7CC2291
    //         // x0: 0x285B1F14EDD7E6632340A37DFAE9005FF762EDCFECFE1C732A7474C0708BEF80,
    //         // x1: 0x17CC93077F56F654DA727C1DEF86010339C2B4131094547285ADB083E48C197B,
    //         // y0: 0x2BAD9A374AEC49D329EC66E8F530F68509313450580C4C17C6DB5DDB9BDE7FD0,
    //         // y1: 0x219EDFCEEE1723DE674F5B2F6FDB69D9E32DD53B15844956A630D3C7CDAA6ED9
    //         x0: 0x17CC93077F56F654DA727C1DEF86010339C2B4131094547285ADB083E48C197B,
    //         x1: 0x285B1F14EDD7E6632340A37DFAE9005FF762EDCFECFE1C732A7474C0708BEF80,
    //         y0: 0x219EDFCEEE1723DE674F5B2F6FDB69D9E32DD53B15844956A630D3C7CDAA6ED9,
    //         y1: 0x2BAD9A374AEC49D329EC66E8F530F68509313450580C4C17C6DB5DDB9BDE7FD0
    //     });

    /**
     * @dev Verify a UltraPlonk proof from Jellyfish with 4 input wires
     * @param proofBytes- array of serialized proof data: every elements is 32 bytes
     * @param publicInputsHashBytes- bytes of public data
     */
function verify(bytes calldata acc_proof, bytes calldata proofBytes, bytes calldata publicInputsHashBytes) external view override returns (bool result) {
        // parse the hardecoded vk and construct a vk object
        Types.VerificationKey memory vk = get_verification_key();
        // parse the second part of calldata to get public input
        // we hashed all public inputs into a single value
        uint256 public_inputs_hash;
        assembly {
            public_inputs_hash := calldataload(add(publicInputsHashBytes.offset, 0))
        }

        // parse the input calldata and construct a proof object and public_inputs
        Types.Proof memory decoded_proof
        = deserialize_proof(proofBytes);
        validate_proof(decoded_proof);
        validate_scalar_field(public_inputs_hash);

        // Compute the transcripts by appending vk, public inputs and proof
        // reconstruct the tau, beta, gamma, alpha, zeta, v and u challenges based on the transcripts
        Transcript.TranscriptData memory transcripts;
        Transcript.compute_challengs(transcripts, vk, decoded_proof, public_inputs_hash);
        Types.ChallengeTranscript memory full_challenges = transcripts
            .challenges;

        uint256[] memory public_inputs = new uint256[](vk.num_inputs);
        public_inputs[0] = public_inputs_hash;
        // compute polynomial commitment evaluation info
        Types.PcsInfo memory pcsInfo = prepare_PcsInfo(
            vk,
            public_inputs,
            decoded_proof,
            full_challenges
        );

        return (verify_OpeningProof(full_challenges, pcsInfo, decoded_proof, vk) && verify_accumulation(
            acc_proof,
            vk
        ));
    }

    function verify_accumulation(
        bytes calldata acc_proof,
        Types.VerificationKey memory vk
    ) internal view returns (bool) {
    require(acc_proof.length == 256, "Invalid accumulator proof length");
    bytes32[8] memory acc;
    for (uint i = 0; i < 8; i++) {
        acc[i] = bytes32(acc_proof[i*32:(i+1)*32]);
    }
    //blk.rollup_proof[32:64], accumulator_1_comm_x, acc[0]
    //blk.rollup_proof[64:96], accumulator_1_comm_y, acc[1]
    //blk.rollup_proof[96:128], accumulator_1_proof_x, acc[2]
    //blk.rollup_proof[128:160], accumulator_1_proof_y, acc[3]
    //blk.rollup_proof[160:192], accumulator_2_comm_x, acc[4]
    //blk.rollup_proof[192:224], accumulator_2_comm_y, acc[5]
    //blk.rollup_proof[224:256], accumulator_2_proof2_x, acc[6]
    //blk.rollup_proof[256:288], accumulator_2_proof2_y, acc[7]

    // First accumulator
    bool res_1 = Bn254Crypto.pairingProd2(Types.G1Point(uint256(acc[2]), uint256(acc[3])), vk.beta_h, Bn254Crypto.negate_G1Point(Types.G1Point(uint256(acc[0]), uint256(acc[1]))), vk.h);
    // Second accumulator
    bool res_2 = Bn254Crypto.pairingProd2(Types.G1Point(uint256(acc[6]), uint256(acc[7])), vk.beta_h, Bn254Crypto.negate_G1Point(Types.G1Point(uint256(acc[4]), uint256(acc[5]))), vk.h);
    return (res_1 && res_2);
}

    /**
     * @dev Compute polynomial commitment evaluation info
     * @param - vk: verification key struct
     * @param - publicInput: publicInput array
     * @param - proof: proof struct
     * @param - full_challenges: ChallengeTranscript struct
     * @return - pcsInfo: PcsInfo struct
     */
    function prepare_PcsInfo(
        Types.VerificationKey memory vk,
        uint256[] memory publicInput,
        Types.Proof memory proof,
        Types.ChallengeTranscript memory full_challenges
    ) internal view returns (Types.PcsInfo memory) {
        full_challenges.alpha2 = mulmod(
            full_challenges.alpha,
            full_challenges.alpha,
            p
        );
        uint256 alpha_3 =   mulmod(full_challenges.alpha2, full_challenges.alpha, p);
        uint256 alpha_4 =   mulmod(full_challenges.alpha2, full_challenges.alpha2, p);
        uint256 alpha_5 =   mulmod(full_challenges.alpha2, alpha_3, p);
        uint256 alpha_6 =   mulmod(alpha_4, full_challenges.alpha2, p);
         full_challenges.alpha_powers = [full_challenges.alpha2, alpha_3, alpha_4, alpha_5, alpha_6];
         full_challenges.alpha_base= 1;
         full_challenges.alpha7= mulmod(alpha_3, alpha_4, p);


        // get the domain evaluation information
        // including 2 ^ domainSize, domainSize, sizeInv, groupGen
        // change this, sizeInv, groupGen
        PolynomialEval.EvalDomain memory domain = PolynomialEval.new_EvalDomain(
            vk
        );

        //  pre-compute evaluation data
        //  get vanish_eval, lagrange_1_eval, piEval
        PolynomialEval.EvalData memory evalData = PolynomialEval.evalDataGen(
            domain,
            full_challenges.zeta,
            publicInput
        );
        // compute opening proof in poly comms
        // caller allocates the memory for commScalars and commBases
        uint256[] memory commScalars = new uint256[](58);
        Types.G1Point[] memory commBases = new Types.G1Point[](58);

       
        uint256 eval = prepare_OpeningProof(
            publicInput,
            vk,
            evalData,
            proof,
            full_challenges,
            commScalars,
            commBases,
            domain
        );

        uint256 zeta = full_challenges.zeta;
        uint256 gen = domain.groupGen;
        return (
            Types.PcsInfo(mulmod(zeta, gen, p), eval, commScalars, commBases)
        );
    }

    /**
     * @dev Verify a UltraPlonk proof
     * @param - challenge: A challeng struct
     * @param - pcsInfo: polynomial commitment evaluation info
     * @param - proof: A struct of Plonk proof
     * @return - result: true if the proof is correct
     */
    function verify_OpeningProof(
        Types.ChallengeTranscript memory challenge,
        Types.PcsInfo memory pcsInfo,
        Types.Proof memory proof,
        Types.VerificationKey memory vk
    ) internal view returns (bool) {
        // Compute a pseudorandom challenge from the instances
        Types.G1Point memory A;
        Types.G1Point memory B;
        // A = [open_proof] + u * [shifted_open_proof]
        A = compute_A(proof, challenge);     
        // B = eval_point * open_proof + u * next_eval_point *
        //   shifted_open_proof + comm - eval * [1]1`.
        B = compute_B(pcsInfo, proof, challenge, vk);      

        // Check e(A, [x]2) ?= e(B, [1]2)
        /// By Schwartz-Zippel lemma, it's equivalent to check that for a random r:
        // - `e(A0 + ... + r^{m-1} * Am, [x]2) = e(B0 + ... + r^{m-1} * Bm, [1]2)`.
        return Bn254Crypto.pairingProd2(A, vk.beta_h, B, vk.h);
    }

    function compute_A(
        Types.Proof memory proof,
        Types.ChallengeTranscript memory challenge
    ) internal view returns (Types.G1Point memory A) {
        // Compute A := A0 + r * A1 + ... + r^{m-1} * Am
        {
            uint256[] memory scalars = new uint256[](2);
            Types.G1Point[] memory bases = new Types.G1Point[](2);
            scalars[0] = 1;
            bases[0] = proof.opening_proof;

            scalars[1] = challenge.u;
            bases[1] = proof.shifted_opening_proof;

            A = Bn254Crypto.multiScalarMul(bases, scalars);
        }
    }

    function compute_B(
        Types.PcsInfo memory pcsInfo,
        Types.Proof memory proof,
        Types.ChallengeTranscript memory challenge,
        Types.VerificationKey memory vk
    ) internal view returns (Types.G1Point memory B) {
        // Compute B := B0 + r * B1 + ... + r^{m-1} * Bm
        {
            pcsInfo.commScalars[54] = challenge.zeta;
            pcsInfo.commBases[54] = proof.opening_proof;

            pcsInfo.commScalars[55] = mulmod(
                challenge.u,
                pcsInfo.nextEvalPoint,
                p
            );
            pcsInfo.commBases[55] = proof.shifted_opening_proof;

            pcsInfo.commScalars[56] = Bn254Crypto.negate_fr(pcsInfo.eval);
            pcsInfo.commBases[56] = vk.open_key_g;
            

            // Accumulate scalars which have the same base
            (
                Types.G1Point[] memory bases_after_acc,
                uint256[] memory scalars_after_acc
            ) = accumulate_scalar_with_same_base(
                    pcsInfo.commBases,
                    pcsInfo.commScalars
                );
                B = Bn254Crypto.negate_G1Point(
                Bn254Crypto.multiScalarMul(bases_after_acc, scalars_after_acc)
            );
        }
    }

    /**
     * @dev Compute components in [E]1 and [F]1 used for PolyComm opening verification
     * @param - verifyingKey: A VerificationKey struct
     * @param - evalData: EvalData including vanish_eval, lagrange_1_eval and piEval
     * @param - proof: A struct of Plonk proof
     * @param - chal: A struct of ChallengeTranscript
     * @param - commScalars: an empty uint256[32]
     * @param - commBases: an empty Types.G1Point[32]
     * @return - eval: a commitment which is a generalization of
     `[F]1` described in Sec 8.4, step 10 of https://eprint.iacr.org/2019/953.pdf
     eval is the scalar in `[E]1` described in Sec 8.4, step 11 of https://eprint.iacr.org/2019/953
     */
    function prepare_OpeningProof(
        uint256[] memory publicInput,
        Types.VerificationKey memory verifyingKey,
        PolynomialEval.EvalData memory evalData,
        Types.Proof memory proof,
        Types.ChallengeTranscript memory chal,
        uint256[] memory commScalars,
        Types.G1Point[] memory commBases,
        PolynomialEval.EvalDomain memory domain
    ) internal view returns (uint256) {
        
       uint256 lin_poly_constant = compute_lin_poly_constant_term(
        publicInput,
            chal,
            proof,
            evalData,
            domain
        );

        uint256[] memory buffer_v_and_uv_basis = prepare_PolyCommitments(
            verifyingKey,
            chal,
            evalData,
            proof,
            commScalars,
            commBases,
            domain
        );
        uint256 eval = prepare_evaluations(
            lin_poly_constant,
            proof,
            buffer_v_and_uv_basis
        );
        return eval;
    }

    /**
     * @dev Compute the constant term of the linearization polynomial
     * @param - chal: A challeng struct
     * @param - proof: A struct of Plonk proof
     * @param - EvalData: polynomial commitment evaluation info
     * @return - res: constant term
     */
    //   r_plonk = PI - L1(x) * alpha^2 - alpha *  (w_1 + beta * sigma_1 + gamma) * (w_m + gamma) * z(xw)
    //   where m is the number of wire types.
    //   r_0 = \sum_{j=1..m} alpha^{k_j} * (r_plonk_j)
    //   k_j is the number of alpha power terms added to the first j-1 instances.

    function compute_lin_poly_constant_term(
        uint256[] memory publicInput,
        Types.ChallengeTranscript memory chal,
        Types.Proof memory proof,
        PolynomialEval.EvalData memory evalData,
        PolynomialEval.EvalDomain memory domain
    ) internal view returns (uint256) {
        // evaluate_pi_poly
        // let vanish_eval_div_n = E::ScalarField::from(self.domain.size() as u32)
        //     .inverse()
        //     .ok_or(PlonkError::DivisionError)?
        //     * (*vanish_eval);
        uint256 vanish_eval_div_n = mulmod(
            domain.sizeInv,
            evalData.vanish_eval,
            p
        );
        uint256 result = mulmod(publicInput[0], mulmod(vanish_eval_div_n, Bn254Crypto.invert(addmod(chal.zeta, p - 1, p)), p), p);
         
        //  results - alpha_powers[0] * lagrange_1_eval
        // let mut tmp = self.evaluate_pi_poly(pi, &challenges.zeta, vanish_eval, vk.is_merged)?
        uint256 tmp = addmod(result, Bn254Crypto.negate_fr(mulmod(chal.alpha2, evalData.lagrange_1_eval, p)), p);
        uint256 plookup_constant = compute_plookup_constant(chal, proof, evalData, domain);
        uint256 tmpOut = compute_tmp(tmp, chal, proof);
        tmpOut = addmod(tmpOut, mulmod(chal.alpha_powers[1], plookup_constant, p), p);
        uint256 result_lin = mulmod(chal.alpha_base, tmpOut, p);
        return result_lin;
    }

    function compute_plookup_constant(
    Types.ChallengeTranscript memory chal,
    Types.Proof memory proof,
    PolynomialEval.EvalData memory evalData,
    PolynomialEval.EvalDomain memory domain
) internal view returns (uint256) {
    uint256 gamma_mul_beta_plus_one = mulmod(
        addmod(chal.beta, 1, p),
        chal.gamma,
        p
    );

    uint256 term1 = mulmod(
        evalData.lagrange_n_eval,
        addmod(
            proof.h_1_eval,
            p - addmod(proof.h_2_next_eval, chal.alpha_powers[0], p),
            p
        ),
        p
    );

    uint256 term2 = mulmod(chal.alpha, evalData.lagrange_1_eval, p);

    uint256 part = mulmod(
        chal.alpha_powers[1],
        mulmod(
            addmod(chal.zeta, p - domain.groupGenInv, p),
            proof.prod_next_eval,
            p
        ),
        p
    );

    part = mulmod(
        part,
        addmod(
            gamma_mul_beta_plus_one,
            addmod(proof.h_1_eval, mulmod(chal.beta, proof.h_1_next_eval, p), p),
            p
        ),
        p
    );

    part = mulmod(
        part,
        addmod(
            gamma_mul_beta_plus_one,
            mulmod(chal.beta, proof.h_2_next_eval, p),
            p
        ),
        p
    );

    return addmod(
        addmod(term1, p - term2, p),
        p - part,
        p
    );
}

 function compute_tmp(
    uint256 tmp,
    Types.ChallengeTranscript memory chal,
    Types.Proof memory proof
) internal view returns (uint256) {
     uint256[5] memory first_w_evals = [proof.wires_evals_1, proof.wires_evals_2, proof.wires_evals_3, proof.wires_evals_4, proof.wires_evals_5];
        uint256 last_w_eval = proof.wires_evals_6;
        uint256[5] memory sigma_evals = [proof.wire_sigma_evals_1, proof.wire_sigma_evals_2, proof.wire_sigma_evals_3, proof.wire_sigma_evals_4, proof.wire_sigma_evals_5];
        uint256 acc =  mulmod(mulmod(chal.alpha,proof.perm_next_eval,p),addmod(chal.gamma,last_w_eval,p),p);
        for (uint256 i = 0; i < 5; i++) {
            acc = mulmod(acc,addmod(addmod(chal.gamma, first_w_evals[i], p), mulmod(chal.beta, sigma_evals[i], p), p),p);
        }
        tmp = addmod(tmp, Bn254Crypto.negate_fr(acc), p);
    return tmp;
    }
    // a helper function to avoid stack too deep error when computing plookup_constant
    function help(
        Types.ChallengeTranscript memory chal,
        Types.Proof memory proof,
        PolynomialEval.EvalDomain memory domain
    ) internal view returns (uint256) {
        uint256 gamma_mul_beta_plus_one = mulmod(addmod(chal.beta, 1, p), chal.gamma, p);
       uint256 res =  mulmod(
        mulmod(
            mulmod(
                chal.alpha_powers[1],
                addmod(
                    chal.zeta,
                    p - domain.groupGenInv,
                    p
                ),
                p
            ),
            proof.prod_next_eval,
            p
        ),
        mulmod(
            addmod(
                gamma_mul_beta_plus_one,
                addmod(
                    proof.h_1_eval,
                    mulmod(chal.beta, proof.h_1_next_eval, p),
                    p
                ),
                p
            ),
            addmod(
                gamma_mul_beta_plus_one,
                mulmod(chal.beta, proof.h_2_next_eval, p),
                p
            ),
            p
        ),
        p
    );
     return res;
    }

    /**
     * @dev Prepar the polynomial commitments to a single commitment (in the ScalarsAndBases form).
     This is a simplified version of  `aggregate_poly_commitments()` in Jellyfish preparing for `[F]1` from a single proof
     * @param - verifyingKey
     * @param - chal
     * @param - evalData
     * @param - proof: A struct of Plonk proof
     * @param - commScalars
     * @param - commBases
     * @return - buffer_v_and_uv_basis: a generalization of `[F]1` described in Sec 8.4, step 10 of https://eprint.iacr.org/2019/953.pdf
     */
    function prepare_PolyCommitments(
        Types.VerificationKey memory verifyingKey,
        Types.ChallengeTranscript memory chal,
        PolynomialEval.EvalData memory evalData,
        Types.Proof memory proof,
        uint256[] memory commScalars,
        Types.G1Point[] memory commBases,
        PolynomialEval.EvalDomain memory domain
    ) internal view returns (uint256[] memory) {
        // Compute the first part of the batched polynomial commitment `[D]1` described in Sec 8.4, step 9 of https://eprint.iacr.org/2019/953.pdf
        linearization_scalars_and_bases(
            verifyingKey,
            chal,
            evalData,
            proof,
            commBases,
            commScalars,
            domain
        );
        // linearization_scalars_and_bases added 0-25 scalars and bases

        // Add wire witness polynomial commitments.

        // divide into two functions to avoid stack too deep
        (
            uint256[] memory buffer_v_and_uv_basis,
            uint256 v_base,
            uint256 uv_base
        ) = compute_buffer_v_and_uv_basis_1(
                chal,
                proof,
                commScalars,
                commBases
            );
            // have 32 scalars

        // Add wire sigma polynomial commitments. The last sigma commitment is excluded.
        compute_buffer_v_and_uv_basis_2_parameters memory z;
        z.buffer_v_and_uv_basis = buffer_v_and_uv_basis;
        z.start_index = 32;
        z.verifyingKey = verifyingKey;
        z.chal = chal;
        z.commScalars = commScalars;
        z.commBases = commBases;
        z.v_base = v_base;
        uint256 new_v_base= compute_buffer_v_and_uv_basis_2(z);

        compute_buffer_v_and_uv_basis_3_parameters memory z3;
        z3.chal = chal;
        z3.vk = verifyingKey;
        z3.proof = proof;
        z3.start_index = 31;
        z3.buffer_v_and_uv_basis = buffer_v_and_uv_basis;
        z3.v_base = new_v_base;
        z3.uv_base = uv_base;
        z3.commScalars = commScalars;
        z3.commBases = commBases;
        compute_buffer_v_and_uv_basis_3(z3);
        return buffer_v_and_uv_basis;
    }

    /**
     * @dev Add wire witness polynomial commitments.
     * @param - challenge: A challeng struct
     * @param - proof: A struct of Plonk proof
     * @param - commScalars
     * @param - commBases
     * @return - buffer_v_and_uv_basis, v_base
     */
    function compute_buffer_v_and_uv_basis_1(
        Types.ChallengeTranscript memory chal,
        Types.Proof memory proof,
        uint256[] memory commScalars,
        Types.G1Point[] memory commBases
    ) internal pure returns (uint256[] memory, uint256, uint256) {
        // uint256 start_index = 26;
        uint256 v = chal.v;
        uint256 v_base = chal.v;
        uint256 uv_base = chal.u;

        uint256[] memory buffer_v_and_uv_basis = new uint256[](27);
        // Add poly commitments to be evaluated at point `zeta * g`.
       
        Types.G1Point memory proof_elem2;
        uint256 p_local = Bn254Crypto.r_mod;

        assembly {
            for {
                let i := 0
            } lt(i, 6) {
                i := add(i, 1)
            } {
                let commIndex := add(27, i)
                mstore(add(buffer_v_and_uv_basis, mul(add(i, 1), 0x20)), v_base)
                mstore(add(commScalars, mul(add(commIndex, 1), 0x20)), v_base)
                let proof_elem := mload(add(add(proof, 0x00), mul(i, 0x20)))
                mstore(add(commBases, mul(add(commIndex, 1), 0x20)), proof_elem)
                v_base := mulmod(v_base, v, p_local)
            }
            // Add poly commitments to be evaluated at point `zeta * g`.
            // mstore(add(buffer_v_and_uv_basis, mul(add(8, 1), 0x20)), uv_base)
            let commIndex := add(27, 11)
            mstore(add(commScalars, mul(add(commIndex, 1), 0x20)), uv_base)
            proof_elem2 := mload(add(proof, 0x180)) //prod_perm_poly_comm
            mstore(add(commBases, mul(add(commIndex, 1), 0x20)), proof_elem2)
            
        }

        buffer_v_and_uv_basis[11] = uv_base;
        commScalars[38] = uv_base;
        commBases[38] = proof.prod_perm_poly_comm;
        return (buffer_v_and_uv_basis, v_base,mulmod(uv_base, v, p_local));
    }

    /**
     * Add sigma polynomial commitments
     * compute_buffer_v_and_uv_basis_2_parameters: including:
     buffer_v_and_uv_basis,start_index,verifyingKey,chal,commScalars,commBases,v_base
     */
    function compute_buffer_v_and_uv_basis_2(
        compute_buffer_v_and_uv_basis_2_parameters memory z
    ) internal pure returns (uint256 res){
        uint256[] memory buffer_v_and_uv_basis = z.buffer_v_and_uv_basis;
        uint256 start_index = 27;//z.start_index;
        Types.VerificationKey memory verifyingKey = z.verifyingKey;
        Types.ChallengeTranscript memory chal = z.chal;
        uint256[] memory commScalars = z.commScalars;
        Types.G1Point[] memory commBases = z.commBases;
        uint256 v_base = z.v_base;
        uint256 v = chal.v;
        uint256 p_local = Bn254Crypto.r_mod;

        // Add wire sigma polynomial commitments. The last sigma commitment is excluded.
        assembly {
            for {
                let i := 6
            } lt(i, 11) { 
                i := add(i, 1)
            } {
                let commIndex := add(start_index, i)
                mstore(add(buffer_v_and_uv_basis, mul(add(i, 1), 0x20)), v_base)
                mstore(add(commScalars, mul(add(commIndex, 1), 0x20)), v_base)
                let verifyingKey_elem := mload(
                    add(add(verifyingKey, 0x40), mul(sub(i, 6), 0x20))
                )
                mstore(
                    add(commBases, mul(add(commIndex, 1), 0x20)),
                    verifyingKey_elem
                )
                v_base := mulmod(v_base, v, p_local)
            }
        }
        res = v_base;
       

    }

    // Add Plookup polynomial commitments
     function compute_buffer_v_and_uv_basis_3(
        compute_buffer_v_and_uv_basis_3_parameters memory z
    ) internal pure {
        uint256 p_local = Bn254Crypto.r_mod;
        uint256 v = z.chal.v;
        z.start_index =39;
        Types.G1Point[6] memory plookup_comms = 
        [
            z.vk.range_table_comm,
            z.vk.key_table_comm,
            z.proof.h_poly_comm_1,
            z.vk.selector_comms_18,
            z.vk.table_dom_sep_comm,
            z.vk.q_dom_sep_comm
        ];
        
        for (uint256 i = 0; i < 6; i++) {
            z.buffer_v_and_uv_basis[12 + i] = z.v_base;
            z.commScalars[z.start_index + i] = z.v_base;
            z.commBases[z.start_index + i] = plookup_comms[i];
            z.v_base = mulmod(z.v_base, v, p_local);
        }

Types.G1Point[9] memory plookup_shifted_comms =[
    z.proof.prod_lookup_poly_comm, //45
    z.vk.range_table_comm, //46
    z.vk.key_table_comm, //47
    z.proof.h_poly_comm_1, //48
    z.proof.h_poly_comm_2, //49
    // q_dom_sep_comm, z.vk.selector_comms_18
    z.vk.selector_comms_18, // 50
    z.proof.wires_poly_comms_4, //51
    z.proof.wires_poly_comms_5, //52
    z.vk.table_dom_sep_comm //53
];

z.start_index =45;
 for (uint256 i = 0; i < 9; i++) {
            z.buffer_v_and_uv_basis[18 + i] = z.uv_base;
            z.commScalars[z.start_index + i] = z.uv_base;
            z.commBases[z.start_index + i] = plookup_shifted_comms[i];
            z.uv_base = mulmod(z.uv_base, v, p_local);

        }
    }
    function linearization_scalars_and_bases(
        Types.VerificationKey memory verifyingKey,
        Types.ChallengeTranscript memory challenge,
        PolynomialEval.EvalData memory evalData,
        Types.Proof memory proof,
        Types.G1Point[] memory bases,
        uint256[] memory scalars,
        PolynomialEval.EvalDomain memory domain
    ) internal view {
        scalars[0] = compute_first_scalar(
            evalData,
            verifyingKey,
            proof,
            challenge
        );
        
        scalars[1] = compute_second_scalar(proof, challenge);
       

        // compute first base and second base
       
        assembly {
            // G1Point prod_perm_poly_comm;
            mstore(add(bases, 0x20), mload(add(proof, 0xc0)))
            // G1Point sigma_comms_6;
            mstore(add(bases, 0x40), mload(add(verifyingKey, 0xe0)))
        }
        // set the function parameters to avoid stack too deep error
        add_selector_polynomial_commitments_parameters
            memory x = add_selector_polynomial_commitments_parameters(
                verifyingKey,
                bases,
                scalars,
                proof
            );

        add_selector_polynomial_commitments(x);

        add_plookup_commitments(bases,scalars,proof,challenge,domain,evalData);
       
        add_splitted_quotient_commitments_parameter memory y;

       
        y.index = 21; // 21 scalars so far
        y.challenge_zeta = challenge.zeta;
        y.evalData_vanish_eval = evalData.vanish_eval;
        y.bases = bases;
        y.scalars = scalars;
        y.proof = proof;
        add_splitted_quotient_commitments(y);
    }

    function compute_first_scalar(
        PolynomialEval.EvalData memory evalData,
        Types.VerificationKey memory verifyingKey,
        Types.Proof memory proof,
        Types.ChallengeTranscript memory challenge
    ) internal pure returns (uint256 firstScalar) {
        uint256 p_local = Bn254Crypto.r_mod;

        // ============================================
        // Compute coefficient for the permutation product polynomial commitment.
        // firstScalar =
        //          L1(zeta) * alpha^2
        //          + alpha
        //              * (beta * zeta      + wireEval0 + gamma)
        //              * (beta * k1 * zeta + wireEval1 + gamma)
        //              * (beta * k2 * zeta + wireEval2 + gamma)
        //              * ...
        // where wireEval0, wireEval1, wireEval2, ... are in w_evals
        // ============================================

        assembly {
            // Load challenges directly into registers
            let challenge_alpha := mload(add(challenge, 0x60))
            let challenge_beta := mload(add(challenge, 0x20))
            let challenge_gamma := mload(add(challenge, 0x40))
            let challenge_zeta := mload(add(challenge, 0x80))
            // firstScalar = L1(zeta) * alpha^2
            //       + alpha
            //       * (beta * zeta      + a_bar + gamma)
            //       * (beta * k1 * zeta + b_bar + gamma)
            //       * (beta * k2 * zeta + c_bar + gamma)
            // where a_bar, b_bar and c_bar are in w_evals
            firstScalar := mulmod(
                mload(add(challenge, 0xe0)), //alpha2
                mload(add(evalData, 0x20)), //lagrange_1_eval
                p_local
            )
            
            // firstScalar += w_evals
            //             .iter()
            //             .zip(vk.k.iter())
            //             .fold(challenges.alpha, |acc, (w_eval, k)| {
            //                 acc * (challenges.beta * k * challenges.zeta + challenges.gamma + w_eval)
            //             });
            let acc := challenge_alpha
            let tmp := mulmod(
                challenge_beta,
                mload(add(verifyingKey, 0x340)),
                p_local
            ) //K1
            tmp := mulmod(tmp, challenge_zeta, p_local)
            tmp := addmod(tmp, challenge_gamma, p_local)
            tmp := addmod(tmp, mload(add(proof, 0x200)), p_local) // wires_evals_1
            acc := mulmod(acc, tmp, p_local)

            tmp := mulmod(
                challenge_beta,
                mload(add(verifyingKey, 0x360)),
                p_local
            )//K2
            tmp := mulmod(tmp, challenge_zeta, p_local)
            tmp := addmod(tmp, challenge_gamma, p_local)
            tmp := addmod(tmp, mload(add(proof, 0x220)), p_local) // wires_evals_2
            acc := mulmod(acc, tmp, p_local)

            tmp := mulmod(
                challenge_beta,
                mload(add(verifyingKey, 0x380)),
                p_local
            )//k3
            tmp := mulmod(tmp, challenge_zeta, p_local)
            tmp := addmod(tmp, challenge_gamma, p_local)
            tmp := addmod(tmp, mload(add(proof, 0x240)), p_local) // wires_evals_3
            acc := mulmod(acc, tmp, p_local)

            tmp := mulmod(
                challenge_beta,
                mload(add(verifyingKey, 0x3a0)),
                p_local
            )//k4
            tmp := mulmod(tmp, challenge_zeta, p_local)
            tmp := addmod(tmp, challenge_gamma, p_local)
            tmp := addmod(tmp, mload(add(proof, 0x260)), p_local) // wires_evals_4
            acc := mulmod(acc, tmp, p_local)

            tmp := mulmod(
                challenge_beta,
                mload(add(verifyingKey,0x3c0)),
                p_local
            ) // k5
            tmp := mulmod(tmp, challenge_zeta, p_local)
            tmp := addmod(tmp, challenge_gamma, p_local)
            tmp := addmod(tmp, mload(add(proof, 0x280)), p_local) // wires_evals_5
            acc := mulmod(acc, tmp, p_local)

            tmp := mulmod(
                challenge_beta,
                mload(add(verifyingKey,0x3e0)),
                p_local
            ) // k6
            tmp := mulmod(tmp, challenge_zeta, p_local)
            tmp := addmod(tmp, challenge_gamma, p_local)
            tmp := addmod(tmp, mload(add(proof, 0x2a0)), p_local) // wires_evals_6
            acc := mulmod(acc, tmp, p_local)

            firstScalar := addmod(firstScalar, acc, p_local)

        }
        return firstScalar;
    }

    // ============================================
    // Compute coefficient for the last wire sigma polynomial commitment.
    // secondScalar = alpha * beta * z_w * [s_sigma_3]_1
    //              * (wireEval0 + gamma + beta * sigmaEval0)
    //              * (wireEval1 + gamma + beta * sigmaEval1)
    //              * ...
    // ============================================
    function compute_second_scalar(
        Types.Proof memory proof,
        Types.ChallengeTranscript memory challenge
    ) internal pure returns (uint256 secondScalar) {
        uint256 p_local = Bn254Crypto.r_mod;

        assembly {
            // Load challenges and necessary proof data into registers
            let challenge_alpha := mload(add(challenge, 0x60)) // alpha
            let challenge_beta := mload(add(challenge, 0x20)) // beta
            let challenge_gamma := mload(add(challenge, 0x40)) // gamma

            secondScalar := mulmod(challenge_alpha, challenge_beta, p_local)
            secondScalar := mulmod(
                secondScalar,
                mload(add(proof, 0x360)),
                p_local
            ) // perm_next_eval

            let tmp := mulmod(challenge_beta, mload(add(proof, 0x2c0)), p_local) // wire_sigma_evals_1
            tmp := addmod(tmp, challenge_gamma, p_local)
            tmp := addmod(tmp, mload(add(proof, 0x200)), p_local) // wires_evals_1
            secondScalar := mulmod(secondScalar, tmp, p_local)

            tmp := mulmod(challenge_beta, mload(add(proof, 0x2e0)), p_local) // wire_sigma_evals_2
            tmp := addmod(tmp, challenge_gamma, p_local)
            tmp := addmod(tmp, mload(add(proof, 0x220)), p_local) // wires_evals_2
            secondScalar := mulmod(secondScalar, tmp, p_local)

            tmp := mulmod(challenge_beta, mload(add(proof, 0x300)), p_local) // wire_sigma_evals_3
            tmp := addmod(tmp, challenge_gamma, p_local)
            tmp := addmod(tmp, mload(add(proof, 0x240)), p_local) // wires_evals_3
            secondScalar := mulmod(secondScalar, tmp, p_local)

            tmp := mulmod(challenge_beta, mload(add(proof, 0x320)), p_local) // wire_sigma_evals_4
            tmp := addmod(tmp, challenge_gamma, p_local)
            tmp := addmod(tmp, mload(add(proof, 0x260)), p_local) // wires_evals_4
            secondScalar := mulmod(secondScalar, tmp, p_local)

            tmp := mulmod(challenge_beta, mload(add(proof, 0x340)), p_local) 
            // wire_sigma_evals_5
            tmp := addmod(tmp, challenge_gamma, p_local)
            tmp := addmod(tmp, mload(add(proof, 0x280)), p_local) // wires_evals_5
            secondScalar := mulmod(secondScalar, tmp, p_local)
        }
        return Bn254Crypto.negate_fr(secondScalar);
    }

    function add_selector_polynomial_commitments(
        add_selector_polynomial_commitments_parameters memory x
    ) internal pure {
        uint256 start_index = 2;
        Types.VerificationKey memory verifyingKey = x.verifyingKey;
        Types.G1Point[] memory bases = x.bases;
        uint256[] memory scalars = x.scalars;
        Types.Proof memory proof = x.proof;
        uint256 p_local = Bn254Crypto.r_mod;

        assembly {
            let proofPtr := proof
            let verifyingKeyPtr := verifyingKey
            let scalarsPtr := add(scalars, mul(add(start_index, 1), 0x20)) // Point to scalars[start_index]

            // Load proof evaluations into variables
            let wires_evals_1 := mload(add(proofPtr, 0x200))
            let wires_evals_2 := mload(add(proofPtr, 0x220))
            let wires_evals_3 := mload(add(proofPtr, 0x240))
            let wires_evals_4 := mload(add(proofPtr, 0x260))
            let wires_evals_5 := mload(add(proofPtr, 0x280))
            // let wires_evals_6 := mload(add(proofPtr, 0x2a0))

            // scalars calculations
            mstore(scalarsPtr, wires_evals_1)
            mstore(add(scalarsPtr, 0x20), wires_evals_2)
            mstore(add(scalarsPtr, 0x40), wires_evals_3)
            mstore(add(scalarsPtr, 0x60), wires_evals_4)
            mstore(
                add(scalarsPtr, 0x80),
                mulmod(wires_evals_1, wires_evals_2, p_local)
            )
            mstore(
                add(scalarsPtr, 0xA0),
                mulmod(wires_evals_3, wires_evals_4, p_local)
            )
        }
        scalars[start_index + 6] = PolynomialEval.power(
            proof.wires_evals_1,
            5,
            p_local
        );
        scalars[start_index + 7] = PolynomialEval.power(
            proof.wires_evals_2,
            5,
            p_local
        );
        scalars[start_index + 8] = PolynomialEval.power(
            proof.wires_evals_3,
            5,
            p_local
        );
        scalars[start_index + 9] = PolynomialEval.power(
            proof.wires_evals_4,
            5,
            p_local
        );
        assembly {
            let proofPtr := proof
            let verifyingKeyPtr := verifyingKey
            let scalarsPtr := add(scalars, mul(add(start_index, 1), 0x20)) // Point to scalars[start_index]
            let basesPtr := add(bases, mul(add(start_index, 1), 0x20)) // Point to bases[start_index] (each element is two 32-byte words)

            // Load proof evaluations into variables
            let wires_evals_1 := mload(add(proofPtr, 0x200))
            let wires_evals_2 := mload(add(proofPtr, 0x220))
            let wires_evals_3 := mload(add(proofPtr, 0x240))
            let wires_evals_4 := mload(add(proofPtr, 0x260))
            let wires_evals_5 := mload(add(proofPtr, 0x280))
            // let wires_evals_6 := mload(add(proofPtr, 0x2a0))

            // scalars calculations
            mstore(add(scalarsPtr, 0x160), 1)
            mstore(
                add(scalarsPtr, 0x180),
                mulmod(
                    wires_evals_1,
                    mulmod(
                        wires_evals_2,
                        mulmod(
                            wires_evals_3,
                            mulmod(wires_evals_4, wires_evals_5, p_local),
                            p_local
                        ),
                        p_local
                    ),
                    p_local
                )
            )
               //// q_scalars[13] = w_evals[0] * w_evals[3] * w_evals[2] * w_evals[3]
            //     + w_evals[1] * w_evals[2] * w_evals[2] * w_evals[3];
             mstore(
                add(scalarsPtr, 0x1A0),
        addmod(
           mulmod( mulmod(mulmod(wires_evals_1, wires_evals_4, p_local), wires_evals_3, p_local), wires_evals_4, p_local),
           mulmod( mulmod(mulmod(wires_evals_2, wires_evals_3, p_local), wires_evals_3, p_local), wires_evals_4, p_local),
            p_local
        )
        )
        // q_scalars[14] = w_evals[0] * w_evals[2]
            //     + w_evals[1] * w_evals[3]
            //     + E::ScalarField::from(2u8) * w_evals[0] * w_evals[3]
            //     + E::ScalarField::from(2u8) * w_evals[1] * w_evals[2];
          mstore(
                add(scalarsPtr, 0x1C0),
        addmod(
           mulmod(wires_evals_1, wires_evals_3, p_local),
              addmod(
                mulmod(wires_evals_2, wires_evals_4, p_local),
                addmod(
                     mulmod(2, mulmod(wires_evals_1, wires_evals_4, p_local), p_local),
                     mulmod(2, mulmod(wires_evals_2, wires_evals_3, p_local), p_local),
                     p_local
                ),
                p_local
        ),
        p_local
        )
        )
        // q_scalars[15] = w_evals[2] * w_evals[2] * w_evals[3] * w_evals[3];
        mstore( add(scalarsPtr, 0x1E0),
        mulmod( mulmod(mulmod(wires_evals_3, wires_evals_3, p_local), wires_evals_4, p_local), wires_evals_4, p_local)
        )
        // q_scalars[16] =
            //     w_evals[0] * w_evals[0] * w_evals[1] + w_evals[0] * w_evals[1] * w_evals[1];
        mstore( add(scalarsPtr, 0x200),
        addmod(
              mulmod(mulmod(wires_evals_1, wires_evals_1, p_local), wires_evals_2, p_local),
                mulmod(mulmod(wires_evals_1, wires_evals_2, p_local), wires_evals_2, p_local),
                p_local
        )
        )
            mstore(basesPtr, mload(add(verifyingKeyPtr, 0x100))) //selector_comms_1
            mstore(add(basesPtr, 0x20), mload(add(verifyingKeyPtr, 0x120))) //selector_comms_2
            mstore(add(basesPtr, 0x40), mload(add(verifyingKeyPtr, 0x140))) //selector_comms_3
            mstore(add(basesPtr, 0x60), mload(add(verifyingKeyPtr, 0x160))) //selector_comms_4
            mstore(add(basesPtr, 0x80), mload(add(verifyingKeyPtr, 0x180))) //selector_comms_5
            mstore(add(basesPtr, 0xa0), mload(add(verifyingKeyPtr, 0x1a0))) //selector_comms_6
            mstore(add(basesPtr, 0xc0), mload(add(verifyingKeyPtr, 0x1c0))) //selector_comms_7
            mstore(add(basesPtr, 0xe0), mload(add(verifyingKeyPtr, 0x1e0))) //selector_comms_8
            mstore(add(basesPtr, 0x100), mload(add(verifyingKeyPtr, 0x200))) //selector_comms_9
            mstore(add(basesPtr, 0x120), mload(add(verifyingKeyPtr, 0x220))) //selector_comms_10
            mstore(add(basesPtr, 0x140), mload(add(verifyingKeyPtr, 0x240))) //selector_comms_11
            mstore(add(basesPtr, 0x160), mload(add(verifyingKeyPtr, 0x260))) //selector_comms_12
            mstore(add(basesPtr, 0x180), mload(add(verifyingKeyPtr, 0x280))) //selector_comms_13
             mstore(add(basesPtr, 0x1A0), mload(add(verifyingKeyPtr, 0x2a0))) //selector_comms_14
            mstore(add(basesPtr, 0x1C0), mload(add(verifyingKeyPtr, 0x2c0))) //selector_comms_15
            mstore(add(basesPtr, 0x1E0), mload(add(verifyingKeyPtr, 0x2e0))) //selector_comms_16
            mstore(add(basesPtr, 0x200), mload(add(verifyingKeyPtr, 0x300))) //selector_comms_17
            mstore(add(basesPtr, 0x220), mload(add(verifyingKeyPtr, 0x320))) //selector_comms_18

        }

        scalars[start_index + 10] = Bn254Crypto.negate_fr(proof.wires_evals_5);
       
    }

    // add Plookup related commitments
    function add_plookup_commitments(
        Types.G1Point[] memory bases,
        uint256[] memory scalars,
        Types.Proof memory proof,
        Types.ChallengeTranscript memory challenge,
        PolynomialEval.EvalDomain memory domain,
         PolynomialEval.EvalData memory evalData
    ) internal view {
     scalars[19] =add_plookup_commitments_helper1(
         proof,
         challenge,
         domain,
         evalData
     );
     bases[19] = proof.prod_lookup_poly_comm;
     scalars[20] = add_plookup_commitments_helper2(
            proof,
            challenge,
            domain
        );
     bases[20] = proof.h_poly_comm_2;
    }
    
    // to avoid the stack too deep error
     function add_plookup_commitments_helper1(
        Types.Proof memory proof,
        Types.ChallengeTranscript memory challenge,
        PolynomialEval.EvalDomain memory domain,
        PolynomialEval.EvalData memory evalData
    ) internal view returns (uint256 res) {
       uint256 merged_lookup_x = add_plookup_commitments_helper1_1(proof, challenge);

       uint256 merged_table_x = add_plookup_commitments_helper1_2(proof, challenge);

       uint256 merged_table_xw = add_plookup_commitments_helper1_3(proof, challenge);
        res = add_plookup_commitments_helper1_4 (
            challenge,
            domain,
            evalData,
            merged_lookup_x,
            merged_table_x,
            merged_table_xw
        );
    }

    function add_plookup_commitments_helper1_1(
        Types.Proof memory proof,
        Types.ChallengeTranscript memory challenge
    ) internal view returns (uint256 res) {
       res = addmod(proof.wires_evals_6, mulmod(proof.q_lookup_eval, mulmod(challenge.tau, addmod(proof.q_dom_sep_eval, mulmod(challenge.tau, addmod(proof.wires_evals_1, mulmod(challenge.tau, addmod(proof.wires_evals_2, mulmod(challenge.tau, proof.wires_evals_3, p), p), p), p), p), p), p), p), p);
    }

    function add_plookup_commitments_helper1_2(
        Types.Proof memory proof,
        Types.ChallengeTranscript memory challenge
    ) internal view returns (uint256 res) {
       res = addmod(proof.range_table_eval, mulmod(proof.q_lookup_eval, mulmod(challenge.tau, addmod(proof.table_dom_sep_eval, mulmod(challenge.tau, addmod(proof.key_table_eval, mulmod(challenge.tau, addmod(proof.wires_evals_4, mulmod(challenge.tau, proof.wires_evals_5, p), p), p), p), p), p), p), p), p);
    }

    function add_plookup_commitments_helper1_3(
        Types.Proof memory proof,
        Types.ChallengeTranscript memory challenge
    ) internal view returns (uint256 res) {
       res = addmod(proof.range_table_next_eval, mulmod(proof.q_lookup_next_eval, mulmod(challenge.tau, addmod(proof.table_dom_sep_next_eval, mulmod(challenge.tau, addmod(proof.key_table_next_eval, mulmod(challenge.tau, addmod(proof.w_3_next_eval, mulmod(challenge.tau, proof.w_4_next_eval, p), p), p), p), p), p), p), p), p);
    }

    function add_plookup_commitments_helper1_4(
        Types.ChallengeTranscript memory challenge,
        PolynomialEval.EvalDomain memory domain,
        PolynomialEval.EvalData memory evalData,
        uint256 merged_lookup_x,
        uint256  merged_table_x,
        uint256  merged_table_xw
    ) internal view returns (uint256 res) {
      
    uint256 b = add_plookup_commitments_helper1_4_1 (challenge, domain, evalData, merged_lookup_x, merged_table_x, merged_table_xw);

       res = mulmod( challenge.alpha_base, b, p);
    }
    function add_plookup_commitments_helper1_4_1(
        Types.ChallengeTranscript memory challenge,
        PolynomialEval.EvalDomain memory domain,
        PolynomialEval.EvalData memory evalData,
        uint256 merged_lookup_x,
        uint256  merged_table_x,
        uint256  merged_table_xw
    ) internal view returns (uint256 res) {
      
uint256 c = mulmod(
                challenge.alpha_powers[4],
                addmod(challenge.zeta, Bn254Crypto.negate_fr(domain.groupGenInv), p),
                p
            );
     
     
      res =addmod(
   add_plookup_commitments_helper1_4_2 (challenge, evalData),
    mulmod(
    mulmod(
        mulmod(
            c,
            addmod(challenge.beta, 1, p),
            p
        ),
        addmod(challenge.gamma, merged_lookup_x, p),
        p
    ),
    add_plookup_commitments_helper1_4_3 (challenge,  merged_table_x, merged_table_xw),
    p
),
    p
);
    }
function add_plookup_commitments_helper1_4_2(
        Types.ChallengeTranscript memory challenge,
        PolynomialEval.EvalData memory evalData
    ) internal view returns (uint256 res) {
        res =addmod(
        mulmod(challenge.alpha_powers[2], evalData.lagrange_1_eval, p),
        mulmod(challenge.alpha_powers[3], evalData.lagrange_n_eval, p),
        p
    );
    }
    function add_plookup_commitments_helper1_4_3(
        Types.ChallengeTranscript memory challenge,
        uint256 merged_table_x,
        uint256  merged_table_xw
    ) internal view returns (uint256 res) {
        res =addmod(
        addmod(mulmod(addmod(challenge.beta,1,p), challenge.gamma,p), merged_table_x, p),
        mulmod(challenge.beta, merged_table_xw, p),
        p
    );
    }



     // to avoid the stack too deep error
     function add_plookup_commitments_helper2(
        Types.Proof memory proof,
        Types.ChallengeTranscript memory challenge,
        PolynomialEval.EvalDomain memory domain
    ) internal view returns (uint256 res) {
        res = mulmod(
            mulmod(
                mulmod(
                    mulmod(
                        challenge.alpha_powers[4],
                        addmod(Bn254Crypto.negate_fr(challenge.zeta), domain.groupGenInv, p),
                        p
                    ),
                    proof.prod_next_eval,
                    p
                ),
                addmod(
                    addmod(mulmod(addmod(challenge.beta,1,p), challenge.gamma,p), proof.h_1_eval,p),
                    mulmod(challenge.beta, proof.h_1_next_eval,p),
                    p
                ),
                p
            ),
            challenge.alpha_base,
            p
        );
    }

    function add_splitted_quotient_commitments(
        add_splitted_quotient_commitments_parameter memory y
    ) internal pure {
        uint256 index = y.index;
        uint256 evalData_vanish_eval = y.evalData_vanish_eval;
        uint256 challenge_zeta = y.challenge_zeta;
        Types.G1Point[] memory bases = y.bases;
        uint256[] memory scalars = y.scalars;
        Types.Proof memory proof = y.proof;

        uint256 p_local = Bn254Crypto.r_mod;
        uint256 coeff = Bn254Crypto.negate_fr(evalData_vanish_eval);

        assembly {
            let zeta_to_n_plus_2 := addmod(1, evalData_vanish_eval, p_local)
            zeta_to_n_plus_2 := mulmod(
                zeta_to_n_plus_2,
                challenge_zeta,
                p_local
            )
            zeta_to_n_plus_2 := mulmod(
                zeta_to_n_plus_2,
                challenge_zeta,
                p_local
            )
            let scalarsPtr := add(scalars, mul(add(index, 1), 0x20))
            // let basesPtr := add(bases, mul(add(index, 1), 0x20))

            let split_quot_poly_comms_1 := mload(add(proof, 0xe0))
            let split_quot_poly_comms_2 := mload(add(proof, 0x100))
            let split_quot_poly_comms_3 := mload(add(proof, 0x120))
            let split_quot_poly_comms_4 := mload(add(proof, 0x140))
            let split_quot_poly_comms_5 := mload(add(proof, 0x160))

            mstore(scalarsPtr, coeff)
            // mstore(basesPtr, split_quot_poly_comms_1)
            coeff := mulmod(coeff, zeta_to_n_plus_2, p_local)

            mstore(add(scalarsPtr, 0x20), coeff)
            // mstore(add(basesPtr, 0x20), split_quot_poly_comms_2)
            coeff := mulmod(coeff, zeta_to_n_plus_2, p_local)

            mstore(add(scalarsPtr, 0x40), coeff)
            // mstore(add(basesPtr, 0x40), split_quot_poly_comms_3)
            coeff := mulmod(coeff, zeta_to_n_plus_2, p_local)

            mstore(add(scalarsPtr, 0x60), coeff)
            // mstore(add(basesPtr, 0x60), split_quot_poly_comms_4)
            coeff := mulmod(coeff, zeta_to_n_plus_2, p_local)

            mstore(add(scalarsPtr, 0x80), coeff)
            coeff := mulmod(coeff, zeta_to_n_plus_2, p_local)
            // mstore(add(basesPtr, 0x80), split_quot_poly_comms_5)

            mstore(add(scalarsPtr, 0xa0), coeff)
            // mstore(add(basesPtr, 0xa0), split_quot_poly_comms_6)
        }
        bases[index] = proof.split_quot_poly_comms_1;
        bases[index + 1] = proof.split_quot_poly_comms_2;
        bases[index + 2] = proof.split_quot_poly_comms_3;
        bases[index + 3] = proof.split_quot_poly_comms_4;
        bases[index + 4] = proof.split_quot_poly_comms_5;
        bases[index + 5] = proof.split_quot_poly_comms_6;
    }

    function accumulate_scalar_with_same_base(
        Types.G1Point[] memory bases,
        uint256[] memory scalars
    ) internal pure returns (Types.G1Point[] memory, uint256[] memory) {
        uint256 p_local = Bn254Crypto.r_mod;
        require(bases.length == scalars.length, "Length mismatch");

        // Using uint256 instead of bytes32 since we're now dealing with XOR of two uint256 values
        Types.G1Point[] memory tempBases = new Types.G1Point[](bases.length);
        uint256[] memory tempScalars = new uint256[](bases.length);

        uint256 uniqueCount = 0;

        for (uint256 i = 0; i < bases.length; i++) {
    bool found = false;
    for (uint256 j = 0; j < uniqueCount && !found; j++) {
        if (bases[i].x == tempBases[j].x && bases[i].y == tempBases[j].y) {
            tempScalars[j] = addmod(tempScalars[j], scalars[i], p_local);
            found = true;
        }
    }
    if (!found) {
        tempBases[uniqueCount] = bases[i];
        tempScalars[uniqueCount] = scalars[i];
        uniqueCount++;
    }
}

        Types.G1Point[] memory finalBases = new Types.G1Point[](uniqueCount);
        uint256[] memory finalScalars = new uint256[](uniqueCount);
        for (uint256 i = 0; i < uniqueCount; i++) {
            finalBases[i] = tempBases[i];
            finalScalars[i] = tempScalars[i];
        }

        return (finalBases, finalScalars);
    }

    /**
     * dev Simplified version of`aggregate_evaluations()` in Jellyfish
       preparing `[E]1` from a single proof.
     * param - lin_poly_constant: A linear polynomial constant
     * param - proof: A struct of Plonk proof
     * param - buffer_v_and_uv_basis
     * return - eval:  the scalar in `[E]1` described in Sec 8.4, step 11 of https://eprint.iacr.org/2019/953
     */
    function prepare_evaluations(
        uint256 lin_poly_constant,
        Types.Proof memory proof,
        uint256[] memory buffer_v_and_uv_basis
    ) internal view returns (uint256 eval) {
        eval = Bn254Crypto.negate_fr(lin_poly_constant);
        uint256 p_local = Bn254Crypto.r_mod;
        assembly {
            for {
                let i := 0
            } lt(i, 11) {
                i := add(i, 1)
            } {
                eval := addmod(
                    eval,
                    mulmod(
                        mload(add(buffer_v_and_uv_basis, mul(add(i, 1), 0x20))),
                        mload(add(add(proof, 0x200), mul(i, 0x20))),
                        p_local
                    ),
                    p_local
                )
            }
        }
        eval = addmod(eval,mulmod(buffer_v_and_uv_basis[11],proof.perm_next_eval,p),p);        
        
        // for lookup
                eval = addmod(eval,mulmod(buffer_v_and_uv_basis[12],proof.range_table_eval,p),p);
                eval = addmod(eval,mulmod(buffer_v_and_uv_basis[13],proof.key_table_eval,p),p);
                eval = addmod(eval,mulmod(buffer_v_and_uv_basis[14],proof.h_1_eval,p),p);
                eval = addmod(eval,mulmod(buffer_v_and_uv_basis[15],proof.q_lookup_eval,p),p);
                eval = addmod(eval,mulmod(buffer_v_and_uv_basis[16],proof.table_dom_sep_eval,p),p);
                eval = addmod(eval,mulmod(buffer_v_and_uv_basis[17],proof.q_dom_sep_eval,p),p);
                eval = addmod(eval,mulmod(buffer_v_and_uv_basis[18],proof.prod_next_eval,p),p);
                eval = addmod(eval,mulmod(buffer_v_and_uv_basis[19],proof.range_table_next_eval,p),p);
                eval = addmod(eval,mulmod(buffer_v_and_uv_basis[20],proof.key_table_next_eval,p),p);
                eval = addmod(eval,mulmod(buffer_v_and_uv_basis[21],proof.h_1_next_eval,p),p);
                eval = addmod(eval,mulmod(buffer_v_and_uv_basis[22],proof.h_2_next_eval,p),p);
                eval = addmod(eval,mulmod(buffer_v_and_uv_basis[23],proof.q_lookup_next_eval,p),p);
                eval = addmod(eval,mulmod(buffer_v_and_uv_basis[24],proof.w_3_next_eval,p),p);
                eval = addmod(eval,mulmod(buffer_v_and_uv_basis[25],proof.w_4_next_eval,p),p);
                eval = addmod(eval,mulmod(buffer_v_and_uv_basis[26],proof.table_dom_sep_next_eval,p),p);

    }

    function validate_scalar_field(uint256 fr) internal pure {
        bool isValid;
        uint256 p_local = Bn254Crypto.r_mod;

        assembly {
            isValid := lt(fr, p_local)
        }
        require(isValid, "Error: Invalid Scalar Field (Bn254).");
    }

    // Validate all group points and scalar fields in the proof struct
    // Revert if any are invalid.
    // proof A Ultra Plonk proof from Jellyfish with 4 input wires
    function validate_proof(Types.Proof memory proof) internal pure {
        Bn254Crypto.validate_G1Point(proof.wires_poly_comms_1);
        Bn254Crypto.validate_G1Point(proof.wires_poly_comms_2);
        Bn254Crypto.validate_G1Point(proof.wires_poly_comms_3);
        Bn254Crypto.validate_G1Point(proof.wires_poly_comms_4);
        Bn254Crypto.validate_G1Point(proof.wires_poly_comms_5);
        Bn254Crypto.validate_G1Point(proof.wires_poly_comms_6);
        Bn254Crypto.validate_G1Point(proof.prod_perm_poly_comm);
        Bn254Crypto.validate_G1Point(proof.split_quot_poly_comms_1);
        Bn254Crypto.validate_G1Point(proof.split_quot_poly_comms_2);
        Bn254Crypto.validate_G1Point(proof.split_quot_poly_comms_3);
        Bn254Crypto.validate_G1Point(proof.split_quot_poly_comms_4);
        Bn254Crypto.validate_G1Point(proof.split_quot_poly_comms_5);
        Bn254Crypto.validate_G1Point(proof.split_quot_poly_comms_6);
        Bn254Crypto.validate_G1Point(proof.h_poly_comm_1);
        Bn254Crypto.validate_G1Point(proof.h_poly_comm_2);
        Bn254Crypto.validate_G1Point(proof.prod_lookup_poly_comm);

        Bn254Crypto.validate_scalar_field(proof.wires_evals_1);
        Bn254Crypto.validate_scalar_field(proof.wires_evals_2);
        Bn254Crypto.validate_scalar_field(proof.wires_evals_3);
        Bn254Crypto.validate_scalar_field(proof.wires_evals_4);
        Bn254Crypto.validate_scalar_field(proof.wires_evals_5);
        Bn254Crypto.validate_scalar_field(proof.wires_evals_6);
        Bn254Crypto.validate_scalar_field(proof.wire_sigma_evals_1);
        Bn254Crypto.validate_scalar_field(proof.wire_sigma_evals_2);
        Bn254Crypto.validate_scalar_field(proof.wire_sigma_evals_3);
        Bn254Crypto.validate_scalar_field(proof.wire_sigma_evals_4);
        Bn254Crypto.validate_scalar_field(proof.wire_sigma_evals_5);

        Bn254Crypto.validate_scalar_field(proof.perm_next_eval);
        Bn254Crypto.validate_scalar_field(proof.range_table_eval);
        Bn254Crypto.validate_scalar_field(proof.key_table_eval);
        Bn254Crypto.validate_scalar_field(proof.table_dom_sep_eval);
        Bn254Crypto.validate_scalar_field(proof.q_dom_sep_eval);
        Bn254Crypto.validate_scalar_field(proof.h_1_eval);
        Bn254Crypto.validate_scalar_field(proof.q_lookup_eval);
        Bn254Crypto.validate_scalar_field(proof.prod_next_eval);
        Bn254Crypto.validate_scalar_field(proof.range_table_next_eval);
        Bn254Crypto.validate_scalar_field(proof.key_table_next_eval);
        Bn254Crypto.validate_scalar_field(proof.table_dom_sep_next_eval);
        Bn254Crypto.validate_scalar_field(proof.h_1_next_eval);
        Bn254Crypto.validate_scalar_field(proof.h_2_next_eval);
        Bn254Crypto.validate_scalar_field(proof.q_lookup_next_eval);
        Bn254Crypto.validate_scalar_field(proof.w_3_next_eval);
        Bn254Crypto.validate_scalar_field(proof.w_4_next_eval);
        
        
        Bn254Crypto.validate_G1Point(proof.opening_proof);
        Bn254Crypto.validate_G1Point(proof.shifted_opening_proof);
    }
 function get_verification_key()
        internal
        pure
        returns (Types.VerificationKey memory)
    {
       return UltraPlonkVerificationKey.getVerificationKey();
    }

    function deserialize_proof(
        bytes calldata proofBytes
    ) internal pure returns (Types.Proof memory proof) {
        uint256 data_ptr;
        assembly {
        data_ptr := proofBytes.offset

        // Allocate memory for the Proof struct
        let proof_ptr := mload(0x40)
        mstore(0x40, add(proof, 0x5A0)) // advance free memory pointer by size of Types.Proof struct
        // Initialize each field in the struct to point to memory slots
        // Allocate G1Point structs (each 0x40 bytes) for each commitment and proof
        // proof := proof_ptr
        // wires_poly_comms (6)
        for { let i := 0 } lt(i, 16) { i := add(i, 1) } {
            let ptr := add(proof, mul(i, 0x20)) // G1Point* ptrs at proof[0x00, 0x20, ..., 0xa0]
            let g1 := mload(0x40)
            mstore(0x40, add(g1, 0x40))
            mstore(ptr, g1)
            mstore(g1, calldataload(data_ptr))
            mstore(add(g1, 0x20), calldataload(add(data_ptr, 0x20)))
            data_ptr := add(data_ptr, 0x40)
        }
        // from    uint256 wires_evals_1 to      uint256 w_4_next_eval; 
        for { let i := 0 } lt(i, 27) { i := add(i, 1) } {
            mstore(add(proof, add(0x200, mul(i, 0x20))), calldataload(data_ptr))
            data_ptr := add(data_ptr, 0x20)
        }
        //   G1Point opening_proof; and G1Point shifted_opening_proof;
        for { let i := 0 } lt(i, 2) { i := add(i, 1) } {
            let ptr := add(proof, add(0x560, mul(i, 0x20))) // proof[0x340, ..., 0x3a0]
            let g1 := mload(0x40)
            mstore(0x40, add(g1, 0x40))
            mstore(ptr, g1)
            mstore(g1, calldataload(data_ptr))
            mstore(add(g1, 0x20), calldataload(add(data_ptr, 0x20)))
            data_ptr := add(data_ptr, 0x40)
        }
        }
        return proof;
    }
}

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
        // appendMessage(self, abi.encodePacked(reverse_Endianness(fieldElement)));
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
        // swap bytes
        v =
            ((v &
                0xFF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00) >>
                8) |
            ((v &
                0x00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF) <<
                8);

        // swap 2-byte long pairs
        v =
            ((v &
                0xFFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000) >>
                16) |
            ((v &
                0x0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF) <<
                16);

        // swap 4-byte long pairs
        v =
            ((v &
                0xFFFFFFFF00000000FFFFFFFF00000000FFFFFFFF00000000FFFFFFFF00000000) >>
                32) |
            ((v &
                0x00000000FFFFFFFF00000000FFFFFFFF00000000FFFFFFFF00000000FFFFFFFF) <<
                32);

        // swap 8-byte long pairs
        v =
            ((v &
                0xFFFFFFFFFFFFFFFF0000000000000000FFFFFFFFFFFFFFFF0000000000000000) >>
                64) |
            ((v &
                0x0000000000000000FFFFFFFFFFFFFFFF0000000000000000FFFFFFFFFFFFFFFF) <<
                64);

        // swap 16-byte long pairs
        v = (v >> 128) | (v << 128);
    }

    function compute_vk_hash(Types.VerificationKey memory vk) internal pure returns (uint256 vk_hash) 
    {
        //vk_hash = keccak256(
        //    vk.sigma_comms,
        //    vk.selector_comms,
        //    vk.range_table_comm,
        //    vk.key_table_comm,
        //    vk.table_dom_sep_comm,
        //    vk.q_dom_sep_comm,
    assembly {
        let ptr := mload(0x40)
        let offset := ptr

        // Loop through sigma_comms_1 to sigma_comms_6
        for { let i := 0 } lt(i, 6) { i := add(i, 1) } {
            let g1ptr := mload(add(vk, add(0x40, mul(i, 0x20))))
            mstore(offset, mload(g1ptr))             // x
            mstore(add(offset, 0x20), mload(add(g1ptr, 0x20))) // y
            offset := add(offset, 0x40)
        }

        // selector_comms_1 to selector_comms_18
        for { let i := 0 } lt(i, 18) { i := add(i, 1) } {
            let g1ptr := mload(add(vk, add(0x100, mul(i, 0x20))))
            mstore(offset, mload(g1ptr))             // x
            mstore(add(offset, 0x20), mload(add(g1ptr, 0x20))) // y
            offset := add(offset, 0x40)
        }

        // range_table_comm, key_table_comm, table_dom_sep_comm, q_dom_sep_comm
        for { let i := 0 } lt(i, 4) { i := add(i, 1) } {
            let g1ptr := mload(add(vk, add(0x400, mul(i, 0x20))))
            mstore(offset, mload(g1ptr))             // x
            mstore(add(offset, 0x20), mload(add(g1ptr, 0x20))) // y
            offset := add(offset, 0x40)
        }

        let len := sub(offset, ptr)
        vk_hash := shr(0, keccak256(ptr, len))
        // jj, come back to make 21888242871839275222246405745257275088548364400416034343698204186575808495617 p
        vk_hash := mod(vk_hash, 21888242871839275222246405745257275088548364400416034343698204186575808495617)

    }
}
    function compute_challengs(
        TranscriptData memory self,
        Types.VerificationKey memory vk,
        Types.Proof memory proof,
        uint256  public_inputs_hash
    ) internal pure {
        // compute vk hash
        uint256 vk_hash = compute_vk_hash(vk);
        append_field_element(self, vk_hash);    
        append_field_element(self, public_inputs_hash);
        append_G1_element(self, proof.wires_poly_comms_1);
        append_G1_element(self, proof.wires_poly_comms_2);
        append_G1_element(self, proof.wires_poly_comms_3);
        append_G1_element(self, proof.wires_poly_comms_4);
        append_G1_element(self, proof.wires_poly_comms_5);
        append_G1_element(self, proof.wires_poly_comms_6);
        // tau:
        //    buf0=hash(transcript||0)
        //    buf1=hash(transcript||1)
        //    state=[buf0,buf1]
        //    tau = state[..48]
        Transcript.generate_tau_challenge(self);
        // self.transcript = Transcript.slice_from_index(self.transcript, 64);
        // beta:
        //    transcript:h_poly_comms
        //    buf0=hash(state||transcript-first 64 zeros||0)
        //    buf1=hash(state||transcript-first 64 zeros||1)
        //    state=[buf0,buf1]
        //    tau = state[..48]
        // self.challenges.beta = 
        Transcript.generate_beta_challenges(self, proof);
        //  gamma
        //    buf0=hash(state||transcript-first 64 zeros||0)
        //    buf1=hash(state||transcript-first 64 zeros||1)
        //    state=[buf0,buf1]
        //    tau = state[..48]
        self.challenges.gamma = Transcript.generate_gamma_challenges(self);
        // alpha
        //    transcript:prod_perm_poly_comm
        //    transcript:prod_lookup_poly_comm
        //    buf0=hash(state||transcript-first 64 zeros||0)
        //    buf1=hash(state||transcript-first 64 zeros||1)
        //    state=[buf0,buf1]
        //    tau = state[..48]
        self.challenges.alpha = Transcript.generate_alpha_challenges(
            self,
            proof
        );
        // zeta
        //    transcript:proof.split_quot_poly_comms
        //    buf0=hash(state||transcript-first 64 zeros||0)
        //    buf1=hash(state||transcript-first 64 zeros||1)
        //    state=[buf0,buf1]
        //    tau = state[..48]
        self.challenges.zeta = Transcript.generate_zeta_challenges(self, proof);
        // v
        //    transcript:proof.range_table_eval
        //    transcript:proof.h_1_eval
        //    transcript:proof.prod_next_eval
        //    transcript:proof.range_table_next_eval
        //    transcript:proof.h_1_next_eval
        //    transcript:proof.h_2_next_eval
        //    buf0=hash(state||transcript-first 64 zeros||0)
        //    buf1=hash(state||transcript-first 64 zeros||1)
        //    state=[buf0,buf1]
        //    tau = state[..48]
        self.challenges.v = Transcript.generate_v_challenges(self, proof);
         // u
        //    transcript:proof.opening_proof
        //    transcript:proof.shifted_opening_proof
        //    buf0=hash(state||transcript-first 64 zeros||0)
        //    buf1=hash(state||transcript-first 64 zeros||1)
        //    state=[buf0,buf1]
        //    tau = state[..48]
        self.challenges.u = Transcript.generate_u_challenges(self, proof);
    }

    function generate_initial_transcript(
        TranscriptData memory self,
        Types.VerificationKey memory vk
    ) internal pure {
        // append_field_element(self, vk.domain_size);
        // append_field_element(self, vk.num_inputs);
        //    compute transcript:
        //    KECCAK256_STATE_SIZE=64 [0;64]
        //    modulus_bit_size=254    [254, 0, 0, 0]
        //    domain_size             [1, 0, 0, 0, 0, 0, 0, 0]
        //                             8, 0, 0, 0, 0, 0, 0
        //    num_inputs              [1, 0, 0, 0, 0, 0, 0, 0]
        //                             1, 0, 0, 0, 0, 0, 0, 0  
        //    vk.k.iter k1-k5
        //    selector_comms selector_comms_1-selector_comms_13
        //    sigma_comms    sigma_comms_1-sigma_comms_5
        //    public inputs
        bytes memory transcript_con;
        assembly {
            let ptr := mload(0x40)
            mstore(ptr, 80) // length of our dynamic bytes array
            mstore(add(ptr, 36), 0) // clear the first 32 bytes (after the length field)
            mstore(add(ptr, 68), 0) // clear the second 32 bytes
            // the first 32 bytes are used to store the length of the array.
            // domain_size [8, 0, 0, 0, 0, 0, 0, 0]
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
        /*
        append_field_element(self, vk.k1);56570
        append_field_element(self, 0x1); 56561
        consider to hardcode here instead of hardcoding in the vk struct
         */
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
            // Calculate the total size of the data including padding for the word size
            let total_data_size := add(public_input_byte_length, 32)
            mstore(0x40, add(ptr, total_data_size))
            // Load the big-endian data into memory
            calldatacopy(add(ptr, 32), 68, public_input_byte_length)
            // Reverse byte order from big-endian to little-endian
            for {
                let i := 0
            } lt(i, public_input_byte_length) {
                i := add(i, 32)
            } {
                let chunk_start := add(ptr, add(32, i))
                let chunk_end := add(chunk_start, 31)

                for {
                    let j := 0
                } lt(j, 16) {
                    j := add(j, 1)
                } {
                    let tmp := mload(chunk_start)
                    mstore(chunk_start, mload(chunk_end))
                    mstore(chunk_end, tmp)
                    chunk_start := add(chunk_start, 1)
                    chunk_end := sub(chunk_end, 1)
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
    // Concatenate state and transcript
    // bytes memory input = abi.encodePacked(self.state[0], self.state[1], self.transcript);
    bytes memory input = abi.encodePacked(self.state[0], self.transcript);
    // Hash with keccak256
    bytes32 buf = keccak256(input);

    // Update state with the hash (like Rust: self.state.copy_from_slice(&buf))
    self.state[0] = buf;
    // self.state[1] = bytes32(0); // Rust clears transcript, so set second state to zero

    // Clear the transcript
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

        // Count the number of leading zeros
        assembly {
            let leadingZeros := 0

            for {

            } lt(i, length) {
                i := add(i, 32)
            } {
                leadingZeros := add(leadingZeros, iszero(mload(add(x, i))))
            }

            // Calculate the new length for x1
            x1 := mload(0x40)
            mstore(x1, sub(length, leadingZeros))

            // Copy the data from x to x1
            for {

            } lt(i, length) {
                i := add(i, 32)
            } {
                mstore(add(x1, add(32, i)), mload(add(x, add(32, i))))
            }

            // Set the new length for x1
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
            // Get the pointer to the start of the source data
            let srcPtr := add(data, 0x20) // Adding 0x20 skips the length of the data bytes array

            // Get the pointer to the start of the destination data
            let destPtr := add(result, 0x20) // Adding 0x20 skips the length of the result bytes array

            for {
                let i := 0
            } lt(i, length) {
                i := add(i, 1)
            } {
                // Load 32 bytes from srcPtr+i into a temporary variable
                let temp := mload(add(srcPtr, add(i, index)))

                // Store the loaded 32 bytes at destPtr+i
                mstore(add(destPtr, i), temp)
            }
        }
        return result;
    }


    function generate_beta_challenges(
    Transcript.TranscriptData memory self,
    Types.Proof memory proof
) internal pure {
    // 1. Append h_poly_comms to transcript (just like Rust)
    // Transcript.TranscriptData memory transcripts1;
    Transcript.append_G1_element(self, proof.h_poly_comm_1);
    Transcript.append_G1_element(self, proof.h_poly_comm_2);
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
    ) internal pure returns (uint256)  {
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
        //ProofEvaluations
        // wires_evals
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
    // plookup_proof: poly_evals (PlookupEvaluations)    
    
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

library Bn254Crypto {
    uint256 constant p_mod =
        21888242871839275222246405745257275088696311157297823662689037894645226208583;
    uint256 constant r_mod =
        21888242871839275222246405745257275088548364400416034343698204186575808495617;

    function scalarMul(
        Types.G1Point memory p,
        uint256 s
    ) internal view returns (Types.G1Point memory r) {
        uint256[3] memory input;
        input[0] = p.x;
        input[1] = p.y;
        input[2] = s;
        bool success;
        assembly {
            success := staticcall(sub(gas(), 2000), 7, input, 0x80, r, 0x60)
            // Use "invalid" to make gas estimation work
            switch success
            case 0 {
                revert(0, 0)
            }
        }
        require(success, "Bn254: scalar mul failed!");
    }

    function multiScalarMul(
        Types.G1Point[] memory bases,
        uint256[] memory scalars
    ) internal view returns (Types.G1Point memory r) {
        require(
            scalars.length == bases.length,
            "MSM error: length does not match"
        );

        r = scalarMul(bases[0], scalars[0]);
        for (uint256 i = 1; i < scalars.length; i++) {
            r = add(r, scalarMul(bases[i], scalars[i]));
        }
    }

    function negate_fr(uint256 fr) internal pure returns (uint256 res) {
        return r_mod - (fr % r_mod);
    }

    function negate_G1Point(
        Types.G1Point memory p
    ) internal pure returns (Types.G1Point memory) {
        if (isInfinity(p)) {
            return p;
        }
        return Types.G1Point(p.x, p_mod - (p.y % p_mod));
    }

    function isInfinity(
        Types.G1Point memory point
    ) internal pure returns (bool result) {
        assembly {
            let x := mload(point)
            let y := mload(add(point, 0x20))
            result := and(iszero(x), iszero(y))
        }
    }

    function add(
        Types.G1Point memory p1,
        Types.G1Point memory p2
    ) internal view returns (Types.G1Point memory r) {
        uint256[4] memory input;
        input[0] = p1.x;
        input[1] = p1.y;
        input[2] = p2.x;
        input[3] = p2.y;
        bool success;
        assembly {
            success := staticcall(sub(gas(), 2000), 6, input, 0xc0, r, 0x60)
            // Use "invalid" to make gas estimation work
            switch success
            case 0 {
                revert(0, 0)
            }
        }
        require(success, "Bn254: group addition failed!");
    }

    function fromLeBytesModOrder(
        bytes memory leBytes
    ) internal pure returns (uint256 ret) {
        assembly {
            let len := mload(leBytes)
            let byteData := add(leBytes, 0x20) // points to the start of byte data

            for {
                let i := 0
            } lt(i, len) {
                i := add(i, 1)
            } {
                // Multiply ret by 256 (shift left by one byte) with mod
                ret := mulmod(ret, 256, r_mod)

                // Fetch byte in little-endian order
                let byteVal := byte(
                    0,
                    mload(sub(sub(add(byteData, len), i), 1))
                )

                // Add the byte to ret with mod
                ret := addmod(ret, byteVal, r_mod)
            }
        }
    }

    function fromBeBytesModOrder(bytes memory beBytes) internal pure returns (uint256 ret) {
    assembly {
        let len := mload(beBytes)
        let byteData := add(beBytes, 0x20)
        for { let i := 0 } lt(i, len) { i := add(i, 1) } {
            ret := mulmod(ret, 256, r_mod)
            let byteVal := byte(0, mload(add(byteData, i)))
            ret := addmod(ret, byteVal, r_mod)
        }
    }
}


    function invert(uint256 fr) internal view returns (uint256) {
        uint256 output;
        bool success;
        uint256 p = r_mod;
        assembly {
            let mPtr := mload(0x40)
            mstore(mPtr, 0x20)
            mstore(add(mPtr, 0x20), 0x20)
            mstore(add(mPtr, 0x40), 0x20)
            mstore(add(mPtr, 0x60), fr)
            mstore(add(mPtr, 0x80), sub(p, 2))
            mstore(add(mPtr, 0xa0), p)
            success := staticcall(gas(), 0x05, mPtr, 0xc0, 0x00, 0x20)
            output := mload(0x00)
        }
        require(success, "pow precompile call failed!");
        return output;
    }

    /// Evaluate the following pairing product:
    /// e(a1, a2).e(-b1, b2) == 1
    function pairingProd2(
        Types.G1Point memory a1,
        Types.G2Point memory a2,
        Types.G1Point memory b1,
        Types.G2Point memory b2
    ) internal view returns (bool) {
        validate_G1Point(a1);
        validate_G1Point(b1);
        bool success;
        uint256 out;
        assembly {
            let mPtr := mload(0x40)
            mstore(mPtr, mload(a1))
            mstore(add(mPtr, 0x20), mload(add(a1, 0x20)))
            mstore(add(mPtr, 0x40), mload(a2))
            mstore(add(mPtr, 0x60), mload(add(a2, 0x20)))
            mstore(add(mPtr, 0x80), mload(add(a2, 0x40)))
            mstore(add(mPtr, 0xa0), mload(add(a2, 0x60)))

            mstore(add(mPtr, 0xc0), mload(b1))
            mstore(add(mPtr, 0xe0), mload(add(b1, 0x20)))
            mstore(add(mPtr, 0x100), mload(b2))
            mstore(add(mPtr, 0x120), mload(add(b2, 0x20)))
            mstore(add(mPtr, 0x140), mload(add(b2, 0x40)))
            mstore(add(mPtr, 0x160), mload(add(b2, 0x60)))
            success := staticcall(gas(), 8, mPtr, 0x180, 0x00, 0x20)
            out := mload(0x00)
        }
        require(success, "Pairing check failed!");
        return (out != 0);
    }

    /**
     * validate the following:
     *   x != 0
     *   y != 0
     *   x < p
     *   y < p
     *   y^2 = x^3 + 3 mod p
     */
    function validate_G1Point(Types.G1Point memory point) internal pure {
        bool is_well_formed;
        uint256 p = p_mod;
        assembly {
            let x := mload(point)
            let y := mload(add(point, 0x20))

            is_well_formed := and(
                and(and(lt(x, p), lt(y, p)), not(or(iszero(x), iszero(y)))),
                eq(mulmod(y, y, p), addmod(mulmod(x, mulmod(x, x, p), p), 3, p))
            )
        }
        require(
            is_well_formed,
            "Bn254: G1 point not on curve, or is malformed"
        );
    }

    function validate_scalar_field(uint256 fr) internal pure {
        bool isValid;
        assembly {
            isValid := lt(fr, r_mod)
        }
        require(isValid, "Bn254: invalid scalar field");
    }
}

library PolynomialEval {
    /// @dev a Radix 2 Evaluation Domain
    struct EvalDomain {
        uint256 size; // Size of the domain as a field element
        // uint256 logSize; // log_2(self.size)
        uint256 sizeInv; // Inverse of the size in the field
        uint256 groupGen; // A generator of the subgroup
        uint256 groupGenInv; // Inverse of the generator
    }

    /// @dev stores vanishing poly, lagrange at 1, and Public input poly
    struct EvalData {
        uint256 vanish_eval; //0x00
        uint256 lagrange_1_eval; //0x20
        uint256 piEval; //0x40
         uint256 lagrange_n_eval; //0x60
    }

    /// @dev compute the EvalData for a given domain and a challenge zeta
    function evalDataGen(
        EvalDomain memory self,
        uint256 zeta,
        uint256[] memory publicInput
    ) internal view returns (EvalData memory evalData) {
        evalData.vanish_eval = evaluate_VanishingPoly(self, zeta);
       (evalData.lagrange_1_eval, evalData.lagrange_n_eval) = evaluate_lagrange_1_n(
            self,
            zeta,
            evalData.vanish_eval
        );
        evalData.piEval = evaluate_PiPoly(
            self,
            publicInput,
            zeta,
            evalData.vanish_eval
        );
    }

    /// @dev Create a new Radix2EvalDomain with `domainSize` which should be power of 2.
    function new_EvalDomain(
        Types.VerificationKey memory vk
    ) internal pure returns (EvalDomain memory evalDomain) {
        // Note that this part is hardencoded based on the domainsize
        // Check this for the last rollup proof
        // get sizeInv, groupGen, groupGenInv from vk structure
      
           return   EvalDomain(
                // size
                vk.domain_size,
                // log size
                // 11,
                // sizeInv size_inv
                vk.size_inv,
                // groupGen
                vk.group_gen,
                // groupGenInv
                vk.group_gen_inv
            );
    }

    // This evaluates the vanishing polynomial for this domain at zeta.
    // For multiplicative subgroups, this polynomial is
    // `z(X) = X^self.size - 1`.
    function evaluate_VanishingPoly(
        EvalDomain memory self,
        uint256 zeta
    ) internal pure returns (uint256 res) {
        // zeta.pow([self.size() as u64]) - self.coset_offset_pow_size()

        uint256 p = Bn254Crypto.r_mod;
        uint256 size = self.size;
        res = power(zeta, size, p);
        res = res - 1;
    }

    function power(
        uint256 base,
        uint256 exponent,
        uint256 modulus
    ) internal pure returns (uint256) {
        uint256 result = 1;
        assembly {
            // Continue looping until exponent is zero
            for {

            } gt(exponent, 0) {

            } {
                // If the least significant bit of exponent is 1, multiply
                if and(exponent, 1) {
                    result := mulmod(result, base, modulus)
                }

                // Square the base
                base := mulmod(base, base, modulus)

                // Right shift the exponent by 1
                exponent := shr(1, exponent)
            }
        }
        return result;
    }

    /// @dev Evaluate the lagrange polynomial at point `zeta` given the vanishing polynomial evaluation `vanish_eval`.
    function evaluate_lagrange_1_n(
        EvalDomain memory self,
        uint256 zeta,
        uint256 vanish_eval
    ) internal view returns (uint256 lagrange_1_eval, uint256 lagrange_n_eval) {
        uint256 p = Bn254Crypto.r_mod;
        uint256 divisor1 = mulmod(self.size, (zeta - 1), p);
        divisor1 = Bn254Crypto.invert(divisor1);
        lagrange_1_eval = mulmod(vanish_eval, divisor1, p);

        uint256 divisor_n = mulmod(self.size,addmod(zeta, Bn254Crypto.negate_fr(self.groupGenInv),p) ,p);
        divisor_n = Bn254Crypto.invert(divisor_n);
        lagrange_n_eval =mulmod(vanish_eval, mulmod(self.groupGenInv, divisor_n, p),p);
    }

    /// @dev Evaluate public input polynomial at point `zeta`.
    function evaluate_PiPoly(
        EvalDomain memory self,
        uint256[] memory publicInput,
        uint256 zeta,
        uint256 vanish_eval
    ) internal view returns (uint256 res) {
        // If zeta is a root of the vanishing polynomial, directly return zero.
        if (vanish_eval == 0) {
            return 0;
        }

        uint256 p = Bn254Crypto.r_mod;
        uint256 length = publicInput.length;
        uint256 ithLagrange;
        uint256 ithDivisor;
        uint256 tmp;
        uint256 vanish_eval_div_n = mulmod(self.sizeInv, vanish_eval, p);

        uint256 divisorProd;
        uint256[] memory localdomain_elements = domain_elements(self, length);
        uint256[] memory divisors = new uint256[](length);

        assembly {
            divisorProd := 1

            for {
                let i := 0
            } lt(i, length) {
                i := add(i, 1)
            } {
                // tmp points to g^i
                // first 32 bytes of reference is the length of an array
                tmp := mload(add(add(localdomain_elements, 0x20), mul(i, 0x20)))
                // compute (zeta - g^i)
                ithDivisor := addmod(sub(p, tmp), zeta, p)
                // accumulate (zeta - g^i) to the divisorProd
                divisorProd := mulmod(divisorProd, ithDivisor, p)
                // store ithDivisor in the array
                mstore(add(add(divisors, 0x20), mul(i, 0x20)), ithDivisor)
            }
        }

        // compute 1 / \prod_{i=0}^length (zeta - g^i)
        divisorProd = Bn254Crypto.invert(divisorProd);

        assembly {
            for {
                let i := 0
            } lt(i, length) {
                i := add(i, 1)
            } {
                // tmp points to g^i
                // first 32 bytes of reference is the length of an array
                tmp := mload(add(add(localdomain_elements, 0x20), mul(i, 0x20)))
                // vanish_eval_div_n * g^i
                ithLagrange := mulmod(vanish_eval_div_n, tmp, p)

                // now we compute vanish_eval_div_n * g^i / (zeta - g^i) via
                // vanish_eval_div_n * g^i * divisorProd * \prod_{j!=i} (zeta - g^j)
                ithLagrange := mulmod(ithLagrange, divisorProd, p)
                for {
                    let j := 0
                } lt(j, length) {
                    j := add(j, 1)
                } {
                    if iszero(eq(i, j)) {
                        ithDivisor := mload(
                            add(add(divisors, 0x20), mul(j, 0x20))
                        )
                        ithLagrange := mulmod(ithLagrange, ithDivisor, p)
                    }
                }

                // multiply by pub_input[i] and update res
                // tmp points to public input
                tmp := mload(add(add(publicInput, 0x20), mul(i, 0x20)))
                ithLagrange := mulmod(ithLagrange, tmp, p)
                res := addmod(res, ithLagrange, p)
            }
        }
    }

    /// @dev Generate the domain elements for indexes 0..length
    /// which are essentially g^0, g^1, ..., g^{length-1}
    function domain_elements(
        EvalDomain memory self,
        uint256 length
    ) internal pure returns (uint256[] memory elements) {
        uint256 groupGen = self.groupGen;
        uint256 tmp = 1;
        uint256 p = Bn254Crypto.r_mod;
        elements = new uint256[](length);
        assembly {
            if not(iszero(length)) {
                let ptr := add(elements, 0x20)
                let end := add(ptr, mul(0x20, length))
                mstore(ptr, 1)
                ptr := add(ptr, 0x20)
                // for (; ptr < end; ptr += 32) loop through the memory of `elements`
                for {

                } lt(ptr, end) {
                    ptr := add(ptr, 0x20)
                } {
                    tmp := mulmod(tmp, groupGen, p)
                    mstore(ptr, tmp)
                }
            }
        }
    }
}
