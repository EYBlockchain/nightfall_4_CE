// SPDX-License-Identifier: GPL-2.0-only

pragma solidity >=0.8.20;
import "./lib/BytesLib.sol";
import "./lib/Types.sol";
import "./IVKProvider.sol";

import {INFVerifier} from "./INFVerifier.sol";

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {Transcript} from "./lib/Transcript.sol";
import {Bn254Crypto} from "./lib/Bn254Crypto.sol";
import {PolynomialEval} from "./lib/PolynomialEval.sol";
import ".././X509/Certified.sol";



/**
@title RollupProofVerifier
@dev Verifier Implementation for Nightfish Ultra plonk proof verification
*/

contract RollupProofVerifier is     
    INFVerifier,
    OwnableUpgradeable,
    UUPSUpgradeable
{
    IVKProvider public vkProvider;
    /**
        Calldata formatting:
        0x00 - 0x04 : function signature
        0x04 - 0x24 : proof_data pointer (location in calldata that contains the proof_data array)
        0x44 - 0x64 : length of `proof_data` array
        0x64 - ???? : array containing our zk proof data
    **/

    // Global r-modulus cached for mod ops
    uint256 public p;

    // constructor() {
    //     // disable initializers on the implementation
    //     _disableInitializers();
    // }
    function initialize(address vkProviderProxy, address initialOwner) public initializer {
        __Ownable_init(initialOwner);
        __UUPSUpgradeable_init();
        // __Certified_init(msg.sender, x509_address, sanctionsListAddress);

        p = Bn254Crypto.r_mod;
        vkProvider = IVKProvider(vkProviderProxy);
    }

    function _authorizeUpgrade(address) internal virtual override onlyOwner {}

    event VKProviderUpdated(address indexed oldProvider, address indexed newProvider);

    function setVKProvider(address newProvider) external onlyOwner {
        require(newProvider != address(0), "zero addr");
        emit VKProviderUpdated(address(vkProvider), newProvider);
        vkProvider = IVKProvider(newProvider);
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

    /**
     * @dev Verify a rollup proof and accumulators
     * @param acc_proof- array of serialized accumulators data: every elements is 32 bytes
     * @param proofBytes- array of serialized proof data: every elements is 32 bytes
     * @param publicInputsHashBytes- bytes of public data
     */
    function verify(
        bytes calldata acc_proof, 
        bytes calldata proofBytes, 
        bytes calldata publicInputsHashBytes
    ) external view override returns (bool result) {
        // parse the hardecoded vk and construct a vk object
        Types.VerificationKey memory vk = get_verification_key();
        // parse the second part of calldata to get public input
        // we hashed all public inputs into a single value
        uint256 public_inputs_hash;

        assembly {
            public_inputs_hash := calldataload(add(publicInputsHashBytes.offset, 0))
        }

        // parse the input calldata and construct a proof object and public_inputs
        Types.Proof memory decoded_proof = deserialize_proof(proofBytes);
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
        view
        returns (Types.VerificationKey memory)
    {
       return vkProvider.getVerificationKey();
    //    return UltraPlonkVerificationKey.getVerificationKey();
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

    // storage gap for future variables
    uint256[50] private __gap;
}

