// SPDX-License-Identifier: MIT

pragma solidity ^0.8.1;

import "@erc-3525/contracts/ERC3525.sol";

contract ERC3525Mock is ERC3525 {
    constructor(
        uint256 initial_id,
        uint256 value,
        uint256 initial_id_two,
        uint256 value_two,
        uint256 slot,
        address initialOwner
    ) ERC3525("ERC3525Mock", "ERC3525", 100) {
        mint(initialOwner, initial_id, slot, value);
        mint(initialOwner, initial_id_two, slot, value_two);
    }

    function mint(
        address mintTo_,
        uint256 tokenId_,
        uint256 slot_,
        uint256 value_
    ) public virtual {
        ERC3525._mint(mintTo_, tokenId_, slot_, value_);
    }

    function mintValue(uint256 tokenId_, uint256 value_) public virtual {
        ERC3525._mintValue(tokenId_, value_);
    }

    function burn(uint256 tokenId_) public virtual {
        require(
            _isApprovedOrOwner(_msgSender(), tokenId_),
            "ERC3525: caller is not token owner nor approved"
        );
        ERC3525._burn(tokenId_);
    }

    function burnValue(uint256 tokenId_, uint256 burnValue_) public virtual {
        require(
            _isApprovedOrOwner(_msgSender(), tokenId_),
            "ERC3525: caller is not token owner nor approved"
        );
        ERC3525._burnValue(tokenId_, burnValue_);
    }
}
