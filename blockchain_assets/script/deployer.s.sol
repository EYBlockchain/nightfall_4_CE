// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script} from "@forge-std/Script.sol";
import "@forge-std/StdToml.sol";

import "../contracts/Nightfall.sol";
import "../contracts/RoundRobinUUPS.sol";

// Verifier stack
import "../contracts/proof_verification/MockVerifier.sol";
import "../contracts/proof_verification/RollupProofVerifierUUPS.sol";
import "../contracts/proof_verification/INFVerifier.sol";
import "../contracts/proof_verification/IVKProvider.sol";
import "../contracts/proof_verification/RollupProofVerificationKeyUUPS.sol";
import "../contracts/proof_verification/lib/Types.sol";

// X509 & sanctions
import "../contracts/X509/X509.sol";
import "../contracts/SanctionsListMock.sol";

// OZ Foundry Upgrades
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import "forge-std/console.sol";
contract Deployer is Script {
    struct RoundRobinConfig {
        address defaultProposerAddress;
        string  defaultProposerUrl;
        uint    stake;
        uint    ding;
        uint    exitPenalty;
        uint    coolingBlocks;
        uint    rotationBlocks;
    }

    using stdToml for string;

    // e.g. NF4_RUN_MODE=local -> "$.local"
    string public runMode = string.concat("$.", vm.envString("NF4_RUN_MODE"));

    function run() external {
        // Make OZ Upgrades use our custom build dir
        vm.setEnv("FOUNDRY_OUT", "blockchain_assets/artifacts");

        uint256 deployerPrivateKey = vm.envUint("NF4_SIGNING_KEY");
        string  memory root = vm.projectRoot();
        string  memory path = string.concat(root, "/nightfall.toml");
        string  memory toml = vm.readFile(path);

        address deployerAddress = vm.addr(deployerPrivateKey);

        // Owners from TOML (fallbacks)
        address verifierOwner = toml.readAddress(string.concat(runMode, ".owners.verifier_owner"));
        if (verifierOwner == address(0)) verifierOwner = deployerAddress;

        address xOwner = toml.readAddress(string.concat(runMode, ".owners.x509_owner"));
        if (xOwner == address(0)) xOwner = deployerAddress;

        // Optional RR owner (Certified has no transferOwnership; see note below)
        address rrOwner = toml.readAddress(string.concat(runMode, ".owners.round_robin_owner"));
        if (rrOwner == address(0)) rrOwner = deployerAddress;

        vm.startBroadcast(deployerPrivateKey);

        // (1) VK provider UUPS proxy (holds full VK blob)
        address vkProxy = deployVKProvider(toml);

        // (2) Verifier & sanctions
        (INFVerifier verifier, SanctionsListInterface sanctionsList) =
            initializeVerifierAndSanctions(toml, vkProxy, verifierOwner);

        // (3) X509 UUPS
        address x509Proxy = Upgrades.deployUUPSProxy(
            "X509.sol:X509",
            abi.encodeCall(X509.initialize, (deployerAddress))
        );
        X509 x509Contract = X509(x509Proxy);
        X509Interface x509 = X509Interface(x509Proxy);

        // Configure X509 while deployer still owns it (optional, for tests)
        if (toml.readBool(string.concat(runMode, ".test_x509_certificates"))) {
            configureX509locally(x509Contract, toml);
        }

        // Transfer X509 ownership if different
        if (xOwner != deployerAddress) {
            x509Contract.transferOwnership(xOwner);
        }

        // (4) Nightfall UUPS proxy (initialize wires verifier/x509/sanctions)
        uint256 initialNullifierRoot = 5626012003977595441102792096342856268135928990590954181023475305010363075697;
        address nightfallProxy = Upgrades.deployUUPSProxy(
            "Nightfall.sol:Nightfall",
            abi.encodeCall(Nightfall.initialize, (initialNullifierRoot, uint256(0), uint256(0), int256(0), verifier, address(x509), address(sanctionsList)))
        );
        Nightfall nightfall = Nightfall(nightfallProxy);

        // (5) RoundRobin UUPS proxy
        RoundRobinConfig memory rr = readRoundRobinConfig(toml);
        address rrProxy = Upgrades.deployUUPSProxy(
            // NOTE: artifact path must match your file name; you import RoundRobinUUPS.sol
            "RoundRobinUUPS.sol:RoundRobin",
            abi.encodeCall(
                RoundRobin.initialize,
                (
                    address(x509),
                    address(sanctionsList),
                    rr.stake,
                    rr.ding,
                    rr.exitPenalty,
                    rr.coolingBlocks,
                    rr.rotationBlocks
                )
            )
        );
        RoundRobin roundRobin = RoundRobin(payable(rrProxy));

        // Bootstrap the default proposer & seed stake (payable step)
        roundRobin.bootstrapDefaultProposer{value: rr.stake}(
            rr.defaultProposerAddress,
            rr.defaultProposerUrl,
            address(nightfall)
        );

        // Wire Nightfall <-> RoundRobin
        nightfall.set_proposer_manager(roundRobin);

        // NOTE on RoundRobin/Nightfall ownership:
        // Certified has no transferOwnership; owner = msg.sender at initialize().
        // If you need rrOwner or a dedicated nightfall owner, either deploy from that key
        // or add transferOwnership() to Certified and call it here.

        vm.stopBroadcast();
    }

    // ---------- Deploy the UUPS VK provider proxy ----------
    function deployVKProvider(string memory toml) internal returns (address vkProxy) {
        // 1) Build full VK from TOML
        Types.VerificationKey memory vk = readVKFromToml(toml);

        // 2) Encode initializer for initialize(bytes)
        bytes memory vkBlob = abi.encode(vk);
        bytes memory init   = abi.encodeWithSignature("initialize(bytes)", vkBlob);

        // 3) Deploy UUPS proxy
        vkProxy = Upgrades.deployUUPSProxy(
            "RollupProofVerificationKeyUUPS.sol:RollupProofVerificationKeyUUPS",
            init
        );

        // 4) Transfer VK provider ownership (if different from msg.sender)
        string memory ownerKey = string.concat(runMode, ".owners.vk_provider_owner");
        address newOwner = toml.readAddress(ownerKey);
        if (newOwner != address(0) && newOwner != msg.sender) {
            RollupProofVerificationKeyUUPS(vkProxy).transferOwnership(newOwner);
        }
    }

    // ---------- Build VK from TOML ----------
    function readVKFromToml(string memory toml) internal view returns (Types.VerificationKey memory vk) {
        vk.domain_size = toml.readUint(string.concat(runMode, ".verifier.domain_size"));
        vk.num_inputs  = toml.readUint(string.concat(runMode, ".verifier.num_inputs"));

        // Sigma commitments (1..6)
        vk.sigma_comms_1 = readG1(toml, ".verifier.sigma_comms_1");
        vk.sigma_comms_2 = readG1(toml, ".verifier.sigma_comms_2");
        vk.sigma_comms_3 = readG1(toml, ".verifier.sigma_comms_3");
        vk.sigma_comms_4 = readG1(toml, ".verifier.sigma_comms_4");
        vk.sigma_comms_5 = readG1(toml, ".verifier.sigma_comms_5");
        vk.sigma_comms_6 = readG1(toml, ".verifier.sigma_comms_6");

        // Selector commitments (1..18)
        vk.selector_comms_1  = readG1(toml, ".verifier.selector_comms_1");
        vk.selector_comms_2  = readG1(toml, ".verifier.selector_comms_2");
        vk.selector_comms_3  = readG1(toml, ".verifier.selector_comms_3");
        vk.selector_comms_4  = readG1(toml, ".verifier.selector_comms_4");
        vk.selector_comms_5  = readG1(toml, ".verifier.selector_comms_5");
        vk.selector_comms_6  = readG1(toml, ".verifier.selector_comms_6");
        vk.selector_comms_7  = readG1(toml, ".verifier.selector_comms_7");
        vk.selector_comms_8  = readG1(toml, ".verifier.selector_comms_8");
        vk.selector_comms_9  = readG1(toml, ".verifier.selector_comms_9");
        vk.selector_comms_10 = readG1(toml, ".verifier.selector_comms_10");
        vk.selector_comms_11 = readG1(toml, ".verifier.selector_comms_11");
        vk.selector_comms_12 = readG1(toml, ".verifier.selector_comms_12");
        vk.selector_comms_13 = readG1(toml, ".verifier.selector_comms_13");
        vk.selector_comms_14 = readG1(toml, ".verifier.selector_comms_14");
        vk.selector_comms_15 = readG1(toml, ".verifier.selector_comms_15");
        vk.selector_comms_16 = readG1(toml, ".verifier.selector_comms_16");
        vk.selector_comms_17 = readG1(toml, ".verifier.selector_comms_17");
        vk.selector_comms_18 = readG1(toml, ".verifier.selector_comms_18");

        // Scalars
        vk.k1 = toml.readUint(string.concat(runMode, ".verifier.k1"));
        vk.k2 = toml.readUint(string.concat(runMode, ".verifier.k2"));
        vk.k3 = toml.readUint(string.concat(runMode, ".verifier.k3"));
        vk.k4 = toml.readUint(string.concat(runMode, ".verifier.k4"));
        vk.k5 = toml.readUint(string.concat(runMode, ".verifier.k5"));
        vk.k6 = toml.readUint(string.concat(runMode, ".verifier.k6"));

        // Table commitments
        vk.range_table_comm   = readG1(toml, ".verifier.range_table_comm");
        vk.key_table_comm     = readG1(toml, ".verifier.key_table_comm");
        vk.table_dom_sep_comm = readG1(toml, ".verifier.table_dom_sep_comm");
        vk.q_dom_sep_comm     = readG1(toml, ".verifier.q_dom_sep_comm");

        // Group params
        vk.size_inv      = toml.readUint(string.concat(runMode, ".verifier.size_inv"));
        vk.group_gen     = toml.readUint(string.concat(runMode, ".verifier.group_gen"));
        vk.group_gen_inv = toml.readUint(string.concat(runMode, ".verifier.group_gen_inv"));

        // Open key
        vk.open_key_g = readG1(toml, ".verifier.open_key_g");

        // G2 points
        vk.h      = readG2(toml, ".verifier.h");
        vk.beta_h = readG2(toml, ".verifier.beta_h");

        return vk;
    }

    // --- helper functions ---
    function readG1(string memory toml, string memory key) internal view returns (Types.G1Point memory p) {
        string[] memory arr = toml.readStringArray(string.concat(runMode, key));
        require(arr.length == 2, "bad G1 array");
        p.x = parseHexToUint256(arr[0]);
        p.y = parseHexToUint256(arr[1]);
    }

    function readG2(string memory toml, string memory key) internal view returns (Types.G2Point memory p) {
        string[] memory arr = toml.readStringArray(string.concat(runMode, key));
        require(arr.length == 4, "bad G2 array");
        p.x0 = parseHexToUint256(arr[0]);
        p.x1 = parseHexToUint256(arr[1]);
        p.y0 = parseHexToUint256(arr[2]);
        p.y1 = parseHexToUint256(arr[3]);
    }

    // ---------- TOML helpers ----------
    function parseHexUintFromArray(string memory toml, string memory key, uint256 i) internal pure returns (uint256) {
        string[] memory arr = toml.readStringArray(key);
        require(i < arr.length, "TOML array index OOB");
        return parseHexToUint256(arr[i]);
    }

    function parseHexToUint256(string memory s) internal pure returns (uint256 out) {
        bytes memory b = bytes(s);
        require(b.length >= 3 && b[0] == "0" && (b[1] == "x" || b[1] == "X"), "hex str");
        uint256 i = 2;
        for (; i < b.length; ++i) {
            uint8 c = uint8(b[i]);
            uint8 v;
            if (c >= 0x30 && c <= 0x39) v = c - 0x30;
            else if (c >= 0x41 && c <= 0x46) v = c - 0x41 + 10;
            else if (c >= 0x61 && c <= 0x66) v = c - 0x61 + 10;
            else revert("bad hex");
            out = (out << 4) | uint256(v);
        }
    }

    function readRoundRobinConfig(string memory toml) internal view returns (RoundRobinConfig memory cfg) {
        cfg.defaultProposerAddress = toml.readAddress(string.concat(runMode, ".nightfall_deployer.default_proposer_address"));
        cfg.defaultProposerUrl     = toml.readString(string.concat(runMode, ".nightfall_deployer.default_proposer_url"));
        cfg.stake                  = toml.readUint(string.concat(runMode, ".nightfall_deployer.proposer_stake"));
        cfg.ding                   = toml.readUint(string.concat(runMode, ".nightfall_deployer.proposer_ding"));
        cfg.exitPenalty            = toml.readUint(string.concat(runMode, ".nightfall_deployer.proposer_exit_penalty"));
        cfg.coolingBlocks          = toml.readUint(string.concat(runMode, ".nightfall_deployer.proposer_cooling_blocks"));
        cfg.rotationBlocks         = toml.readUint(string.concat(runMode, ".nightfall_deployer.proposer_rotation_blocks"));
    }

    // ---------- Verifier & sanctions ----------
    function initializeVerifierAndSanctions(
        string memory toml,
        address vkProxy,
        address initialOwner
    ) internal returns (INFVerifier verifier, SanctionsListInterface sanctionsList) {
        if (toml.readBool(string.concat(runMode, ".test_x509_certificates"))) {
            sanctionsList = new SanctionsListMock(address(0x123456789abcdef1234567890));
        } else {
            sanctionsList = SanctionsListInterface(address(0x40C57923924B5c5c5455c48D93317139ADDaC8fb));
        }

        if (toml.readBool(string.concat(runMode, ".mock_prover"))) {
            verifier = new MockVerifier();
        } else {
            address verifierProxy = Upgrades.deployUUPSProxy(
                "proof_verification/RollupProofVerifierUUPS.sol:RollupProofVerifier",
                abi.encodeCall(RollupProofVerifier.initialize, (vkProxy, initialOwner))
            );
            verifier = INFVerifier(verifierProxy);
        }
    }

    // ---------- X509 helpers ----------

    function _readFileIfExists(string memory p) internal view returns (bytes memory data, bool ok) {
        try vm.readFileBinary(p) returns (bytes memory b) { return (b, true); }
        catch { return ("", false); }
    }

    function configureX509locally(X509 x509Contract, string memory toml) internal {
        uint256 authorityKeyIdentifier = toml.readUint(string.concat(runMode, ".certificates.authority_key_identifier"));
        bytes memory modulus = vm.parseBytes(toml.readString(string.concat(runMode, ".certificates.modulus")));
        uint256 exponent = toml.readUint(string.concat(runMode, ".certificates.exponent"));

        X509.RSAPublicKey memory nightfallRootPublicKey = X509.RSAPublicKey({
            modulus: modulus,
            exponent: exponent
        });

        x509Contract.setTrustedPublicKey(nightfallRootPublicKey, authorityKeyIdentifier);
        x509Contract.enableAllowlisting(true);

        configureExtendedKeyUsages(x509Contract, toml);
        configureCertificatePolicies(x509Contract, toml);

        // bytes memory intermediate_ca_derBuffer = vm.readFileBinary(
        //     "./blockchain_assets/test_contracts/X509/_certificates/intermediate_ca.der"
        // );

        // --- Robust file read (absolute path + existence check) ---
        string memory root = vm.projectRoot(); 
        string memory certPath = string.concat(
            root,
            "/blockchain_assets/test_contracts/X509/_certificates/intermediate_ca.der"
        ); 

        (bytes memory intermediate_ca_derBuffer, bool ok) = _readFileIfExists(certPath); 
        require(ok, "Missing intermediate_ca.der at expected path");                     


        uint256 intermediate_ca_tlvLength = x509Contract.computeNumberOfTlvs(intermediate_ca_derBuffer, 0);
        X509.CertificateArgs memory intermediate_certificate_args = X509.CertificateArgs({
            certificate: intermediate_ca_derBuffer,
            tlvLength: intermediate_ca_tlvLength,
            addressSignature: "",
            isEndUser: false,
            checkOnly: false,
            oidGroup: 0,
            addr: address(0)
        });
        x509Contract.validateCertificate(intermediate_certificate_args);
    }

    function configureExtendedKeyUsages(X509 x509Contract, string memory toml) internal {
        string[] memory extendedKeyUsages = toml.readStringArray(string.concat(runMode, ".certificates.extended_key_usages"));
        bytes32[] memory extendedKeyUsageOIDs = new bytes32[](extendedKeyUsages.length);
        for (uint i = 0; i < extendedKeyUsages.length; i++) {
            extendedKeyUsageOIDs[i] = parseHexStringToBytes32(extendedKeyUsages[i]);
        }
        x509Contract.addExtendedKeyUsage(extendedKeyUsageOIDs);
    }

    function configureCertificatePolicies(X509 x509Contract, string memory toml) internal {
        string[] memory certificatePolicies = toml.readStringArray(string.concat(runMode, ".certificates.certificate_policies"));
        bytes32[] memory certificatePoliciesOIDs = new bytes32[](certificatePolicies.length);
        for (uint256 i = 0; i < certificatePolicies.length; i++) {
            certificatePoliciesOIDs[i] = parseHexStringToBytes32(certificatePolicies[i]);
        }
        x509Contract.addCertificatePolicies(certificatePoliciesOIDs);
    }

    // --- Utilities ---
    function parseHexStringToBytes32(string memory s) internal pure returns (bytes32) {
        bytes memory ss = bytes(s);
        require(ss.length == 66, "Invalid hex string length");
        bytes memory hexData = new bytes(32);
        for (uint256 i = 2; i < 66; i += 2) {
            hexData[(i - 2) / 2] = bytes1(parseHexChar(ss[i]) * 16 + parseHexChar(ss[i + 1]));
        }
        return bytes32(hexData);
    }

    function parseHexChar(bytes1 c) internal pure returns (uint8) {
        if (c >= bytes1("0") && c <= bytes1("9")) return uint8(c) - uint8(bytes1("0"));
        if (c >= bytes1("a") && c <= bytes1("f")) return 10 + uint8(c) - uint8(bytes1("a"));
        if (c >= bytes1("A") && c <= bytes1("F")) return 10 + uint8(c) - uint8(bytes1("A"));
        revert("Invalid hex character");
    }
}
