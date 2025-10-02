// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ERC1155} from "@openzeppelin/contracts/token/ERC1155/ERC1155.sol";

contract ERC1155Mock is ERC1155 {
    constructor(
        address spender,
        uint256 initial_id,
        uint256 value,
        uint256 initial_id_two,
        uint256 value_two,
        address initialOwner
    ) ERC1155("ERC1155Mock") {
        mint(initialOwner, spender, initial_id, value);
        mint(initialOwner, spender, initial_id_two, value_two);
    }

    function mint(
        address owner,
        address spender,
        uint256 tokenId,
        uint256 value
    ) public {
        ERC1155._setApprovalForAll(owner, spender, true);
        ERC1155._mint(owner, tokenId, value, "");
    }
}
