// SPDX-License-Identifier: CC0
pragma solidity >=0.8.20;

import "forge-std/Test.sol";
import "forge-std/console2.sol";

contract BlockHashTest is Test {
    function sha256_and_shift(
        bytes memory inputs
    ) private pure returns (uint256) {
        bytes32 hash = sha256(inputs);
        return uint256(hash) >> 3;
    }

    function test_block_hash() public pure {
        uint256[] memory transactions = new uint256[](8);

        for (uint i; i < 8; i++) {
            transactions[i] = sha256_and_shift(
                abi.encode(
                    uint256(0),
                    uint256(1),
                    uint256(2),
                    uint256(3),
                    uint256(4),
                    uint256(5),
                    uint256(6),
                    uint256(7),
                    uint256(8),
                    uint256(9),
                    uint256(10),
                    uint256(11)
                )
            );
        }

        uint256 length = 4;

        while (length >= 1) {
            for (uint256 i = 0; i < length; i++) {
                transactions[i] = uint256(
                    sha256_and_shift(
                        abi.encodePacked(
                            transactions[2 * i],
                            transactions[(2 * i) + 1]
                        )
                    )
                ); // hash a row
            }
            length = length / 2;
        }
    }
}