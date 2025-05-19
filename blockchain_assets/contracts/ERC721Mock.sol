// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";

contract ERC721Mock is ERC721 {
    constructor(
        uint256 initial_id,
        uint256 initial_id2,
        address initialOwner
    ) ERC721("ERC721Mock", "E721") {
        for (uint256 i = initial_id; i < initial_id2; i++) {
            mint(initialOwner, i);
        }
    }

    function mint(address owner, uint256 tokenId) public {
        ERC721._mint(owner, tokenId);
    }
}
