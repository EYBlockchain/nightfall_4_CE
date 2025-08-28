// SPDX-License-Identifier: GPL-2.0-only
pragma solidity ^0.8.20;

import "./Types.sol";
import "./Bn254Crypto.sol";

library PolynomialEval {
    struct EvalDomain {
        uint256 size;
        uint256 sizeInv;
        uint256 groupGen;
        uint256 groupGenInv;
    }

    struct EvalData {
        uint256 vanish_eval;      // 0x00
        uint256 lagrange_1_eval;  // 0x20
        uint256 piEval;           // 0x40
        uint256 lagrange_n_eval;  // 0x60
    }

    function evalDataGen(
        EvalDomain memory self,
        uint256 zeta,
        uint256[] memory publicInput
    ) internal view returns (EvalData memory evalData) {
        evalData.vanish_eval = evaluate_VanishingPoly(self, zeta);
        (evalData.lagrange_1_eval, evalData.lagrange_n_eval) =
            evaluate_lagrange_1_n(self, zeta, evalData.vanish_eval);
        evalData.piEval = evaluate_PiPoly(self, publicInput, zeta, evalData.vanish_eval);
    }

    function new_EvalDomain(
        Types.VerificationKey memory vk
    ) internal pure returns (EvalDomain memory) {
        return EvalDomain({
            size: vk.domain_size,
            sizeInv: vk.size_inv,
            groupGen: vk.group_gen,
            groupGenInv: vk.group_gen_inv
        });
    }

    function evaluate_VanishingPoly(
        EvalDomain memory self,
        uint256 zeta
    ) internal pure returns (uint256 res) {
        uint256 p = Bn254Crypto.r_mod;
        res = power(zeta, self.size, p);
        res = res - 1;
    }

    function power(
        uint256 base,
        uint256 exponent,
        uint256 modulus
    ) internal pure returns (uint256) {
        uint256 result = 1;
        assembly {
            for { } gt(exponent, 0) { } {
                if and(exponent, 1) { result := mulmod(result, base, modulus) }
                base := mulmod(base, base, modulus)
                exponent := shr(1, exponent)
            }
        }
        return result;
    }

    function evaluate_lagrange_1_n(
        EvalDomain memory self,
        uint256 zeta,
        uint256 vanish_eval
    ) internal view returns (uint256 lagrange_1_eval, uint256 lagrange_n_eval) {
        uint256 p = Bn254Crypto.r_mod;
        uint256 divisor1 = mulmod(self.size, (zeta - 1), p);
        divisor1 = Bn254Crypto.invert(divisor1);
        lagrange_1_eval = mulmod(vanish_eval, divisor1, p);

        uint256 divisor_n = mulmod(self.size, addmod(zeta, Bn254Crypto.negate_fr(self.groupGenInv), p), p);
        divisor_n = Bn254Crypto.invert(divisor_n);
        lagrange_n_eval = mulmod(vanish_eval, mulmod(self.groupGenInv, divisor_n, p), p);
    }

    function evaluate_PiPoly(
        EvalDomain memory self,
        uint256[] memory publicInput,
        uint256 zeta,
        uint256 vanish_eval
    ) internal view returns (uint256 res) {
        if (vanish_eval == 0) return 0;

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
            for { let i := 0 } lt(i, length) { i := add(i, 1) } {
                tmp := mload(add(add(localdomain_elements, 0x20), mul(i, 0x20)))
                ithDivisor := addmod(sub(p, tmp), zeta, p)
                divisorProd := mulmod(divisorProd, ithDivisor, p)
                mstore(add(add(divisors, 0x20), mul(i, 0x20)), ithDivisor)
            }
        }

        divisorProd = Bn254Crypto.invert(divisorProd);

        assembly {
            for { let i := 0 } lt(i, length) { i := add(i, 1) } {
                tmp := mload(add(add(localdomain_elements, 0x20), mul(i, 0x20)))
                ithLagrange := mulmod(vanish_eval_div_n, tmp, p)
                ithLagrange := mulmod(ithLagrange, divisorProd, p)
                for { let j := 0 } lt(j, length) { j := add(j, 1) } {
                    if iszero(eq(i, j)) {
                        ithDivisor := mload(add(add(divisors, 0x20), mul(j, 0x20)))
                        ithLagrange := mulmod(ithLagrange, ithDivisor, p)
                    }
                }
                tmp := mload(add(add(publicInput, 0x20), mul(i, 0x20)))
                ithLagrange := mulmod(ithLagrange, tmp, p)
                res := addmod(res, ithLagrange, p)
            }
        }
    }

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
                for { } lt(ptr, end) { ptr := add(ptr, 0x20) } {
                    tmp := mulmod(tmp, groupGen, p)
                    mstore(ptr, tmp)
                }
            }
        }
    }
}
