// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract ERC20Mock is ERC20 {
    constructor(
        uint initialSupply,
        address initialOwner,
        address other_client
    ) ERC20("ERC20Mock", "E20") {
        mint(initialOwner, initialSupply/2);
        mint(other_client, initialSupply/2);
    }

    function decimals() public view virtual override returns (uint8) {
        return 9;
    }

    function mint(address mintTo_, uint256 value_) public virtual {
        ERC20._mint(mintTo_, value_);
    }
}
