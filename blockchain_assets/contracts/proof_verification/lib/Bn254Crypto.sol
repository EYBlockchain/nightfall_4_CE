// SPDX-License-Identifier: GPL-2.0-only
pragma solidity ^0.8.20;

import "./Types.sol";

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
            switch success
            case 0 { revert(0, 0) }
        }
        require(success, "Bn254: scalar mul failed!");
    }

    function multiScalarMul(
        Types.G1Point[] memory bases,
        uint256[] memory scalars
    ) internal view returns (Types.G1Point memory r) {
        require(scalars.length == bases.length, "MSM error: length mismatch");
        r = scalarMul(bases[0], scalars[0]);
        for (uint256 i = 1; i < scalars.length; i++) {
            r = add(r, scalarMul(bases[i], scalars[i]));
        }
    }

    function negate_fr(uint256 fr) internal pure returns (uint256 res) {
        uint256 m = r_mod;
        uint256 a = fr % m;        // a ∈ [0, m-1]
        if (a == 0) return 0;      // canonical zero
        return m - a;              // ∈ [1, m-1]
    }

    function negate_G1Point(
        Types.G1Point memory p
    ) internal pure returns (Types.G1Point memory) {
        if (isInfinity(p)) return p;
        uint256 m = p_mod;
        uint256 y = p.y % m;
        uint256 ny = (y == 0) ? 0 : (m - y);
        return Types.G1Point(p.x % m, ny);
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
        input[0] = p1.x; input[1] = p1.y;
        input[2] = p2.x; input[3] = p2.y;
        bool success;
        assembly {
            success := staticcall(sub(gas(), 2000), 6, input, 0xc0, r, 0x60)
            switch success
            case 0 { revert(0, 0) }
        }
        require(success, "Bn254: group addition failed!");
    }

    function fromLeBytesModOrder(
        bytes memory leBytes
    ) internal pure returns (uint256 ret) {
        assembly {
            let len := mload(leBytes)
            let byteData := add(leBytes, 0x20)
            for { let i := 0 } lt(i, len) { i := add(i, 1) } {
                ret := mulmod(ret, 256, r_mod)
                let byteVal := byte(0, mload(sub(sub(add(byteData, len), i), 1)))
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
        uint256 output; bool success; uint256 p = r_mod;
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

    function pairingProd2(
        Types.G1Point memory a1,
        Types.G2Point memory a2,
        Types.G1Point memory b1,
        Types.G2Point memory b2
    ) internal view returns (bool) {
        validate_G1Point(a1);
        validate_G1Point(b1);
        bool success; uint256 out;
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

    function validate_G1Point(Types.G1Point memory point) internal pure {
        bool ok; uint256 p = p_mod;
        assembly {
            let x := mload(point)
            let y := mload(add(point, 0x20))
            ok := and(
                and(and(lt(x, p), lt(y, p)), not(and(iszero(x), iszero(y)))),
                eq(mulmod(y, y, p), addmod(mulmod(x, mulmod(x, x, p), p), 3, p))
            )
        }
        require(ok, "Bn254: G1 point not on curve, or malformed");
    }

    function validate_scalar_field(uint256 fr) internal pure {
        bool isValid;
        assembly { isValid := lt(fr, r_mod) }
        require(isValid, "Bn254: invalid scalar field");
    }
}
