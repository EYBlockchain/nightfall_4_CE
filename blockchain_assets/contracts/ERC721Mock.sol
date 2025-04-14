// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";

contract ERC721Mock is ERC721 {
    constructor(
        uint256 initial_id,
        address initialOwner
    ) ERC721("ERC721Mock", "E721") {
        mint(initialOwner, initial_id);
    }

    function mint(address owner, uint256 tokenId) public {
        ERC721._mint(owner, tokenId);
    }
}
