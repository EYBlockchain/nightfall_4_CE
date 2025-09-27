// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script} from "@forge-std/Script.sol";
import "@forge-std/StdToml.sol";
import "../contracts/ERC20Mock.sol";
import "../contracts/ERC721Mock.sol";
import "../contracts/ERC1155Mock.sol";
import "../contracts/ERC3525Mock.sol";
import "../contracts/X509/X509.sol";
import "../contracts/Nightfall.sol";
import "forge-std/console.sol";

contract MockDeployer is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("NF4_SIGNING_KEY");
        address owner = vm.envAddress("CLIENT_ADDRESS");
        address client2 = vm.envAddress("CLIENT2_ADDRESS");
        // This is the address of the Nightfall contract, which is not used in this script
        address nightfallAddress = vm.envAddress("NIGHTFALL_ADDRESS");
        vm.startBroadcast(deployerPrivateKey);
        ERC20Mock erc20 = new ERC20Mock(2000, nightfallAddress, owner, client2);
        ERC721Mock erc721 = new ERC721Mock(426, owner, nightfallAddress);
        ERC1155Mock erc1155 = new ERC1155Mock(
            nightfallAddress,
            2,
            100,
            73,
            0,
            owner
        );
        ERC3525Mock erc3525 = new ERC3525Mock(
            nightfallAddress,
            7,
            100,
            8,
            120,
            5,
            owner
        );
        // Keep this log
        console.log("Deployed ERC20Mock to:", address(erc20));
        console.log("Deployed ERC721Mock to:", address(erc721));
        console.log("Deployed ERC1155Mock to:", address(erc1155));
        console.log("Deployed ERC3525Mock to:", address(erc3525));
        vm.stopBroadcast();
    }
}
