// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import "forge-std/console.sol";
import "@forge-std/StdToml.sol";

import "../contracts/proof_verification/IVKProvider.sol";
import "../contracts/proof_verification/RollupProofVerificationKey.sol";
import "../contracts/proof_verification/lib/Types.sol";

contract ReplaceVKFromToml is Script {
    using stdToml for string;

    // e.g. NF4_RUN_MODE=development -> "$.development"
    string public runMode = string.concat("$.", vm.envString("NF4_RUN_MODE"));

    function run(address vkProxy) public {
        vm.setEnv("FOUNDRY_OUT", "blockchain_assets/artifacts");
        require(vkProxy != address(0), "vkProxy=0");
        require(vkProxy.code.length > 0, "VK proxy has no code on RPC_URL");

        // read replacement file
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/nightfall_replace_vk.toml");
        string memory toml = vm.readFile(path);

        // build VK struct
        Types.VerificationKey memory vk = _readVK(toml);
        bytes memory blob = abi.encode(vk);

        // log before/after
        bytes32 oldHash = RollupProofVerificationKey(vkProxy).vkHash();
        console.log("Old vkHash:");
        console.logBytes32(oldHash);

        uint256 pk = vm.envUint("NF4_SIGNING_KEY"); // must be VK owner
        vm.startBroadcast(pk);
        RollupProofVerificationKey(vkProxy).replaceVK(blob);
        vm.stopBroadcast();

        bytes32 newHash = RollupProofVerificationKey(vkProxy).vkHash();
        console.log("New vkHash:");
        console.logBytes32(newHash);
        require(newHash != oldHash, "vkHash unchanged");
    }

    // Convenience: read VK fields from TOML (same shape as deployer)
    function _readVK(string memory toml) internal view returns (Types.VerificationKey memory vk) {
        vk.domain_size = toml.readUint(string.concat(runMode, ".verifier.domain_size"));
        vk.num_inputs  = toml.readUint(string.concat(runMode, ".verifier.num_inputs"));

        vk.sigma_comms_1 = _g1(toml, ".verifier.sigma_comms_1");
        vk.sigma_comms_2 = _g1(toml, ".verifier.sigma_comms_2");
        vk.sigma_comms_3 = _g1(toml, ".verifier.sigma_comms_3");
        vk.sigma_comms_4 = _g1(toml, ".verifier.sigma_comms_4");
        vk.sigma_comms_5 = _g1(toml, ".verifier.sigma_comms_5");
        vk.sigma_comms_6 = _g1(toml, ".verifier.sigma_comms_6");

        vk.selector_comms_1  = _g1(toml, ".verifier.selector_comms_1");
        vk.selector_comms_2  = _g1(toml, ".verifier.selector_comms_2");
        vk.selector_comms_3  = _g1(toml, ".verifier.selector_comms_3");
        vk.selector_comms_4  = _g1(toml, ".verifier.selector_comms_4");
        vk.selector_comms_5  = _g1(toml, ".verifier.selector_comms_5");
        vk.selector_comms_6  = _g1(toml, ".verifier.selector_comms_6");
        vk.selector_comms_7  = _g1(toml, ".verifier.selector_comms_7");
        vk.selector_comms_8  = _g1(toml, ".verifier.selector_comms_8");
        vk.selector_comms_9  = _g1(toml, ".verifier.selector_comms_9");
        vk.selector_comms_10 = _g1(toml, ".verifier.selector_comms_10");
        vk.selector_comms_11 = _g1(toml, ".verifier.selector_comms_11");
        vk.selector_comms_12 = _g1(toml, ".verifier.selector_comms_12");
        vk.selector_comms_13 = _g1(toml, ".verifier.selector_comms_13");
        vk.selector_comms_14 = _g1(toml, ".verifier.selector_comms_14");
        vk.selector_comms_15 = _g1(toml, ".verifier.selector_comms_15");
        vk.selector_comms_16 = _g1(toml, ".verifier.selector_comms_16");
        vk.selector_comms_17 = _g1(toml, ".verifier.selector_comms_17");
        vk.selector_comms_18 = _g1(toml, ".verifier.selector_comms_18");

        vk.k1 = toml.readUint(string.concat(runMode, ".verifier.k1"));
        vk.k2 = toml.readUint(string.concat(runMode, ".verifier.k2"));
        vk.k3 = toml.readUint(string.concat(runMode, ".verifier.k3"));
        vk.k4 = toml.readUint(string.concat(runMode, ".verifier.k4"));
        vk.k5 = toml.readUint(string.concat(runMode, ".verifier.k5"));
        vk.k6 = toml.readUint(string.concat(runMode, ".verifier.k6"));

        vk.range_table_comm   = _g1(toml, ".verifier.range_table_comm");
        vk.key_table_comm     = _g1(toml, ".verifier.key_table_comm");
        vk.table_dom_sep_comm = _g1(toml, ".verifier.table_dom_sep_comm");
        vk.q_dom_sep_comm     = _g1(toml, ".verifier.q_dom_sep_comm");

        vk.size_inv = toml.readUint(string.concat(runMode, ".verifier.size_inv"));
        vk.group_gen = toml.readUint(string.concat(runMode, ".verifier.group_gen"));
        vk.group_gen_inv = toml.readUint(string.concat(runMode, ".verifier.group_gen_inv"));

        vk.open_key_g = _g1(toml, ".verifier.open_key_g");
        vk.h = _g2(toml, ".verifier.h");
        vk.beta_h = _g2(toml, ".verifier.beta_h");
    }

    function _g1(string memory toml, string memory key) internal view returns (Types.G1Point memory p) {
        string[] memory arr = toml.readStringArray(string.concat(runMode, key));
        require(arr.length == 2, "bad G1 array");
        p.x = _hexToUint(arr[0]); p.y = _hexToUint(arr[1]);
    }

    function _g2(string memory toml, string memory key) internal view returns (Types.G2Point memory p) {
        string[] memory arr = toml.readStringArray(string.concat(runMode, key));
        require(arr.length == 4, "bad G2 array");
        p.x0 = _hexToUint(arr[0]); p.x1 = _hexToUint(arr[1]);
        p.y0 = _hexToUint(arr[2]); p.y1 = _hexToUint(arr[3]);
    }

    function _hexToUint(string memory s) internal pure returns (uint256 out) {
        bytes memory b = bytes(s);
        require(b.length >= 3 && b[0] == "0" && (b[1] == "x" || b[1] == "X"), "hex str");
        for (uint256 i = 2; i < b.length; i++) {
            uint8 c = uint8(b[i]); uint8 v;
            if (c >= 0x30 && c <= 0x39) v = c - 0x30;
            else if (c >= 0x41 && c <= 0x46) v = c - 0x41 + 10;
            else if (c >= 0x61 && c <= 0x66) v = c - 0x61 + 10;
            else revert("bad hex");
            out = (out << 4) | uint256(v);
        }
    }
}
