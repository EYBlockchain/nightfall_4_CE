// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script} from "@forge-std/Script.sol";
import "@forge-std/StdToml.sol";
import "../contracts/ERC20Mock.sol";
import "../contracts/ERC721Mock.sol";
import "../contracts/ERC1155Mock.sol";
import "../contracts/ERC3525Mock.sol";
import "../contracts/X509/X509.sol";


contract MockDeployer is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("NF4_SIGNING_KEY");
        address owner = vm.envAddress("CLIENT_ADDRESS");
        address client2 = vm.envAddress("CLIENT2_ADDRESS");
        vm.startBroadcast(deployerPrivateKey);
        new ERC20Mock(999999, owner, client2);
        new ERC721Mock(426, owner);
        new ERC1155Mock(2, 100, 73, 0, owner);
        new ERC3525Mock(7, 100, 8, 120, 5, owner);
        vm.stopBroadcast();
    }
}
