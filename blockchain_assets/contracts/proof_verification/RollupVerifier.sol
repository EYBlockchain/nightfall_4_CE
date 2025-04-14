// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {UltraVK as VerifyingKey} from "./VerificationKey.sol";
import {INFVerifier} from "./INFVerifier.sol";

contract RollupVerifier is INFVerifier {
    // Memory locations for verification key parts
    uint256 internal constant DOMAIN_SIZE = 0x400;
    uint256 internal constant NUM_PUBLIC_INPUTS = 0x420;
    uint256 internal constant SIGMA1_X = 0x440;
    uint256 internal constant SIGMA1_Y = 0x460;
    uint256 internal constant SIGMA2_X = 0x480;
    uint256 internal constant SIGMA2_Y = 0x4A0;
    uint256 internal constant SIGMA3_X = 0x4C0;
    uint256 internal constant SIGMA3_Y = 0x4E0;
    uint256 internal constant SIGMA4_X = 0x500;
    uint256 internal constant SIGMA4_Y = 0x520;
    uint256 internal constant SIGMA5_X = 0x540;
    uint256 internal constant SIGMA5_Y = 0x560;
    uint256 internal constant SIGMA6_X = 0x580;
    uint256 internal constant SIGMA6_Y = 0x5A0;
    uint256 internal constant QLC1_X = 0x5C0;
    uint256 internal constant QLC1_Y = 0x5E0;
    uint256 internal constant QLC2_X = 0x600;
    uint256 internal constant QLC2_Y = 0x620;
    uint256 internal constant QLC3_X = 0x640;
    uint256 internal constant QLC3_Y = 0x660;
    uint256 internal constant QLC4_X = 0x680;
    uint256 internal constant QLC4_Y = 0x6A0;
    uint256 internal constant QMUL1_X = 0x6C0;
    uint256 internal constant QMUL1_Y = 0x6E0;
    uint256 internal constant QMUL2_X = 0x700;
    uint256 internal constant QMUL2_Y = 0x720;
    uint256 internal constant QHASH1_X = 0x740;
    uint256 internal constant QHASH1_Y = 0x760;
    uint256 internal constant QHASH2_X = 0x780;
    uint256 internal constant QHASH2_Y = 0x7A0;
    uint256 internal constant QHASH3_X = 0x7C0;
    uint256 internal constant QHASH3_Y = 0x7E0;
    uint256 internal constant QHASH4_X = 0x800;
    uint256 internal constant QHASH4_Y = 0x820;
    uint256 internal constant QOUT_X = 0x840;
    uint256 internal constant QOUT_Y = 0x860;
    uint256 internal constant QCONST_X = 0x880;
    uint256 internal constant QCONST_Y = 0x8A0;
    uint256 internal constant QECC_X = 0x8C0;
    uint256 internal constant QECC_Y = 0x8E0;
    uint256 internal constant QSWX1_X = 0x900;
    uint256 internal constant QSWX1_Y = 0x920;
    uint256 internal constant QSWX2_X = 0x940;
    uint256 internal constant QSWX2_Y = 0x960;
    uint256 internal constant QSWY1_X = 0x980;
    uint256 internal constant QSWY1_Y = 0x9A0;
    uint256 internal constant QSWY2_X = 0x9C0;
    uint256 internal constant QSWY2_Y = 0x9E0;
    uint256 internal constant QLOOKUP_X = 0xA00;
    uint256 internal constant QLOOKUP_Y = 0xA20;
    uint256 internal constant K1 = 0xA40;
    uint256 internal constant K2 = 0xA60;
    uint256 internal constant K3 = 0xA80;
    uint256 internal constant K4 = 0xAA0;
    uint256 internal constant K5 = 0xAC0;
    uint256 internal constant K6 = 0xAE0;
    uint256 internal constant OKG1_X = 0xB00;
    uint256 internal constant OKG1_Y = 0xB20;
    uint256 internal constant OKG2_X1 = 0xB40;
    uint256 internal constant OKG2_X2 = 0xB60;
    uint256 internal constant OKG2_Y1 = 0xB80;
    uint256 internal constant OKG2_Y2 = 0xBA0;
    uint256 internal constant OKG3_X1 = 0xBC0;
    uint256 internal constant OKG3_X2 = 0xBE0;
    uint256 internal constant OKG3_Y1 = 0xC00;
    uint256 internal constant OKG3_Y2 = 0xC20;
    uint256 internal constant PLRANGE_X = 0xC40;
    uint256 internal constant PLRANGE_Y = 0xC60;
    uint256 internal constant PLKEY_X = 0xC80;
    uint256 internal constant PLKEY_Y = 0xCA0;
    uint256 internal constant PLTDS_X = 0xCC0;
    uint256 internal constant PLTDS_Y = 0xCE0;
    uint256 internal constant QDOMSEP_X = 0xD00;
    uint256 internal constant QDOMSEP_Y = 0xD20;

    // Memory locations for proof parts first all the commitments
    uint256 internal constant W1_X = 0xD40;
    uint256 internal constant W1_Y = 0xD60;
    uint256 internal constant W2_X = 0xD80;
    uint256 internal constant W2_Y = 0xDA0;
    uint256 internal constant W3_X = 0xDC0;
    uint256 internal constant W3_Y = 0xDE0;
    uint256 internal constant W4_X = 0xE00;
    uint256 internal constant W4_Y = 0xE20;
    uint256 internal constant W5_X = 0xE40;
    uint256 internal constant W5_Y = 0xE60;
    uint256 internal constant W6_X = 0xE80;
    uint256 internal constant W6_Y = 0xEA0;
    uint256 internal constant PERM_X = 0xEC0;
    uint256 internal constant PERM_Y = 0xEE0;
    uint256 internal constant QUOT1_X = 0xF00;
    uint256 internal constant QUOT1_Y = 0xF20;
    uint256 internal constant QUOT2_X = 0xF40;
    uint256 internal constant QUOT2_Y = 0xF60;
    uint256 internal constant QUOT3_X = 0xF80;
    uint256 internal constant QUOT3_Y = 0xFA0;
    uint256 internal constant QUOT4_X = 0xFC0;
    uint256 internal constant QUOT4_Y = 0xFE0;
    uint256 internal constant QUOT5_X = 0x1000;
    uint256 internal constant QUOT5_Y = 0x1020;
    uint256 internal constant QUOT6_X = 0x1040;
    uint256 internal constant QUOT6_Y = 0x1060;
    uint256 internal constant H1_X = 0x1080;
    uint256 internal constant H1_Y = 0x10A0;
    uint256 internal constant H2_X = 0x10C0;
    uint256 internal constant H2_Y = 0x10E0;
    uint256 internal constant LOOKUP_PERM_X = 0x1100;
    uint256 internal constant LOOKUP_PERM_Y = 0x1120;

    // Memory locations of the proof evaluations
    uint256 internal constant W1_EVAL = 0x1140;
    uint256 internal constant W2_EVAL = 0x1160;
    uint256 internal constant W3_EVAL = 0x1180;
    uint256 internal constant W4_EVAL = 0x11A0;
    uint256 internal constant W5_EVAL = 0x11C0;
    uint256 internal constant W6_EVAL = 0x11E0;
    uint256 internal constant SIGMA1_EVAL = 0x1200;
    uint256 internal constant SIGMA2_EVAL = 0x1220;
    uint256 internal constant SIGMA3_EVAL = 0x1240;
    uint256 internal constant SIGMA4_EVAL = 0x1260;
    uint256 internal constant SIGMA5_EVAL = 0x1280;
    uint256 internal constant PERM_EVAL = 0x12A0;
    uint256 internal constant RANGE_EVAL = 0x12C0;
    uint256 internal constant KEY_EVAL = 0x12E0;
    uint256 internal constant TDS_EVAL = 0x1300;
    uint256 internal constant QDOMSEP_EVAL = 0x1320;
    uint256 internal constant H1_EVAL = 0x1340;
    uint256 internal constant Q_LOOKUP_EVAL = 0x1360;
    uint256 internal constant LOOKUP_PERM_EVAL = 0x1380;
    uint256 internal constant RANGE_NEXT_EVAL = 0x13A0;
    uint256 internal constant KEY_NEXT_EVAL = 0x13C0;
    uint256 internal constant TDS_NEXT_EVAL = 0x13E0;
    uint256 internal constant H1_NEXT_EVAL = 0x1400;
    uint256 internal constant H2_NEXT_EVAL = 0x1420;
    uint256 internal constant Q_LOOKUP_NEXT_EVAL = 0x1440;
    uint256 internal constant W3_NEXT_EVAL = 0x1460;
    uint256 internal constant W4_NEXT_EVAL = 0x1480;

    // Opening proof memory locations
    uint256 internal constant OPENING1_X = 0x14A0;
    uint256 internal constant OPENING1_Y = 0x14C0;
    uint256 internal constant OPENING2_X = 0x14E0;
    uint256 internal constant OPENING2_Y = 0x1500;

    // Transcript challenge locations
    uint256 internal constant TAU = 0x1520;
    uint256 internal constant BETA = 0x1540;
    uint256 internal constant GAMMA = 0x1560;
    uint256 internal constant ALPHA = 0x1580;
    uint256 internal constant ZETA = 0x15A0;
    uint256 internal constant CHALLENGE_V = 0x15C0;
    uint256 internal constant CHALLENGE_U = 0x15E0;

    uint256 internal constant PUBLIC_INPUTS_HASH_LOCATION = 0x1600;

    // Memory locations for other variables
    uint256 internal constant N_INV_LOCATION = 0x2000;
    uint256 internal constant OMEGA_LOCATION = 0x2020;
    uint256 internal constant OMEGA_INVERSE_LOCATION = 0x2040;
    uint256 internal constant ZETA_POW_N = 0x2060;
    uint256 internal constant LAGRANGE_1_EVAL = 0x2080;
    uint256 internal constant LAGRANGE_N_EVAL = 0x20A0;
    uint256 internal constant VANISH_EVAL = 0x20C0;
    uint256 internal constant PI_EVAL = 0x20E0;
    uint256 internal constant LIN_POLY_CONST = 0x2100;
    uint256 internal constant MERGED_LW_EVAL = 0x2120;
    uint256 internal constant MERGED_TABLE_EVAL = 0x2140;
    uint256 internal constant MERGED_TABLE_NEXT_EVAL = 0x2160;
    uint256 internal constant ACC_X = 0x2180;
    uint256 internal constant ACC_Y = 0x21A0;
    uint256 internal constant ACC_SPACE_X = 0x21C0;
    uint256 internal constant ACC_SPACE_Y = 0x21E0;
    uint256 internal constant ACC_NEXT_X = 0x2200;
    uint256 internal constant ACC_NEXT_Y = 0x2220;
    uint256 internal constant PI_HASH = 0x2280;
    uint256 internal constant ACC_INSTANCE1_X = 0x22A0;
    uint256 internal constant ACC_INSTANCE1_Y = 0x22C0;
    uint256 internal constant ACC_PROOF1_X = 0x22E0;
    uint256 internal constant ACC_PROOF1_Y = 0x2300;
    uint256 internal constant ACC_INSTANCE2_X = 0x2320;
    uint256 internal constant ACC_INSTANCE2_Y = 0x2340;
    uint256 internal constant ACC_PROOF2_X = 0x2360;
    uint256 internal constant ACC_PROOF2_Y = 0x2380;

    bytes4 internal constant INVALID_VERIFICATION_KEY_SELECTOR = 0x7e5769bf;
    bytes4 internal constant MOD_EXP_FAILURE_SELECTOR = 0xf894a7bc;
    bytes4 internal constant POINT_NOT_ON_CURVE_SELECTOR = 0xa3dad654;
    bytes4 internal constant PAIRING_FAILED_SELECTOR = 0xd71fd263;
    bytes4 internal constant INVALID_COORDS_SELECTOR = 0xcf757e2e;

    error PUBLIC_INPUT_COUNT_INVALID(uint256 expected, uint256 actual);
    error INVALID_COORDS(bytes32 x, bytes32 y);
    error INVALID_VERIFICATION_KEY();
    error MOD_EXP_FAILURE();
    error POINT_NOT_ON_CURVE();
    error PAIRING_FAILED();
    error INVALID_VERIFICATION_KEY_HASH(uint256 expected, uint256 actual);

    function loadVerificationKey(
        uint256 _vk,
        uint256 _second_loc
    ) internal pure {
        VerifyingKey.loadVerificationKey(_vk, _second_loc);
    }

    constructor() {
        loadVerificationKey(DOMAIN_SIZE, N_INV_LOCATION);
        bytes32 vkHash;
        assembly {
            let
                q
            := 21888242871839275222246405745257275088696311157297823662689037894645226208583 // EC group order
            let
                p
            := 21888242871839275222246405745257275088548364400416034343698204186575808495617 // Prime field order

            let success := 1

            // VALIDATE SIGMA1
            {
                let x := mload(SIGMA1_X)
                let y := mload(SIGMA1_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE SIGMA2
            {
                let x := mload(SIGMA2_X)
                let y := mload(SIGMA2_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE SIGMA3
            {
                let x := mload(SIGMA3_X)
                let y := mload(SIGMA3_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE SIGMA4
            {
                let x := mload(SIGMA4_X)
                let y := mload(SIGMA4_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE SIGMA5
            {
                let x := mload(SIGMA5_X)
                let y := mload(SIGMA5_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE SIGMA6
            {
                let x := mload(SIGMA6_X)
                let y := mload(SIGMA6_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE QLC1
            {
                let x := mload(QLC1_X)
                let y := mload(QLC1_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE QLC2
            {
                let x := mload(QLC2_X)
                let y := mload(QLC2_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE QLC3
            {
                let x := mload(QLC3_X)
                let y := mload(QLC3_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE QLC4
            {
                let x := mload(QLC4_X)
                let y := mload(QLC4_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE QMUL1
            {
                let x := mload(QMUL1_X)
                let y := mload(QMUL1_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE QMUL2
            {
                let x := mload(QMUL2_X)
                let y := mload(QMUL2_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE QHASH1
            {
                let x := mload(QHASH1_X)
                let y := mload(QHASH1_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE QHASH2
            {
                let x := mload(QHASH2_X)
                let y := mload(QHASH2_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE QHASH3
            {
                let x := mload(QHASH3_X)
                let y := mload(QHASH3_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE QHASH4
            {
                let x := mload(QHASH4_X)
                let y := mload(QHASH4_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE QOUT
            {
                let x := mload(QOUT_X)
                let y := mload(QOUT_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE QCONST
            {
                let x := mload(QCONST_X)
                let y := mload(QCONST_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE QECC
            {
                let x := mload(QECC_X)
                let y := mload(QECC_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE QSWX1
            {
                let x := mload(QSWX1_X)
                let y := mload(QSWX1_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE QSWX2
            {
                let x := mload(QSWX2_X)
                let y := mload(QSWX2_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE QSWY1
            {
                let x := mload(QSWY1_X)
                let y := mload(QSWY1_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE QSWY2
            {
                let x := mload(QSWY2_X)
                let y := mload(QSWY2_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE QLOOKUP
            {
                let x := mload(QLOOKUP_X)
                let y := mload(QLOOKUP_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE PLRANGE
            {
                let x := mload(PLRANGE_X)
                let y := mload(PLRANGE_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE PLKEY
            {
                let x := mload(PLKEY_X)
                let y := mload(PLKEY_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE PLTDS
            {
                let x := mload(PLTDS_X)
                let y := mload(PLTDS_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            // VALIDATE QDOMSEP
            {
                let x := mload(QDOMSEP_X)
                let y := mload(QDOMSEP_Y)
                let xx := mulmod(x, x, q)
                // validate on curve
                success := and(
                    success,
                    eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                )
            }

            if iszero(success) {
                mstore(0x0, INVALID_VERIFICATION_KEY_SELECTOR)
                revert(0x00, 0x04)
            }

            // Calculate and set the Verifying Key hash
            {
                let ptr := add(mload(0x40), 0x20)
                mcopy(ptr, SIGMA1_X, 0x600)
                mcopy(add(ptr, 0x600), PLRANGE_X, 0x100)
                vkHash := mod(keccak256(ptr, 0x700), p)
            }
        }

        if (vkHash != getVerificationKeyHash()) {
            revert INVALID_VERIFICATION_KEY_HASH(
                uint256(vkHash),
                uint256(getVerificationKeyHash())
            );
        }
    }

    function getVerificationKeyHash() internal pure returns (bytes32) {
        return VerifyingKey.getVerificationKeyHash();
    }

    /**
     * @notice Verify a Ultra Plonk proof
     * @param - The serialized proof
     * @param - An array of the public inputs
     * @return True if proof is valid, reverts otherwise
     */
    function verify(
        bytes calldata,
        bytes32[] calldata
    ) external view override returns (bool) {
        loadVerificationKey(DOMAIN_SIZE, N_INV_LOCATION);
        bytes32 vkHash = getVerificationKeyHash();
        uint256 requiredPublicInputCount;

        assembly {
            requiredPublicInputCount := mload(NUM_PUBLIC_INPUTS)
        }

        assembly {
            let
                q
            := 21888242871839275222246405745257275088696311157297823662689037894645226208583 // EC group order
            let
                p
            := 21888242871839275222246405745257275088548364400416034343698204186575808495617 // Prime field order

            /**
             * LOAD PROOF FROM CALLDATA
             */
            {
                let data_ptr := add(calldataload(0x04), 0x24)
                let success := 1
                mstore(W1_X, mod(calldataload(data_ptr), q))
                mstore(W1_Y, mod(calldataload(add(data_ptr, 0x20)), q))
                {
                    let x := mload(W1_X)
                    let y := mload(W1_Y)
                    let xx := mulmod(x, x, q)
                    // validate on curve
                    success := and(
                        success,
                        eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                    )
                }
                // G1Point wires_poly_comms_2;
                mstore(W2_X, mod(calldataload(add(data_ptr, 0x40)), q))
                mstore(W2_Y, mod(calldataload(add(data_ptr, 0x60)), q))
                {
                    let x := mload(W2_X)
                    let y := mload(W2_Y)
                    let xx := mulmod(x, x, q)
                    // validate on curve
                    success := and(
                        success,
                        eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                    )
                }
                // G1Point wires_poly_comms_3;
                mstore(W3_X, mod(calldataload(add(data_ptr, 0x80)), q))
                mstore(W3_Y, mod(calldataload(add(data_ptr, 0xa0)), q))
                {
                    let x := mload(W3_X)
                    let y := mload(W3_Y)
                    let xx := mulmod(x, x, q)
                    // validate on curve
                    success := and(
                        success,
                        eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                    )
                }
                // G1Point wires_poly_comms_4;
                mstore(W4_X, mod(calldataload(add(data_ptr, 0xc0)), q))
                mstore(W4_Y, mod(calldataload(add(data_ptr, 0xe0)), q))
                {
                    let x := mload(W4_X)
                    let y := mload(W4_Y)
                    let xx := mulmod(x, x, q)
                    // validate on curve
                    success := and(
                        success,
                        eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                    )
                }
                // G1Point wires_poly_comms_5;
                mstore(W5_X, mod(calldataload(add(data_ptr, 0x100)), q))
                mstore(W5_Y, mod(calldataload(add(data_ptr, 0x120)), q))
                {
                    let x := mload(W5_X)
                    let y := mload(W5_Y)
                    let xx := mulmod(x, x, q)
                    // validate on curve
                    success := and(
                        success,
                        eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                    )
                }
                // G1Point wires_poly_comms_6;
                mstore(W6_X, mod(calldataload(add(data_ptr, 0x140)), q))
                mstore(W6_Y, mod(calldataload(add(data_ptr, 0x160)), q))
                {
                    let x := mload(W6_X)
                    let y := mload(W6_Y)
                    let xx := mulmod(x, x, q)
                    // validate on curve
                    success := and(
                        success,
                        eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                    )
                }
                // G1Point prod_perm_poly_comm;
                mstore(PERM_X, mod(calldataload(add(data_ptr, 0x180)), q))
                mstore(PERM_Y, mod(calldataload(add(data_ptr, 0x1A0)), q))
                {
                    let x := mload(PERM_X)
                    let y := mload(PERM_Y)
                    let xx := mulmod(x, x, q)
                    // validate on curve
                    success := and(
                        success,
                        eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                    )
                }
                // // split_quot_poly_comms
                // G1Point split_quot_poly_comms_1;
                mstore(QUOT1_X, mod(calldataload(add(data_ptr, 0x1C0)), q))
                mstore(QUOT1_Y, mod(calldataload(add(data_ptr, 0x1E0)), q))
                // VALIDATE QUOT
                {
                    let x := mload(QUOT1_X)
                    let y := mload(QUOT1_Y)
                    let xx := mulmod(x, x, q)
                    // validate on curve
                    success := and(
                        success,
                        eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                    )
                }
                // G1Point split_quot_poly_comms_2;
                mstore(QUOT2_X, mod(calldataload(add(data_ptr, 0x200)), q))
                mstore(QUOT2_Y, mod(calldataload(add(data_ptr, 0x220)), q))
                {
                    let x := mload(QUOT2_X)
                    let y := mload(QUOT2_Y)
                    let xx := mulmod(x, x, q)
                    // validate on curve
                    success := and(
                        success,
                        eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                    )
                }
                // G1Point split_quot_poly_comms_3;
                mstore(QUOT3_X, mod(calldataload(add(data_ptr, 0x240)), q))
                mstore(QUOT3_Y, mod(calldataload(add(data_ptr, 0x260)), q))
                {
                    let x := mload(QUOT3_X)
                    let y := mload(QUOT3_Y)
                    let xx := mulmod(x, x, q)
                    // validate on curve
                    success := and(
                        success,
                        eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                    )
                }
                // G1Point split_quot_poly_comms_4;
                mstore(QUOT4_X, mod(calldataload(add(data_ptr, 0x280)), q))
                mstore(QUOT4_Y, mod(calldataload(add(data_ptr, 0x2A0)), q))
                {
                    let x := mload(QUOT4_X)
                    let y := mload(QUOT4_Y)
                    let xx := mulmod(x, x, q)
                    // validate on curve
                    success := and(
                        success,
                        eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                    )
                }
                // G1Point split_quot_poly_comms_5;
                mstore(QUOT5_X, mod(calldataload(add(data_ptr, 0x2C0)), q))
                mstore(QUOT5_Y, mod(calldataload(add(data_ptr, 0x2E0)), q))
                {
                    let x := mload(QUOT5_X)
                    let y := mload(QUOT5_Y)
                    let xx := mulmod(x, x, q)
                    // validate on curve
                    success := and(
                        success,
                        eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                    )
                }
                // G1Point split_quot_poly_comms_6;
                mstore(QUOT6_X, mod(calldataload(add(data_ptr, 0x300)), q))
                mstore(QUOT6_Y, mod(calldataload(add(data_ptr, 0x320)), q))
                {
                    let x := mload(QUOT6_X)
                    let y := mload(QUOT6_Y)
                    let xx := mulmod(x, x, q)
                    // validate on curve
                    success := and(
                        success,
                        eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                    )
                }

                // G1Point h_poly_comm_1;
                mstore(H1_X, mod(calldataload(add(data_ptr, 0x340)), q))
                mstore(H1_Y, mod(calldataload(add(data_ptr, 0x360)), q))
                {
                    let x := mload(H1_X)
                    let y := mload(H1_Y)
                    let xx := mulmod(x, x, q)
                    // validate on curve
                    success := and(
                        success,
                        eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                    )
                }
                // G1Point h_poly_comm_2;
                mstore(H2_X, mod(calldataload(add(data_ptr, 0x380)), q))
                mstore(H2_Y, mod(calldataload(add(data_ptr, 0x3a0)), q))
                {
                    let x := mload(H2_X)
                    let y := mload(H2_Y)
                    let xx := mulmod(x, x, q)
                    // validate on curve
                    success := and(
                        success,
                        eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                    )
                }
                // G1Point prod_lookup_poly_comm
                mstore(
                    LOOKUP_PERM_X,
                    mod(calldataload(add(data_ptr, 0x3c0)), q)
                )
                mstore(
                    LOOKUP_PERM_Y,
                    mod(calldataload(add(data_ptr, 0x3e0)), q)
                )
                {
                    let x := mload(LOOKUP_PERM_X)
                    let y := mload(LOOKUP_PERM_Y)
                    let xx := mulmod(x, x, q)
                    // validate on curve
                    success := and(
                        success,
                        eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                    )
                }

                // wires_evals
                // uint256 wires_evals_1;
                mstore(W1_EVAL, mod(calldataload(add(data_ptr, 0x400)), p))
                // uint256 wires_evals_2;
                mstore(W2_EVAL, mod(calldataload(add(data_ptr, 0x420)), p))
                // uint256 wires_evals_3;
                mstore(W3_EVAL, mod(calldataload(add(data_ptr, 0x440)), p))
                // uint256 wires_evals_4;
                mstore(W4_EVAL, mod(calldataload(add(data_ptr, 0x460)), p))
                // uint256 wires_evals_5;
                mstore(W5_EVAL, mod(calldataload(add(data_ptr, 0x480)), p))
                // uint256 wires_evals_6;
                mstore(W6_EVAL, mod(calldataload(add(data_ptr, 0x4A0)), p))
                // wire_sigma_evals
                // uint256 wire_sigma_evals_1;
                mstore(SIGMA1_EVAL, mod(calldataload(add(data_ptr, 0x4C0)), p))
                // uint256 wire_sigma_evals_2;
                mstore(SIGMA2_EVAL, mod(calldataload(add(data_ptr, 0x4E0)), p))
                // uint256 wire_sigma_evals_3;
                mstore(SIGMA3_EVAL, mod(calldataload(add(data_ptr, 0x500)), p))
                // uint256 wire_sigma_evals_4;
                mstore(SIGMA4_EVAL, mod(calldataload(add(data_ptr, 0x520)), p))
                // uint256 wire_sigma_evals_5;
                mstore(SIGMA5_EVAL, mod(calldataload(add(data_ptr, 0x540)), p))
                // uint256 perm_next_eval;
                mstore(PERM_EVAL, mod(calldataload(add(data_ptr, 0x560)), p))

                // uint256 range_table_eval;
                mstore(RANGE_EVAL, mod(calldataload(add(data_ptr, 0x580)), p))
                // uint256 key_table_eval;
                mstore(KEY_EVAL, mod(calldataload(add(data_ptr, 0x5A0)), p))
                // uint256 table_dom_sep_eval;
                mstore(TDS_EVAL, mod(calldataload(add(data_ptr, 0x5C0)), p))
                // uint256 q_dom_sep_eval;
                mstore(QDOMSEP_EVAL, mod(calldataload(add(data_ptr, 0x5E0)), p))
                // uint256 h_1_eval;
                mstore(H1_EVAL, mod(calldataload(add(data_ptr, 0x600)), p))
                // uint256 q_lookup_eval;
                mstore(
                    Q_LOOKUP_EVAL,
                    mod(calldataload(add(data_ptr, 0x620)), p)
                )
                // uint256 prod_next_eval;
                mstore(
                    LOOKUP_PERM_EVAL,
                    mod(calldataload(add(data_ptr, 0x640)), p)
                )
                // uint256 range_table_next_eval;
                mstore(
                    RANGE_NEXT_EVAL,
                    mod(calldataload(add(data_ptr, 0x660)), p)
                )
                // uint256 key_table_next_eval;
                mstore(
                    KEY_NEXT_EVAL,
                    mod(calldataload(add(data_ptr, 0x680)), p)
                )
                // uint256 table_dom_sep_next_eval;
                mstore(
                    TDS_NEXT_EVAL,
                    mod(calldataload(add(data_ptr, 0x6A0)), p)
                )
                // uint256 h_1_next_eval;
                mstore(H1_NEXT_EVAL, mod(calldataload(add(data_ptr, 0x6C0)), p))
                // uint256 h_2_next_eval;
                mstore(H2_NEXT_EVAL, mod(calldataload(add(data_ptr, 0x6E0)), p))
                // uint256 q_lookup_next_eval;
                mstore(
                    Q_LOOKUP_NEXT_EVAL,
                    mod(calldataload(add(data_ptr, 0x700)), p)
                )
                // uint256 w_3_next_eval;
                mstore(W3_NEXT_EVAL, mod(calldataload(add(data_ptr, 0x720)), p))
                // uint256 w_4_next_eval;
                mstore(W4_NEXT_EVAL, mod(calldataload(add(data_ptr, 0x740)), p))
                // opening_proof
                // G1Point opening_proof;
                mstore(OPENING1_X, mod(calldataload(add(data_ptr, 0x760)), q))
                mstore(OPENING1_Y, mod(calldataload(add(data_ptr, 0x780)), q))
                {
                    let x := mload(OPENING1_X)
                    let y := mload(OPENING1_Y)
                    let xx := mulmod(x, x, q)
                    // validate on curve
                    success := and(
                        success,
                        eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                    )
                }
                // shifted_opening_proof
                // G1Point shifted_opening_proof;
                mstore(OPENING2_X, mod(calldataload(add(data_ptr, 0x7A0)), q))
                mstore(OPENING2_Y, mod(calldataload(add(data_ptr, 0x7C0)), q))
                {
                    let x := mload(OPENING2_X)
                    let y := mload(OPENING2_Y)
                    let xx := mulmod(x, x, q)
                    // validate on curve
                    success := and(
                        success,
                        eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                    )
                }

                if iszero(success) {
                    mstore(0x0, POINT_NOT_ON_CURVE_SELECTOR)
                    revert(0x00, 0x04)
                }

                // We also load the accumulators from the public inputs and validate them along with
                // the public input hash
                let piPtr := add(calldataload(0x24), 0x24)
                mstore(ACC_INSTANCE1_X, mod(calldataload(add(piPtr, 0x100)), q))
                mstore(ACC_INSTANCE1_Y, mod(calldataload(add(piPtr, 0x120)), q))
                {
                    let x := mload(ACC_INSTANCE1_X)
                    let y := mload(ACC_INSTANCE1_Y)
                    let xx := mulmod(x, x, q)
                    // validate on curve
                    success := and(
                        success,
                        eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                    )
                    if iszero(success) {
                        mstore(0x0, 1)
                        revert(0x00, 0x04)
                    }
                }
                mstore(ACC_PROOF1_X, mod(calldataload(add(piPtr, 0x140)), q))
                mstore(ACC_PROOF1_Y, mod(calldataload(add(piPtr, 0x160)), q))
                {
                    let x := mload(ACC_PROOF1_X)
                    let y := mload(ACC_PROOF1_Y)
                    let xx := mulmod(x, x, q)
                    // validate on curve
                    success := and(
                        success,
                        eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                    )
                    if iszero(success) {
                        mstore(0x0, 2)
                        revert(0x00, 0x04)
                    }
                }
                mstore(ACC_INSTANCE2_X, mod(calldataload(add(piPtr, 0x180)), q))
                mstore(ACC_INSTANCE2_Y, mod(calldataload(add(piPtr, 0x1A0)), q))
                {
                    let x := mload(ACC_INSTANCE2_X)
                    let y := mload(ACC_INSTANCE2_Y)
                    let xx := mulmod(x, x, q)
                    // validate on curve
                    success := and(
                        success,
                        eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                    )
                    if iszero(success) {
                        mstore(0x0, 3)
                        revert(0x00, 0x04)
                    }
                }
                mstore(ACC_PROOF2_X, mod(calldataload(add(piPtr, 0x1C0)), q))
                mstore(ACC_PROOF2_Y, mod(calldataload(add(piPtr, 0x1E0)), q))
                {
                    let x := mload(ACC_PROOF2_X)
                    let y := mload(ACC_PROOF2_Y)
                    let xx := mulmod(x, x, q)
                    // validate on curve
                    success := and(
                        success,
                        eq(mulmod(y, y, q), addmod(mulmod(x, xx, q), 3, q))
                    )
                    if iszero(success) {
                        mstore(0x0, 4)
                        revert(0x00, 0x04)
                    }
                }
                if iszero(success) {
                    mstore(0x0, POINT_NOT_ON_CURVE_SELECTOR)
                    revert(0x00, 0x04)
                }

                // Now th pi Hash
                {
                    calldatacopy(0x00, piPtr, 0x200)
                    mstore(
                        add(PUBLIC_INPUTS_HASH_LOCATION, 0x40),
                        mod(keccak256(0x00, 0x200), p)
                    )
                }
            }
            // Now we begin transcript and challenge generation
            // The first 64 bytes of the initial transcript are zero
            {
                mstore(PUBLIC_INPUTS_HASH_LOCATION, 0)

                // copy the public inputs over

                mstore(add(PUBLIC_INPUTS_HASH_LOCATION, 0x20), vkHash)

                let w_start := add(calldataload(0x04), 0x24)
                calldatacopy(
                    add(PUBLIC_INPUTS_HASH_LOCATION, 0x60),
                    w_start,
                    0x180
                )
                // Now we hash the initial information
                let challenge := keccak256(PUBLIC_INPUTS_HASH_LOCATION, 0x1E0)

                {
                    let tau := mod(challenge, p)
                    mstore(TAU, tau)
                }

                // Append lookup commitments and generate beta
                {
                    mstore(0x00, challenge)
                    mstore(0x20, mload(H1_X))
                    mstore(0x40, mload(H1_Y))
                    mstore(0x60, mload(H2_X))
                    mstore(0x80, mload(H2_Y))
                    challenge := keccak256(0x00, 0xa0)

                    mstore(BETA, mod(challenge, p))
                }
                {
                    mstore(0x00, challenge)
                    challenge := keccak256(0x00, 0x20)

                    mstore(GAMMA, mod(challenge, p))
                }
                {
                    mstore(0x00, challenge)
                    mstore(0x20, mload(PERM_X))
                    mstore(0x40, mload(PERM_Y))
                    mstore(0x60, mload(LOOKUP_PERM_X))
                    mstore(0x80, mload(LOOKUP_PERM_Y))
                    challenge := keccak256(0x00, 0xa0)

                    mstore(ALPHA, mod(challenge, p))
                }
                {
                    mstore(0x00, challenge)
                    mstore(0x20, mload(QUOT1_X))
                    mstore(0x40, mload(QUOT1_Y))
                    mstore(0x60, mload(QUOT2_X))
                    mstore(0x80, mload(QUOT2_Y))
                    mstore(0xa0, mload(QUOT3_X))
                    mstore(0xc0, mload(QUOT3_Y))
                    mstore(0xe0, mload(QUOT4_X))
                    mstore(0x100, mload(QUOT4_Y))
                    mstore(0x120, mload(QUOT5_X))
                    mstore(0x140, mload(QUOT5_Y))
                    mstore(0x160, mload(QUOT6_X))
                    mstore(0x180, mload(QUOT6_Y))
                    challenge := keccak256(0x00, 0x1a0)

                    mstore(ZETA, mod(challenge, p))
                }
                {
                    mstore(0x00, challenge)
                    mstore(0x20, mload(W1_EVAL))
                    mstore(0x40, mload(W2_EVAL))
                    mstore(0x60, mload(W3_EVAL))
                    mstore(0x80, mload(W4_EVAL))
                    mstore(0xa0, mload(W5_EVAL))
                    mstore(0xc0, mload(W6_EVAL))
                    mstore(0xe0, mload(SIGMA1_EVAL))
                    mstore(0x100, mload(SIGMA2_EVAL))
                    mstore(0x120, mload(SIGMA3_EVAL))
                    mstore(0x140, mload(SIGMA4_EVAL))
                    mstore(0x160, mload(SIGMA5_EVAL))
                    mstore(0x180, mload(PERM_EVAL))
                    mstore(0x1a0, mload(KEY_EVAL))
                    mstore(0x1c0, mload(TDS_EVAL))
                    mstore(0x1e0, mload(RANGE_EVAL))
                    mstore(0x200, mload(QDOMSEP_EVAL))
                    mstore(0x220, mload(H1_EVAL))
                    mstore(0x240, mload(Q_LOOKUP_EVAL))
                    mstore(0x260, mload(LOOKUP_PERM_EVAL))
                    mstore(0x280, mload(RANGE_NEXT_EVAL))
                    mstore(0x2a0, mload(KEY_NEXT_EVAL))
                    mstore(0x2c0, mload(TDS_NEXT_EVAL))
                    mstore(0x2e0, mload(H1_NEXT_EVAL))
                    mstore(0x300, mload(H2_NEXT_EVAL))
                    mstore(0x320, mload(Q_LOOKUP_NEXT_EVAL))
                    mstore(0x340, mload(W3_NEXT_EVAL))
                    mstore(0x360, mload(W4_NEXT_EVAL))
                    challenge := keccak256(0x00, 0x380)

                    mstore(CHALLENGE_V, mod(challenge, p))
                }
                {
                    mstore(0x00, challenge)
                    mstore(0x20, mload(OPENING1_X))
                    mstore(0x40, mload(OPENING1_Y))
                    mstore(0x60, mload(OPENING2_X))
                    mstore(0x80, mload(OPENING2_Y))
                    challenge := keccak256(0x00, 0xa0)

                    mstore(CHALLENGE_U, mod(challenge, p))
                }
            }
            {
                /**
                 * Now we comput the Vanisihing evaluation, Lagrange poly evals and the PI poly eval
                 */

                let zeta := mload(ZETA)
                let omega := mload(OMEGA_LOCATION)
                // compute zeta^n, where n is a power of 2
                let vanishing_numerator := zeta
                let domain_size := mload(DOMAIN_SIZE)
                {
                    // pow_small
                    let exponent := domain_size
                    let count := 1
                    for {

                    } lt(count, exponent) {
                        count := add(count, count)
                    } {
                        vanishing_numerator := mulmod(
                            vanishing_numerator,
                            vanishing_numerator,
                            p
                        )
                    }
                }
                mstore(ZETA_POW_N, vanishing_numerator)
                vanishing_numerator := addmod(vanishing_numerator, sub(p, 1), p)
                let vanish_num_div_n := mulmod(
                    vanishing_numerator,
                    mload(N_INV_LOCATION),
                    p
                )
                mstore(VANISH_EVAL, vanishing_numerator)
                {
                    let lagrange_n_denom := addmod(
                        sub(p, mload(OMEGA_INVERSE_LOCATION)),
                        zeta,
                        p
                    )

                    let lagrange_1_denom := addmod(sub(p, 1), zeta, p)

                    let accumulator := mulmod(
                        lagrange_1_denom,
                        lagrange_n_denom,
                        p
                    )
                    {
                        mstore(0, 0x20)
                        mstore(0x20, 0x20)
                        mstore(0x40, 0x20)
                        mstore(0x60, accumulator)
                        mstore(0x80, sub(p, 2))
                        mstore(0xa0, p)
                        if iszero(
                            staticcall(gas(), 0x05, 0x00, 0xc0, 0x00, 0x20)
                        ) {
                            mstore(0x0, MOD_EXP_FAILURE_SELECTOR)
                            revert(0x00, 0x04)
                        }
                        accumulator := mload(0x00)
                    }

                    {
                        let tmp := mulmod(accumulator, lagrange_n_denom, p)
                        tmp := mulmod(tmp, vanish_num_div_n, p)
                        mstore(LAGRANGE_1_EVAL, tmp)
                        mstore(
                            PI_EVAL,
                            mulmod(
                                tmp,
                                mload(add(PUBLIC_INPUTS_HASH_LOCATION, 0x40)),
                                p
                            )
                        )
                    }

                    {
                        mstore(
                            LAGRANGE_N_EVAL,
                            mulmod(
                                mulmod(
                                    mulmod(accumulator, lagrange_1_denom, p),
                                    mload(OMEGA_INVERSE_LOCATION),
                                    p
                                ),
                                vanish_num_div_n,
                                p
                            )
                        )
                    }
                }
            }
            // Now we compute the constant term
            {
                let w1_gamma := addmod(mload(W1_EVAL), mload(GAMMA), p)
                let w2_gamma := addmod(mload(W2_EVAL), mload(GAMMA), p)
                let w3_gamma := addmod(mload(W3_EVAL), mload(GAMMA), p)
                let w4_gamma := addmod(mload(W4_EVAL), mload(GAMMA), p)
                let w5_gamma := addmod(mload(W5_EVAL), mload(GAMMA), p)
                let w6_gamma := addmod(mload(W6_EVAL), mload(GAMMA), p)

                let acc := mulmod(
                    w6_gamma,
                    addmod(
                        w5_gamma,
                        mulmod(mload(BETA), mload(SIGMA5_EVAL), p),
                        p
                    ),
                    p
                )
                acc := mulmod(
                    acc,
                    addmod(
                        w4_gamma,
                        mulmod(mload(BETA), mload(SIGMA4_EVAL), p),
                        p
                    ),
                    p
                )
                acc := mulmod(
                    acc,
                    addmod(
                        w3_gamma,
                        mulmod(mload(BETA), mload(SIGMA3_EVAL), p),
                        p
                    ),
                    p
                )
                acc := mulmod(
                    acc,
                    addmod(
                        w2_gamma,
                        mulmod(mload(BETA), mload(SIGMA2_EVAL), p),
                        p
                    ),
                    p
                )
                acc := mulmod(
                    acc,
                    addmod(
                        w1_gamma,
                        mulmod(mload(BETA), mload(SIGMA1_EVAL), p),
                        p
                    ),
                    p
                )

                acc := mulmod(acc, mload(PERM_EVAL), p)

                // Add alpha * lagrange_1_eval
                let alpha_lagrange_1 := mulmod(
                    mload(ALPHA),
                    mload(LAGRANGE_1_EVAL),
                    p
                )
                acc := addmod(acc, alpha_lagrange_1, p)
                // mul by alpha
                acc := mulmod(acc, mload(ALPHA), p)
                // Subtract PI Eval
                acc := addmod(sub(p, acc), mload(PI_EVAL), p)

                // Plookup contribution
                let alpha_sq := mulmod(mload(ALPHA), mload(ALPHA), p)
                let alpha_cube := mulmod(alpha_sq, mload(ALPHA), p)

                let tmp := addmod(mload(H2_NEXT_EVAL), alpha_sq, p)
                tmp := mulmod(
                    mload(LAGRANGE_N_EVAL),
                    addmod(mload(H1_EVAL), sub(p, tmp), p),
                    p
                )

                let gamma_beta_one := mulmod(
                    mload(GAMMA),
                    addmod(mload(BETA), 1, p),
                    p
                )

                let tmp2 := addmod(
                    gamma_beta_one,
                    mulmod(mload(BETA), mload(H2_NEXT_EVAL), p),
                    p
                )
                let tmp3 := addmod(
                    gamma_beta_one,
                    addmod(
                        mulmod(mload(BETA), mload(H1_NEXT_EVAL), p),
                        mload(H1_EVAL),
                        p
                    ),
                    p
                )
                tmp2 := mulmod(tmp2, tmp3, p)
                tmp2 := mulmod(tmp2, mload(LOOKUP_PERM_EVAL), p)
                tmp2 := mulmod(
                    addmod(
                        mload(ZETA),
                        sub(p, mload(OMEGA_INVERSE_LOCATION)),
                        p
                    ),
                    tmp2,
                    p
                )
                tmp2 := mulmod(alpha_cube, tmp2, p)
                tmp2 := addmod(alpha_lagrange_1, tmp2, p)

                tmp := addmod(tmp, sub(p, tmp2), p)
                acc := addmod(acc, mulmod(tmp, alpha_cube, p), p)
                mstore(LIN_POLY_CONST, acc)

                // Now we aggregate the polynomial commitments

                // First we compute the coefficient for the prod perm poly
                let beta_zeta := mulmod(mload(BETA), mload(ZETA), p)
                let perm_coeff := addmod(beta_zeta, w1_gamma, p)
                let u_base := mload(CHALLENGE_U)
                let v_base := mload(CHALLENGE_V)
                let v := v_base
                perm_coeff := mulmod(
                    perm_coeff,
                    addmod(w2_gamma, mulmod(beta_zeta, mload(K2), p), p),
                    p
                )
                perm_coeff := mulmod(
                    perm_coeff,
                    addmod(w3_gamma, mulmod(beta_zeta, mload(K3), p), p),
                    p
                )
                perm_coeff := mulmod(
                    perm_coeff,
                    addmod(w4_gamma, mulmod(beta_zeta, mload(K4), p), p),
                    p
                )
                perm_coeff := mulmod(
                    perm_coeff,
                    addmod(w5_gamma, mulmod(beta_zeta, mload(K5), p), p),
                    p
                )
                perm_coeff := mulmod(
                    perm_coeff,
                    addmod(w6_gamma, mulmod(beta_zeta, mload(K6), p), p),
                    p
                )
                perm_coeff := addmod(alpha_lagrange_1, perm_coeff, p)
                perm_coeff := mulmod(perm_coeff, mload(ALPHA), p)
                perm_coeff := addmod(u_base, perm_coeff, p)

                // Progress the u base
                u_base := mulmod(u_base, v_base, p)

                let final_sigma_coeff := mulmod(mload(ALPHA), mload(BETA), p)
                final_sigma_coeff := mulmod(
                    final_sigma_coeff,
                    mload(PERM_EVAL),
                    p
                )
                final_sigma_coeff := mulmod(
                    final_sigma_coeff,
                    addmod(
                        mulmod(mload(BETA), mload(SIGMA1_EVAL), p),
                        w1_gamma,
                        p
                    ),
                    p
                )
                final_sigma_coeff := mulmod(
                    final_sigma_coeff,
                    addmod(
                        mulmod(mload(BETA), mload(SIGMA2_EVAL), p),
                        w2_gamma,
                        p
                    ),
                    p
                )
                final_sigma_coeff := mulmod(
                    final_sigma_coeff,
                    addmod(
                        mulmod(mload(BETA), mload(SIGMA3_EVAL), p),
                        w3_gamma,
                        p
                    ),
                    p
                )
                final_sigma_coeff := mulmod(
                    final_sigma_coeff,
                    addmod(
                        mulmod(mload(BETA), mload(SIGMA4_EVAL), p),
                        w4_gamma,
                        p
                    ),
                    p
                )
                final_sigma_coeff := mulmod(
                    final_sigma_coeff,
                    addmod(
                        mulmod(mload(BETA), mload(SIGMA5_EVAL), p),
                        w5_gamma,
                        p
                    ),
                    p
                )
                final_sigma_coeff := sub(p, final_sigma_coeff)

                // Begin point accumulation
                let success := 1
                mstore(ACC_X, mload(PERM_X))
                mstore(ACC_Y, mload(PERM_Y))
                mstore(ACC_SPACE_X, perm_coeff)
                // Perform the first scalar mul and store in 0x00
                success := staticcall(gas(), 0x07, ACC_X, 0x60, ACC_X, 0x40)

                mstore(ACC_SPACE_X, mload(SIGMA6_X))
                mstore(ACC_SPACE_Y, mload(SIGMA6_Y))
                mstore(ACC_NEXT_X, final_sigma_coeff)
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )

                // Add the two points
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                // Now we add the selector commitments
                // QLC_1
                mstore(ACC_SPACE_X, mload(QLC1_X))
                mstore(ACC_SPACE_Y, mload(QLC1_Y))
                mstore(ACC_NEXT_X, mload(W1_EVAL))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )
                // QLC_2
                mstore(ACC_SPACE_X, mload(QLC2_X))
                mstore(ACC_SPACE_Y, mload(QLC2_Y))
                mstore(ACC_NEXT_X, mload(W2_EVAL))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )
                // QLC_3
                mstore(ACC_SPACE_X, mload(QLC3_X))
                mstore(ACC_SPACE_Y, mload(QLC3_Y))
                mstore(ACC_NEXT_X, mload(W3_EVAL))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )
                // QLC_4
                mstore(ACC_SPACE_X, mload(QLC4_X))
                mstore(ACC_SPACE_Y, mload(QLC4_Y))
                mstore(ACC_NEXT_X, mload(W4_EVAL))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                let q_mul_1 := mulmod(mload(W1_EVAL), mload(W2_EVAL), p)
                let q_mul_2 := mulmod(mload(W3_EVAL), mload(W4_EVAL), p)
                // QMUL_1
                mstore(ACC_SPACE_X, mload(QMUL1_X))
                mstore(ACC_SPACE_Y, mload(QMUL1_Y))
                mstore(ACC_NEXT_X, q_mul_1)
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )
                // QMUL_2
                mstore(ACC_SPACE_X, mload(QMUL2_X))
                mstore(ACC_SPACE_Y, mload(QMUL2_Y))
                mstore(ACC_NEXT_X, q_mul_2)
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                let w_pow := mulmod(mload(W1_EVAL), mload(W1_EVAL), p)
                w_pow := mulmod(w_pow, w_pow, p)
                w_pow := mulmod(w_pow, mload(W1_EVAL), p)
                // QHASH_1
                mstore(ACC_SPACE_X, mload(QHASH1_X))
                mstore(ACC_SPACE_Y, mload(QHASH1_Y))
                mstore(ACC_NEXT_X, w_pow)
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )
                w_pow := mulmod(mload(W2_EVAL), mload(W2_EVAL), p)
                w_pow := mulmod(w_pow, w_pow, p)
                w_pow := mulmod(w_pow, mload(W2_EVAL), p)
                // QHASH_2
                mstore(ACC_SPACE_X, mload(QHASH2_X))
                mstore(ACC_SPACE_Y, mload(QHASH2_Y))
                mstore(ACC_NEXT_X, w_pow)
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                w_pow := mulmod(mload(W3_EVAL), mload(W3_EVAL), p)
                w_pow := mulmod(w_pow, w_pow, p)
                w_pow := mulmod(w_pow, mload(W3_EVAL), p)
                // QHASH_3
                mstore(ACC_SPACE_X, mload(QHASH3_X))
                mstore(ACC_SPACE_Y, mload(QHASH3_Y))
                mstore(ACC_NEXT_X, w_pow)
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )
                w_pow := mulmod(mload(W4_EVAL), mload(W4_EVAL), p)
                w_pow := mulmod(w_pow, w_pow, p)
                w_pow := mulmod(w_pow, mload(W4_EVAL), p)
                // QHASH_4
                mstore(ACC_SPACE_X, mload(QHASH4_X))
                mstore(ACC_SPACE_Y, mload(QHASH4_Y))
                mstore(ACC_NEXT_X, w_pow)
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )
                // QOUT
                mstore(ACC_SPACE_X, mload(QOUT_X))
                mstore(ACC_SPACE_Y, mload(QOUT_Y))
                mstore(ACC_NEXT_X, sub(p, mload(W5_EVAL)))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )
                // QCONST
                mstore(ACC_SPACE_X, mload(QCONST_X))
                mstore(ACC_SPACE_Y, mload(QCONST_Y))
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )
                // QECC
                let ecc_tmp := mulmod(q_mul_1, q_mul_2, p)
                mstore(ACC_SPACE_X, mload(QECC_X))
                mstore(ACC_SPACE_Y, mload(QECC_Y))
                mstore(ACC_NEXT_X, mulmod(ecc_tmp, mload(W5_EVAL), p))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )
                // QSWX1
                w_pow := mulmod(mload(W1_EVAL), mload(W3_EVAL), p)
                let w_2_4 := mulmod(mload(W2_EVAL), mload(W4_EVAL), p)
                let w_1_4 := mulmod(mload(W1_EVAL), mload(W4_EVAL), p)
                let w_2_3 := mulmod(mload(W2_EVAL), mload(W3_EVAL), p)
                mstore(ACC_SPACE_X, mload(QSWX1_X))
                mstore(ACC_SPACE_Y, mload(QSWX1_Y))
                mstore(ACC_NEXT_X, mulmod(q_mul_2, addmod(w_1_4, w_2_3, p), p))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )
                // QSWX2

                let qswx2_tmp := addmod(w_1_4, w_2_3, p)
                qswx2_tmp := mulmod(qswx2_tmp, 2, p)
                qswx2_tmp := addmod(qswx2_tmp, w_pow, p)
                qswx2_tmp := addmod(qswx2_tmp, w_2_4, p)
                mstore(ACC_SPACE_X, mload(QSWX2_X))
                mstore(ACC_SPACE_Y, mload(QSWX2_Y))
                mstore(ACC_NEXT_X, qswx2_tmp)
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )
                // QSWY1
                mstore(ACC_SPACE_X, mload(QSWY1_X))
                mstore(ACC_SPACE_Y, mload(QSWY1_Y))
                mstore(ACC_NEXT_X, mulmod(q_mul_2, q_mul_2, p))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )
                // QSWY2
                w_pow := addmod(mload(W1_EVAL), mload(W2_EVAL), p)
                mstore(ACC_SPACE_X, mload(QSWY2_X))
                mstore(ACC_SPACE_Y, mload(QSWY2_Y))
                mstore(ACC_NEXT_X, mulmod(w_pow, q_mul_1, p))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                if iszero(success) {
                    mstore(0x00, POINT_NOT_ON_CURVE_SELECTOR)
                    revert(0x00, 0x04)
                }
            }
            // Now we compute merged table and lookup wire coefficients
            {
                // Merged lookup wire
                let merged_lw := mulmod(mload(W3_EVAL), mload(TAU), p)
                merged_lw := addmod(merged_lw, mload(W2_EVAL), p)
                merged_lw := mulmod(merged_lw, mload(TAU), p)
                merged_lw := addmod(merged_lw, mload(W1_EVAL), p)
                merged_lw := mulmod(merged_lw, mload(TAU), p)
                merged_lw := addmod(merged_lw, mload(QDOMSEP_EVAL), p)
                merged_lw := mulmod(merged_lw, mload(TAU), p)
                merged_lw := mulmod(merged_lw, mload(Q_LOOKUP_EVAL), p)
                mstore(MERGED_LW_EVAL, addmod(merged_lw, mload(W6_EVAL), p))
            }

            {
                // Merged table eval
                let merged_table := mulmod(mload(W5_EVAL), mload(TAU), p)
                merged_table := addmod(merged_table, mload(W4_EVAL), p)
                merged_table := mulmod(merged_table, mload(TAU), p)
                merged_table := addmod(merged_table, mload(KEY_EVAL), p)
                merged_table := mulmod(merged_table, mload(TAU), p)
                merged_table := addmod(merged_table, mload(TDS_EVAL), p)
                merged_table := mulmod(merged_table, mload(TAU), p)
                merged_table := mulmod(merged_table, mload(Q_LOOKUP_EVAL), p)
                mstore(
                    MERGED_TABLE_EVAL,
                    addmod(merged_table, mload(RANGE_EVAL), p)
                )
            }

            {
                // Merged table next eval
                let merged_table := mulmod(mload(W4_NEXT_EVAL), mload(TAU), p)
                merged_table := addmod(merged_table, mload(W3_NEXT_EVAL), p)
                merged_table := mulmod(merged_table, mload(TAU), p)
                merged_table := addmod(merged_table, mload(KEY_NEXT_EVAL), p)
                merged_table := mulmod(merged_table, mload(TAU), p)
                merged_table := addmod(merged_table, mload(TDS_NEXT_EVAL), p)
                merged_table := mulmod(merged_table, mload(TAU), p)
                merged_table := mulmod(
                    merged_table,
                    mload(Q_LOOKUP_NEXT_EVAL),
                    p
                )
                mstore(
                    MERGED_TABLE_NEXT_EVAL,
                    addmod(merged_table, mload(RANGE_NEXT_EVAL), p)
                )
            }

            {
                let u_base := mulmod(mload(CHALLENGE_U), mload(CHALLENGE_V), p)

                let alpha_cube := mulmod(
                    mulmod(mload(ALPHA), mload(ALPHA), p),
                    mload(ALPHA),
                    p
                )

                // Lookup perm poly coeff
                let lookup_perm_coeff := mulmod(
                    mload(BETA),
                    mload(MERGED_TABLE_NEXT_EVAL),
                    p
                )
                lookup_perm_coeff := addmod(
                    lookup_perm_coeff,
                    mload(MERGED_TABLE_EVAL),
                    p
                )
                lookup_perm_coeff := addmod(
                    lookup_perm_coeff,
                    mulmod(mload(GAMMA), addmod(mload(BETA), 1, p), p),
                    p
                )
                lookup_perm_coeff := mulmod(
                    addmod(mload(GAMMA), mload(MERGED_LW_EVAL), p),
                    lookup_perm_coeff,
                    p
                )
                lookup_perm_coeff := mulmod(
                    addmod(mload(BETA), 1, p),
                    lookup_perm_coeff,
                    p
                )
                lookup_perm_coeff := mulmod(
                    addmod(
                        mload(ZETA),
                        sub(p, mload(OMEGA_INVERSE_LOCATION)),
                        p
                    ),
                    lookup_perm_coeff,
                    p
                )
                lookup_perm_coeff := mulmod(mload(ALPHA), lookup_perm_coeff, p)
                lookup_perm_coeff := addmod(
                    mload(LAGRANGE_N_EVAL),
                    lookup_perm_coeff,
                    p
                )
                lookup_perm_coeff := mulmod(
                    mulmod(mload(ALPHA), mload(ALPHA), p),
                    lookup_perm_coeff,
                    p
                )
                lookup_perm_coeff := addmod(
                    mulmod(mload(ALPHA), mload(LAGRANGE_1_EVAL), p),
                    lookup_perm_coeff,
                    p
                )
                lookup_perm_coeff := mulmod(alpha_cube, lookup_perm_coeff, p)

                lookup_perm_coeff := addmod(u_base, lookup_perm_coeff, p)
                let success := 1

                mstore(ACC_SPACE_X, mload(LOOKUP_PERM_X))
                mstore(ACC_SPACE_Y, mload(LOOKUP_PERM_Y))
                mstore(ACC_NEXT_X, lookup_perm_coeff)
                success := staticcall(
                    gas(),
                    0x07,
                    ACC_SPACE_X,
                    0x60,
                    ACC_SPACE_X,
                    0x40
                )

                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                //h2 poly coeff
                let h2_coeff := mulmod(mload(BETA), mload(H1_NEXT_EVAL), p)
                h2_coeff := addmod(h2_coeff, mload(H1_EVAL), p)
                h2_coeff := addmod(
                    mulmod(mload(GAMMA), addmod(mload(BETA), 1, p), p),
                    h2_coeff,
                    p
                )
                h2_coeff := mulmod(h2_coeff, mload(LOOKUP_PERM_EVAL), p)

                h2_coeff := mulmod(
                    addmod(
                        mload(OMEGA_INVERSE_LOCATION),
                        sub(p, mload(ZETA)),
                        p
                    ),
                    h2_coeff,
                    p
                )

                h2_coeff := mulmod(alpha_cube, h2_coeff, p)
                h2_coeff := mulmod(alpha_cube, h2_coeff, p)

                let uv_tmp := mulmod(mload(CHALLENGE_V), mload(CHALLENGE_V), p)
                uv_tmp := mulmod(uv_tmp, uv_tmp, p)
                uv_tmp := mulmod(uv_tmp, u_base, p)

                h2_coeff := addmod(h2_coeff, uv_tmp, p)

                mstore(ACC_SPACE_X, mload(H2_X))
                mstore(ACC_SPACE_Y, mload(H2_Y))
                mstore(ACC_NEXT_X, h2_coeff)
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                if iszero(success) {
                    mstore(0x00, POINT_NOT_ON_CURVE_SELECTOR)
                    revert(0x00, 0x04)
                }
            }

            {
                // Quot contributions
                let zeta_n_plus_two := mulmod(mload(ZETA_POW_N), mload(ZETA), p)
                zeta_n_plus_two := mulmod(zeta_n_plus_two, mload(ZETA), p)

                let quot_coeff := sub(p, mload(VANISH_EVAL))
                let success := 1

                mstore(ACC_SPACE_X, mload(QUOT1_X))
                mstore(ACC_SPACE_Y, mload(QUOT1_Y))
                mstore(ACC_NEXT_X, quot_coeff)
                success := staticcall(
                    gas(),
                    0x07,
                    ACC_SPACE_X,
                    0x60,
                    ACC_SPACE_X,
                    0x40
                )

                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                quot_coeff := mulmod(quot_coeff, zeta_n_plus_two, p)

                mstore(ACC_SPACE_X, mload(QUOT2_X))
                mstore(ACC_SPACE_Y, mload(QUOT2_Y))
                mstore(ACC_NEXT_X, quot_coeff)
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                quot_coeff := mulmod(quot_coeff, zeta_n_plus_two, p)

                mstore(ACC_SPACE_X, mload(QUOT3_X))
                mstore(ACC_SPACE_Y, mload(QUOT3_Y))
                mstore(ACC_NEXT_X, quot_coeff)
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                quot_coeff := mulmod(quot_coeff, zeta_n_plus_two, p)

                mstore(ACC_SPACE_X, mload(QUOT4_X))
                mstore(ACC_SPACE_Y, mload(QUOT4_Y))
                mstore(ACC_NEXT_X, quot_coeff)
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                quot_coeff := mulmod(quot_coeff, zeta_n_plus_two, p)

                mstore(ACC_SPACE_X, mload(QUOT5_X))
                mstore(ACC_SPACE_Y, mload(QUOT5_Y))
                mstore(ACC_NEXT_X, quot_coeff)
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                quot_coeff := mulmod(quot_coeff, zeta_n_plus_two, p)

                mstore(ACC_SPACE_X, mload(QUOT6_X))
                mstore(ACC_SPACE_Y, mload(QUOT6_Y))
                mstore(ACC_NEXT_X, quot_coeff)
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                if iszero(success) {
                    mstore(0x00, POINT_NOT_ON_CURVE_SELECTOR)
                    revert(0x00, 0x04)
                }
            }

            {
                // Now we do the other commitments and the combined scalars

                let v_base := mload(CHALLENGE_V)

                let acc_scalar := sub(p, mload(LIN_POLY_CONST))

                let success := 1
                // W1
                mstore(ACC_SPACE_X, mload(W1_X))
                mstore(ACC_SPACE_Y, mload(W1_Y))
                mstore(ACC_NEXT_X, v_base)
                success := staticcall(
                    gas(),
                    0x07,
                    ACC_SPACE_X,
                    0x60,
                    ACC_SPACE_X,
                    0x40
                )

                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(v_base, mload(W1_EVAL), p),
                    p
                )
                v_base := mulmod(v_base, mload(CHALLENGE_V), p)
                let uv_sq := mulmod(mload(CHALLENGE_U), v_base, p)

                // W2

                mstore(ACC_SPACE_X, mload(W2_X))
                mstore(ACC_SPACE_Y, mload(W2_Y))
                mstore(ACC_NEXT_X, v_base)
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(v_base, mload(W2_EVAL), p),
                    p
                )
                v_base := mulmod(v_base, mload(CHALLENGE_V), p)
                let uv_cube := mulmod(mload(CHALLENGE_U), v_base, p)

                // W3

                mstore(ACC_SPACE_X, mload(W3_X))
                mstore(ACC_SPACE_Y, mload(W3_Y))
                mstore(ACC_NEXT_X, v_base)
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(v_base, mload(W3_EVAL), p),
                    p
                )
                v_base := mulmod(v_base, mload(CHALLENGE_V), p)
                let uv_quad := mulmod(mload(CHALLENGE_U), v_base, p)
                let uv_seven := mulmod(uv_cube, v_base, p)

                // W4

                mstore(ACC_SPACE_X, mload(W4_X))
                mstore(ACC_SPACE_Y, mload(W4_Y))
                mstore(ACC_NEXT_X, addmod(v_base, uv_seven, p))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(v_base, mload(W4_EVAL), p),
                    p
                )
                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(uv_seven, mload(W3_NEXT_EVAL), p),
                    p
                )
                v_base := mulmod(v_base, mload(CHALLENGE_V), p)
                let uv_eight := mulmod(uv_cube, v_base, p)

                // W5

                mstore(ACC_SPACE_X, mload(W5_X))
                mstore(ACC_SPACE_Y, mload(W5_Y))
                mstore(ACC_NEXT_X, addmod(v_base, uv_eight, p))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(v_base, mload(W5_EVAL), p),
                    p
                )
                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(uv_eight, mload(W4_NEXT_EVAL), p),
                    p
                )
                v_base := mulmod(v_base, mload(CHALLENGE_V), p)
                let uv_nine := mulmod(uv_cube, v_base, p)
                let uv_six := mulmod(mload(CHALLENGE_U), v_base, p)

                // W6

                mstore(ACC_SPACE_X, mload(W6_X))
                mstore(ACC_SPACE_Y, mload(W6_Y))
                mstore(ACC_NEXT_X, v_base)
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(v_base, mload(W6_EVAL), p),
                    p
                )
                v_base := mulmod(v_base, mload(CHALLENGE_V), p)

                // SIGMA1
                mstore(ACC_SPACE_X, mload(SIGMA1_X))
                mstore(ACC_SPACE_Y, mload(SIGMA1_Y))
                mstore(ACC_NEXT_X, v_base)
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(v_base, mload(SIGMA1_EVAL), p),
                    p
                )
                v_base := mulmod(v_base, mload(CHALLENGE_V), p)

                // SIGMA2
                mstore(ACC_SPACE_X, mload(SIGMA2_X))
                mstore(ACC_SPACE_Y, mload(SIGMA2_Y))
                mstore(ACC_NEXT_X, v_base)
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(v_base, mload(SIGMA2_EVAL), p),
                    p
                )
                v_base := mulmod(v_base, mload(CHALLENGE_V), p)

                // SIGMA3
                mstore(ACC_SPACE_X, mload(SIGMA3_X))
                mstore(ACC_SPACE_Y, mload(SIGMA3_Y))
                mstore(ACC_NEXT_X, v_base)
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(v_base, mload(SIGMA3_EVAL), p),
                    p
                )
                v_base := mulmod(v_base, mload(CHALLENGE_V), p)

                // SIGMA4
                mstore(ACC_SPACE_X, mload(SIGMA4_X))
                mstore(ACC_SPACE_Y, mload(SIGMA4_Y))
                mstore(ACC_NEXT_X, v_base)
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(v_base, mload(SIGMA4_EVAL), p),
                    p
                )
                v_base := mulmod(v_base, mload(CHALLENGE_V), p)

                // SIGMA5
                mstore(ACC_SPACE_X, mload(SIGMA5_X))
                mstore(ACC_SPACE_Y, mload(SIGMA5_Y))
                mstore(ACC_NEXT_X, v_base)
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(v_base, mload(SIGMA5_EVAL), p),
                    p
                )
                v_base := mulmod(v_base, mload(CHALLENGE_V), p)

                // Quickly add perm eval times u and lookup perm times uv
                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(mload(CHALLENGE_U), mload(PERM_EVAL), p),
                    p
                )
                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(
                        mulmod(mload(CHALLENGE_U), mload(CHALLENGE_V), p),
                        mload(LOOKUP_PERM_EVAL),
                        p
                    ),
                    p
                )

                // RANGE
                mstore(ACC_SPACE_X, mload(PLRANGE_X))
                mstore(ACC_SPACE_Y, mload(PLRANGE_Y))
                mstore(ACC_NEXT_X, addmod(v_base, uv_sq, p))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(v_base, mload(RANGE_EVAL), p),
                    p
                )
                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(uv_sq, mload(RANGE_NEXT_EVAL), p),
                    p
                )
                v_base := mulmod(v_base, mload(CHALLENGE_V), p)

                // KEY
                mstore(ACC_SPACE_X, mload(PLKEY_X))
                mstore(ACC_SPACE_Y, mload(PLKEY_Y))
                mstore(ACC_NEXT_X, addmod(v_base, uv_cube, p))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(v_base, mload(KEY_EVAL), p),
                    p
                )
                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(uv_cube, mload(KEY_NEXT_EVAL), p),
                    p
                )
                v_base := mulmod(v_base, mload(CHALLENGE_V), p)

                // H1

                mstore(ACC_SPACE_X, mload(H1_X))
                mstore(ACC_SPACE_Y, mload(H1_Y))
                mstore(ACC_NEXT_X, addmod(v_base, uv_quad, p))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(v_base, mload(H1_EVAL), p),
                    p
                )
                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(uv_quad, mload(H1_NEXT_EVAL), p),
                    p
                )
                v_base := mulmod(v_base, mload(CHALLENGE_V), p)

                // Quickly add h2 eval times uv^5
                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(
                        mulmod(uv_quad, mload(CHALLENGE_V), p),
                        mload(H2_NEXT_EVAL),
                        p
                    ),
                    p
                )

                // Q_LOOKUP
                mstore(ACC_SPACE_X, mload(QLOOKUP_X))
                mstore(ACC_SPACE_Y, mload(QLOOKUP_Y))
                mstore(ACC_NEXT_X, addmod(v_base, uv_six, p))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(v_base, mload(Q_LOOKUP_EVAL), p),
                    p
                )
                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(uv_six, mload(Q_LOOKUP_NEXT_EVAL), p),
                    p
                )

                v_base := mulmod(v_base, mload(CHALLENGE_V), p)

                // TDS
                mstore(ACC_SPACE_X, mload(PLTDS_X))
                mstore(ACC_SPACE_Y, mload(PLTDS_Y))
                mstore(ACC_NEXT_X, addmod(v_base, uv_nine, p))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(v_base, mload(TDS_EVAL), p),
                    p
                )
                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(uv_nine, mload(TDS_NEXT_EVAL), p),
                    p
                )

                v_base := mulmod(v_base, mload(CHALLENGE_V), p)

                // Q_DOMSEP
                mstore(ACC_SPACE_X, mload(QDOMSEP_X))
                mstore(ACC_SPACE_Y, mload(QDOMSEP_Y))
                mstore(ACC_NEXT_X, v_base)
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                acc_scalar := addmod(
                    acc_scalar,
                    mulmod(v_base, mload(QDOMSEP_EVAL), p),
                    p
                )

                // Value of the combined scalar
                mstore(ACC_SPACE_X, mload(OKG1_X))
                mstore(ACC_SPACE_Y, mload(OKG1_Y))
                mstore(ACC_NEXT_X, sub(p, acc_scalar))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                // OPENING1
                mstore(ACC_SPACE_X, mload(OPENING1_X))
                mstore(ACC_SPACE_Y, mload(OPENING1_Y))
                mstore(ACC_NEXT_X, mload(ZETA))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )

                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                // Opening proof 2
                mstore(ACC_SPACE_X, mload(OPENING2_X))
                mstore(ACC_SPACE_Y, mload(OPENING2_Y))
                mstore(
                    ACC_NEXT_X,
                    mulmod(
                        mload(OMEGA_LOCATION),
                        mulmod(mload(ZETA), mload(CHALLENGE_U), p),
                        p
                    )
                )
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                mstore(ACC_SPACE_X, mload(ACC_INSTANCE2_X))
                mstore(ACC_SPACE_Y, mload(ACC_INSTANCE2_Y))
                mstore(
                    ACC_NEXT_X,
                    mulmod(mload(CHALLENGE_U), mload(CHALLENGE_U), p)
                )
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                mstore(ACC_NEXT_X, mload(ACC_INSTANCE1_X))
                mstore(ACC_NEXT_Y, mload(ACC_INSTANCE1_Y))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x06,
                        ACC_SPACE_X,
                        0x80,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                mstore(ACC_NEXT_X, mload(CHALLENGE_U))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_X, 0x80, ACC_X, 0x40)
                )

                if iszero(success) {
                    mstore(0x00, POINT_NOT_ON_CURVE_SELECTOR)
                    revert(0x00, 0x04)
                }
            }

            {
                // Other part of pairing
                mstore(ACC_SPACE_X, mload(ACC_PROOF2_X))
                mstore(ACC_SPACE_Y, mload(ACC_PROOF2_Y))
                mstore(ACC_NEXT_X, mload(CHALLENGE_U))
                let success := staticcall(
                    gas(),
                    0x07,
                    ACC_SPACE_X,
                    0x60,
                    ACC_SPACE_X,
                    0x40
                )
                mstore(ACC_NEXT_X, mload(ACC_PROOF1_X))
                mstore(ACC_NEXT_Y, mload(ACC_PROOF1_Y))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x06,
                        ACC_SPACE_X,
                        0x80,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                mstore(ACC_NEXT_X, mload(CHALLENGE_U))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                mstore(ACC_NEXT_X, mload(OPENING2_X))
                mstore(ACC_NEXT_Y, mload(OPENING2_Y))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x06,
                        ACC_SPACE_X,
                        0x80,
                        ACC_SPACE_X,
                        0x40
                    )
                )
                mstore(ACC_NEXT_X, mload(CHALLENGE_U))
                success := and(
                    success,
                    staticcall(
                        gas(),
                        0x07,
                        ACC_SPACE_X,
                        0x60,
                        ACC_SPACE_X,
                        0x40
                    )
                )

                mstore(ACC_NEXT_X, mload(OPENING1_X))
                mstore(ACC_NEXT_Y, mload(OPENING1_Y))
                success := and(
                    success,
                    staticcall(gas(), 0x06, ACC_SPACE_X, 0x80, ACC_NEXT_X, 0x40)
                )
            }

            /**
             * PERFORM PAIRING
             */
            {
                let success := 1
                let ptr := 0x1900
                // rhs paired with [1]_2
                // lhs paired with [x]_2

                mstore(0x1900, mload(ACC_NEXT_X))
                mstore(0x1920, mload(ACC_NEXT_Y))
                mstore(0x1940, mload(OKG3_X2)) // this is [x]_2
                mstore(0x1960, mload(OKG3_X1))
                mstore(0x1980, mload(OKG3_Y2))
                mstore(0x19a0, mload(OKG3_Y1))
                mstore(0x19c0, mload(ACC_X))
                mstore(0x19e0, mload(ACC_Y))
                mstore(0x1a00, mload(OKG2_X2))
                mstore(0x1a20, mload(OKG2_X1))
                mstore(0x1a40, mload(OKG2_Y2))
                mstore(0x1a60, mload(OKG2_Y1))

                success := and(
                    success,
                    staticcall(gas(), 0x08, 0x1900, 0x180, 0x00, 0x20)
                )
                success := and(success, iszero(mload(0x00)))
                if iszero(success) {
                    mstore(0x0, PAIRING_FAILED_SELECTOR)
                    revert(0x00, 0x04)
                }
            }

            {
                mstore(0x00, 0x01)
                return(0x00, 0x20) // Proof succeeded!
            }
        }
    }
}
