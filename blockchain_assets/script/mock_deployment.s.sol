// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script} from "@forge-std/Script.sol";
import "@forge-std/StdToml.sol";
import "forge-std/console.sol";
import "../contracts/ERC20Mock.sol";
import "../contracts/ERC721Mock.sol";
import "../contracts/ERC1155Mock.sol";
import "../contracts/ERC3525Mock.sol";
import "../contracts/X509/X509.sol";


contract MockDeployer is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("NF4_SIGNING_KEY");
        // address owner = vm.addr(deployerPrivateKey);
        address owner = vm.envAddress("CLIENT_ADDRESS");
        address client2 = vm.envAddress("CLIENT2_ADDRESS");
        vm.startBroadcast(deployerPrivateKey);
        ERC20Mock erc20 = new ERC20Mock(2000, owner, client2);
        console.log("ERC20Mock deployed at:", address(erc20));
        ERC721Mock erc721 = new ERC721Mock(426, owner);
        console.log("ERC721Mock deployed at:", address(erc721));
        ERC1155Mock erc1155 = new ERC1155Mock(2, 100, 73, 0, owner);
        console.log("ERC1155Mock deployed at:", address(erc1155));
        ERC3525Mock erc3525 = new ERC3525Mock(7, 100, 8, 120, 5, owner);
        console.log("ERC3525Mock deployed at:", address(erc3525));
        vm.stopBroadcast();
    }
}
