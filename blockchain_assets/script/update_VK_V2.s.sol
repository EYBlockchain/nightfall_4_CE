// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script} from "@forge-std/Script.sol";
import "@forge-std/StdToml.sol";
import "forge-std/console.sol";

import "../contracts/proof_verification/lib/Types.sol";
import "../contracts/proof_verification/RollupProofVerificationKey.sol";

/// Update the stored Verification Key on the VK provider UUPS proxy.
/// Reads VK fields from nightfall.toml using NF4_RUN_MODE (e.g. "local").
/// Requires NF4_SIGNING_KEY (must be the proxy owner).
contract UpdateVKWithToml is Script {
    using stdToml for string;

    // e.g. NF4_RUN_MODE=local -> "$.local"
    string public runMode = string.concat("$.", vm.envString("NF4_RUN_MODE"));

    // ---------- Entry points ----------
    /// Use VK proxy from env: VK_PROXY
    function run() external {
        address vkProxy = vm.envAddress("VK_PROXY");
        _update(vkProxy);
    }

    /// Pass VK proxy explicitly
    function run(address vkProxy) external {
        _update(vkProxy);
    }

    // ---------- Core ----------
    function _update(address vkProxy) internal {
        require(vkProxy != address(0), "VK proxy is zero");

        // Keep OZ upgrades happy with custom out dir 
        vm.setEnv("FOUNDRY_OUT", "blockchain_assets/artifacts");

        // Load TOML
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/nightfall.toml");
        string memory toml = vm.readFile(path);

        // Build new VK from TOML
        Types.VerificationKey memory vk = _readVKFromToml(toml);
        bytes memory vkBlob = abi.encode(vk);

        // Introspect before/after
        RollupProofVerificationKey target = RollupProofVerificationKey(vkProxy);
        bytes32 beforeHash = target.vkHash();
        uint64  beforeVer  = target.vkVersion();

        console.log("== VK Update ==");
        console.log("Proxy:   ", vkProxy);
        console.logBytes32(beforeHash);
        console.log("version: ", beforeVer);

        uint256 pk = vm.envUint("NF4_SIGNING_KEY"); // must be owner
        vm.startBroadcast(pk);
        target.replaceVK(vkBlob);
        vm.stopBroadcast();

        bytes32 afterHash = target.vkHash();
        uint64  afterVer  = target.vkVersion();

        console.log("== VK Updated ==");
        console.logBytes32(afterHash);
        console.log("version: ", afterVer);

        require(afterHash != bytes32(0), "afterHash zero");
        require(afterHash != beforeHash, "VK hash did not change");
        require(afterVer  == beforeVer + 1, "VK version did not increment");
    }

    // ---------- VK readers from TOML ----------
    function _readVKFromToml(string memory toml) internal view returns (Types.VerificationKey memory vk) {
        vk.domain_size = toml.readUint(string.concat(runMode, ".verifier.domain_size"));
        vk.num_inputs  = toml.readUint(string.concat(runMode, ".verifier.num_inputs"));

        // Sigma commitments (1..6)
        vk.sigma_comms_1 = _readG1(toml, ".verifier.sigma_comms_1");
        vk.sigma_comms_2 = _readG1(toml, ".verifier.sigma_comms_2");
        vk.sigma_comms_3 = _readG1(toml, ".verifier.sigma_comms_3");
        vk.sigma_comms_4 = _readG1(toml, ".verifier.sigma_comms_4");
        vk.sigma_comms_5 = _readG1(toml, ".verifier.sigma_comms_5");
        vk.sigma_comms_6 = _readG1(toml, ".verifier.sigma_comms_6");

        // Selector commitments (1..18)
        vk.selector_comms_1  = _readG1(toml, ".verifier.selector_comms_1");
        vk.selector_comms_2  = _readG1(toml, ".verifier.selector_comms_2");
        vk.selector_comms_3  = _readG1(toml, ".verifier.selector_comms_3");
        vk.selector_comms_4  = _readG1(toml, ".verifier.selector_comms_4");
        vk.selector_comms_5  = _readG1(toml, ".verifier.selector_comms_5");
        vk.selector_comms_6  = _readG1(toml, ".verifier.selector_comms_6");
        vk.selector_comms_7  = _readG1(toml, ".verifier.selector_comms_7");
        vk.selector_comms_8  = _readG1(toml, ".verifier.selector_comms_8");
        vk.selector_comms_9  = _readG1(toml, ".verifier.selector_comms_9");
        vk.selector_comms_10 = _readG1(toml, ".verifier.selector_comms_10");
        vk.selector_comms_11 = _readG1(toml, ".verifier.selector_comms_11");
        vk.selector_comms_12 = _readG1(toml, ".verifier.selector_comms_12");
        vk.selector_comms_13 = _readG1(toml, ".verifier.selector_comms_13");
        vk.selector_comms_14 = _readG1(toml, ".verifier.selector_comms_14");
        vk.selector_comms_15 = _readG1(toml, ".verifier.selector_comms_15");
        vk.selector_comms_16 = _readG1(toml, ".verifier.selector_comms_16");
        vk.selector_comms_17 = _readG1(toml, ".verifier.selector_comms_17");
        vk.selector_comms_18 = _readG1(toml, ".verifier.selector_comms_18");

        // Scalars
        vk.k1 = toml.readUint(string.concat(runMode, ".verifier.k1"));
        vk.k2 = toml.readUint(string.concat(runMode, ".verifier.k2"));
        vk.k3 = toml.readUint(string.concat(runMode, ".verifier.k3"));
        vk.k4 = toml.readUint(string.concat(runMode, ".verifier.k4"));
        vk.k5 = toml.readUint(string.concat(runMode, ".verifier.k5"));
        vk.k6 = toml.readUint(string.concat(runMode, ".verifier.k6"));

        // Table commitments
        vk.range_table_comm = _readG1(toml, ".verifier.range_table_comm");
        vk.key_table_comm = _readG1(toml, ".verifier.key_table_comm");
        vk.table_dom_sep_comm = _readG1(toml, ".verifier.table_dom_sep_comm");
        vk.q_dom_sep_comm = _readG1(toml, ".verifier.q_dom_sep_comm");

        // Group params
        vk.size_inv = toml.readUint(string.concat(runMode, ".verifier.size_inv"));
        vk.group_gen = toml.readUint(string.concat(runMode, ".verifier.group_gen"));
        vk.group_gen_inv = toml.readUint(string.concat(runMode, ".verifier.group_gen_inv"));

        // Open key
        vk.open_key_g = _readG1(toml, ".verifier.open_key_g");

        // G2 points
        vk.h = _readG2(toml, ".verifier.h");
        vk.beta_h = _readG2(toml, ".verifier.beta_h");
    }

    // --- helpers (same style as your deployer) ---
    function _readG1(string memory toml, string memory key) internal view returns (Types.G1Point memory p) {
        string[] memory arr = toml.readStringArray(string.concat(runMode, key));
        require(arr.length == 2, "bad G1 array");
        p.x = _parseHexToUint256(arr[0]);
        p.y = _parseHexToUint256(arr[1]);
    }

    function _readG2(string memory toml, string memory key) internal view returns (Types.G2Point memory p) {
        string[] memory arr = toml.readStringArray(string.concat(runMode, key));
        require(arr.length == 4, "bad G2 array");
        p.x0 = _parseHexToUint256(arr[0]);
        p.x1 = _parseHexToUint256(arr[1]);
        p.y0 = _parseHexToUint256(arr[2]);
        p.y1 = _parseHexToUint256(arr[3]);
    }

    function _parseHexToUint256(string memory s) internal pure returns (uint256 out) {
        bytes memory b = bytes(s);
        require(b.length >= 3 && b[0] == "0" && (b[1] == "x" || b[1] == "X"), "hex str");
        for (uint256 i = 2; i < b.length; ++i) {
            uint8 c = uint8(b[i]);
            uint8 v;
            if (c >= 0x30 && c <= 0x39) v = c - 0x30;
            else if (c >= 0x41 && c <= 0x46) v = c - 0x41 + 10;
            else if (c >= 0x61 && c <= 0x66) v = c - 0x61 + 10;
            else revert("bad hex");
            out = (out << 4) | uint256(v);
        }
    }
}
