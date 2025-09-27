// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script} from "@forge-std/Script.sol";
import "@forge-std/StdToml.sol";

import "../contracts/Nightfall.sol";
import "../contracts/RoundRobin.sol";

// Verifier stack
import "../contracts/proof_verification/MockVerifier.sol";
import "../contracts/proof_verification/RollupProofVerifier.sol";
import "../contracts/proof_verification/INFVerifier.sol";
import "../contracts/proof_verification/IVKProvider.sol";
import "../contracts/proof_verification/RollupProofVerificationKey.sol";
import "../contracts/proof_verification/lib/Types.sol";

// X509 & sanctions
import "../contracts/X509/X509.sol";
import "../contracts/SanctionsListMock.sol";

// OZ Foundry Upgrades
import {Upgrades} from "@openzeppelin/foundry-upgrades/Upgrades.sol";
import "forge-std/console.sol";

contract Deployer is Script {
    using stdToml for string;

    struct Owners {
        address deployer;
        address verifierOwner;
        address x509Owner;
        address roundRobinOwner;
        address nightfallOwner;
        uint256 deployerPk;
    }

    struct Deployed {
        address vkProxy;
        INFVerifier verifier;
        SanctionsListInterface sanctionsList;
        address x509Proxy;
        X509Interface x509;
        address nightfallProxy;
        Nightfall nightfall;
        address roundRobinProxy;
        RoundRobin roundRobin;
    }

    // e.g. NF4_RUN_MODE=local -> "$.local"
    string public runMode = string.concat("$.", vm.envString("NF4_RUN_MODE"));

    // ---------------- entrypoint ----------------
    function run() external {
        vm.setEnv("FOUNDRY_OUT", "blockchain_assets/artifacts");

        string memory toml = _loadToml();
        Owners memory owners = _owners(toml);

        // 1) VK   Verifier   X509 (deployer)
        Deployed memory deployed;
        (
            deployed.vkProxy,
            deployed.verifier,
            deployed.sanctionsList
        ) = _deployVerifierStack(toml, owners);

        (deployed.x509Proxy, deployed.x509) = _deployX509(toml, owners);

        // 2) Nightfall (owned by deployer initially)
        (deployed.nightfallProxy, deployed.nightfall) = _deployNightfall(
            owners,
            deployed.verifier,
            deployed.x509,
            deployed.sanctionsList
        );

        // 3) RoundRobin   bootstrap
        (deployed.roundRobinProxy, deployed.roundRobin) = _deployRoundRobin(
            toml,
            owners,
            deployed.x509,
            deployed.sanctionsList,
            deployed.nightfall
        );

        // 4) Wire Nightfall -> RoundRobin while Nightfall is still deployer-owned
        _wireNightfallToRR(owners, deployed.nightfall, deployed.roundRobin);

        // 5) Transfer Nightfall ownership to TOML value (same flow as X509)
        _maybeTransferNightfallOwnership(owners, deployed.nightfall);

        // 6) Transfer RoundRobin ownership to TOML value
        _maybeTransferRoundRobinOwnership(owners, deployed.roundRobin);

        _log(deployed, owners);
    }

    // ---------------- helpers ----------------

    function _loadToml() internal view returns (string memory toml) {
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/nightfall.toml");
        toml = vm.readFile(path);
    }

    function _owners(
        string memory toml
    ) internal view returns (Owners memory owners) {
        owners.deployerPk = vm.envUint("NF4_SIGNING_KEY");
        owners.deployer = vm.addr(owners.deployerPk);

        // read owners (fallback to deployer)
        address verifierOwner = toml.readAddress(
            string.concat(runMode, ".owners.verifier_owner")
        );
        address x509Owner = toml.readAddress(
            string.concat(runMode, ".owners.x509_owner")
        );
        address roundRobinOwner = toml.readAddress(
            string.concat(runMode, ".owners.round_robin_owner")
        );
        address nfOwner = toml.readAddress(
            string.concat(runMode, ".owners.nightfall_owner")
        );

        owners.verifierOwner = (verifierOwner == address(0))
            ? owners.deployer
            : verifierOwner;
        owners.x509Owner = (x509Owner == address(0))
            ? owners.deployer
            : x509Owner;
        owners.roundRobinOwner = (roundRobinOwner == address(0))
            ? owners.deployer
            : roundRobinOwner;
        owners.nightfallOwner = (nfOwner == address(0))
            ? owners.deployer
            : nfOwner;
    }

    function _deployVerifierStack(
        string memory toml,
        Owners memory owners
    )
        internal
        returns (
            address vkProxy,
            INFVerifier verifier,
            SanctionsListInterface sanctionsList
        )
    {
        vm.startBroadcast(owners.deployerPk);

        vkProxy = _deployVKProvider(toml);

        // sanctions
        if (toml.readBool(string.concat(runMode, ".test_x509_certificates"))) {
            sanctionsList = new SanctionsListMock(
                address(0x123456789abcdef1234567890)
            );
        } else {
            sanctionsList = SanctionsListInterface(
                address(0x40C57923924B5c5c5455c48D93317139ADDaC8fb)
            );
        }

        // verifier
        if (toml.readBool(string.concat(runMode, ".mock_prover"))) {
            verifier = new MockVerifier();
        } else {
            address verifierProxy = Upgrades.deployUUPSProxy(
                "RollupProofVerifier.sol:RollupProofVerifier",
                abi.encodeCall(
                    RollupProofVerifier.initialize,
                    (vkProxy, owners.verifierOwner)
                )
            );
            verifier = INFVerifier(verifierProxy);
        }

        vm.stopBroadcast();
    }

    function _deployX509(
        string memory toml,
        Owners memory owners
    ) internal returns (address x509Proxy, X509Interface x509) {
        vm.startBroadcast(owners.deployerPk);

        x509Proxy = Upgrades.deployUUPSProxy(
            "X509.sol:X509",
            abi.encodeCall(X509.initialize, (owners.deployer))
        );

        X509 x509Impl = X509(x509Proxy);
        x509 = X509Interface(x509Proxy);

        if (toml.readBool(string.concat(runMode, ".test_x509_certificates"))) {
            _configureX509locally(x509Impl, toml);
        }

        if (owners.x509Owner != owners.deployer) {
            x509Impl.transferOwnership(owners.x509Owner);
        }

        vm.stopBroadcast();
    }

    function _deployNightfall(
        Owners memory owners,
        INFVerifier verifier,
        X509Interface x509,
        SanctionsListInterface sanctionsList
    ) internal returns (address nightfallProxy, Nightfall nightfall) {
        vm.startBroadcast(owners.deployerPk);

        uint256 initialNullifierRoot = 5626012003977595441102792096342856268135928990590954181023475305010363075697;

        nightfallProxy = Upgrades.deployUUPSProxy(
            "Nightfall.sol:Nightfall",
            abi.encodeCall(
                Nightfall.initialize,
                (
                    initialNullifierRoot,
                    uint256(0),
                    uint256(0),
                    int256(0),
                    verifier,
                    address(x509),
                    address(sanctionsList)
                )
            )
        );
        nightfall = Nightfall(nightfallProxy);

        vm.stopBroadcast();
    }

    function _maybeTransferNightfallOwnership(
        Owners memory owners,
        Nightfall nightfall
    ) internal {
        if (owners.nightfallOwner != owners.deployer) {
            vm.startBroadcast(owners.deployerPk);
            nightfall.transferOwnership(owners.nightfallOwner);
            vm.stopBroadcast();
        }
    }

    function _maybeTransferRoundRobinOwnership(
        Owners memory owners,
        RoundRobin rr
    ) internal {
        if (owners.roundRobinOwner != owners.deployer) {
            vm.startBroadcast(owners.deployerPk);
            rr.transferOwnership(owners.roundRobinOwner);
            vm.stopBroadcast();
        }
    }

    function _deployRoundRobin(
        string memory toml,
        Owners memory owners,
        X509Interface x509,
        SanctionsListInterface sanctionsList,
        Nightfall nightfall
    ) internal returns (address roundRobinProxy, RoundRobin rr) {
        RoundRobinConfig memory cfg = _readRoundRobinConfig(toml);

        vm.startBroadcast(owners.deployerPk);

        roundRobinProxy = Upgrades.deployUUPSProxy(
            "RoundRobin.sol:RoundRobin",
            abi.encodeCall(
                RoundRobin.initialize,
                (
                    address(x509),
                    address(sanctionsList),
                    cfg.stake,
                    cfg.ding,
                    cfg.exitPenalty,
                    cfg.coolingBlocks,
                    cfg.rotationBlocks
                )
            )
        );
        rr = RoundRobin(payable(roundRobinProxy));
        rr.set_nightfall(address(nightfall));
        rr.bootstrapDefaultProposer{value: cfg.stake}(
            cfg.defaultProposerAddress,
            cfg.defaultProposerUrl,
            address(nightfall)
        );

        address cp = rr.get_current_proposer_address();
        require(
            cp != address(0),
            "RoundRobin bootstrap failed: current proposer is zero"
        );

        vm.stopBroadcast();
    }

    function _wireNightfallToRR(
        Owners memory owners,
        Nightfall nightfall,
        RoundRobin rr
    ) internal {
        vm.startBroadcast(owners.deployerPk);
        nightfall.set_proposer_manager(rr);
        vm.stopBroadcast();
    }

    // ---------- VK provider ----------
    function _deployVKProvider(
        string memory toml
    ) internal returns (address vkProxy) {
        Types.VerificationKey memory vk = _readVK(toml);

        // minimal safety net
        VKSanity.sanityCheckVK(vk);

        bytes memory init = abi.encodeWithSignature(
            "initialize(bytes)",
            abi.encode(vk)
        );
        vkProxy = Upgrades.deployUUPSProxy(
            "RollupProofVerificationKey.sol:RollupProofVerificationKey",
            init
        );

        address newOwner = toml.readAddress(
            string.concat(runMode, ".owners.vk_provider_owner")
        );
        if (newOwner != address(0) && newOwner != msg.sender) {
            RollupProofVerificationKey(vkProxy).transferOwnership(newOwner);
        }
    }

    function _readVK(
        string memory toml
    ) internal view returns (Types.VerificationKey memory vk) {
        vk.domain_size = toml.readUint(
            string.concat(runMode, ".verifier.domain_size")
        );
        vk.num_inputs = toml.readUint(
            string.concat(runMode, ".verifier.num_inputs")
        );

        vk.sigma_comms_1 = _g1(toml, ".verifier.sigma_comms_1");
        vk.sigma_comms_2 = _g1(toml, ".verifier.sigma_comms_2");
        vk.sigma_comms_3 = _g1(toml, ".verifier.sigma_comms_3");
        vk.sigma_comms_4 = _g1(toml, ".verifier.sigma_comms_4");
        vk.sigma_comms_5 = _g1(toml, ".verifier.sigma_comms_5");
        vk.sigma_comms_6 = _g1(toml, ".verifier.sigma_comms_6");

        vk.selector_comms_1 = _g1(toml, ".verifier.selector_comms_1");
        vk.selector_comms_2 = _g1(toml, ".verifier.selector_comms_2");
        vk.selector_comms_3 = _g1(toml, ".verifier.selector_comms_3");
        vk.selector_comms_4 = _g1(toml, ".verifier.selector_comms_4");
        vk.selector_comms_5 = _g1(toml, ".verifier.selector_comms_5");
        vk.selector_comms_6 = _g1(toml, ".verifier.selector_comms_6");
        vk.selector_comms_7 = _g1(toml, ".verifier.selector_comms_7");
        vk.selector_comms_8 = _g1(toml, ".verifier.selector_comms_8");
        vk.selector_comms_9 = _g1(toml, ".verifier.selector_comms_9");
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

        vk.range_table_comm = _g1(toml, ".verifier.range_table_comm");
        vk.key_table_comm = _g1(toml, ".verifier.key_table_comm");
        vk.table_dom_sep_comm = _g1(toml, ".verifier.table_dom_sep_comm");
        vk.q_dom_sep_comm = _g1(toml, ".verifier.q_dom_sep_comm");

        vk.size_inv = toml.readUint(
            string.concat(runMode, ".verifier.size_inv")
        );
        vk.group_gen = toml.readUint(
            string.concat(runMode, ".verifier.group_gen")
        );
        vk.group_gen_inv = toml.readUint(
            string.concat(runMode, ".verifier.group_gen_inv")
        );

        vk.open_key_g = _g1(toml, ".verifier.open_key_g");
        vk.h = _g2(toml, ".verifier.h");
        vk.beta_h = _g2(toml, ".verifier.beta_h");
    }

    function _g1(
        string memory toml,
        string memory key
    ) internal view returns (Types.G1Point memory p) {
        string[] memory arr = toml.readStringArray(string.concat(runMode, key));
        require(arr.length == 2, "bad G1 array");
        p.x = _hexToUint(arr[0]);
        p.y = _hexToUint(arr[1]);
    }

    function _g2(
        string memory toml,
        string memory key
    ) internal view returns (Types.G2Point memory p) {
        string[] memory arr = toml.readStringArray(string.concat(runMode, key));
        require(arr.length == 4, "bad G2 array");
        p.x0 = _hexToUint(arr[0]);
        p.x1 = _hexToUint(arr[1]);
        p.y0 = _hexToUint(arr[2]);
        p.y1 = _hexToUint(arr[3]);
    }

    function _hexToUint(string memory s) internal pure returns (uint256 out) {
        bytes memory b = bytes(s);
        // Explicit casting of single-char string literals to bytes1:
        require(
            b.length >= 3 &&
                b[0] == bytes1("0") &&
                (b[1] == bytes1("x") || b[1] == bytes1("X")),
            "hex str"
        );
        for (uint256 i = 2; i < b.length; i++) {
            uint8 c = uint8(b[i]);
            uint8 v;
            if (c >= 0x30 && c <= 0x39) v = c - 0x30;
            else if (c >= 0x41 && c <= 0x46) v = c - 0x41 + 10;
            else if (c >= 0x61 && c <= 0x66) v = c - 0x61 + 10;
            else revert("bad hex");
            out = (out << 4) | uint256(v);
        }
    }

    // ---------- RoundRobin ----------
    struct RoundRobinConfig {
        address defaultProposerAddress;
        string defaultProposerUrl;
        uint stake;
        uint ding;
        uint exitPenalty;
        uint coolingBlocks;
        uint rotationBlocks;
    }

    function _readRoundRobinConfig(
        string memory toml
    ) internal view returns (RoundRobinConfig memory cfg) {
        cfg.defaultProposerAddress = toml.readAddress(
            string.concat(
                runMode,
                ".nightfall_deployer.default_proposer_address"
            )
        );
        cfg.defaultProposerUrl = toml.readString(
            string.concat(runMode, ".nightfall_deployer.default_proposer_url")
        );
        cfg.stake = toml.readUint(
            string.concat(runMode, ".nightfall_deployer.proposer_stake")
        );
        cfg.ding = toml.readUint(
            string.concat(runMode, ".nightfall_deployer.proposer_ding")
        );
        cfg.exitPenalty = toml.readUint(
            string.concat(runMode, ".nightfall_deployer.proposer_exit_penalty")
        );
        cfg.coolingBlocks = toml.readUint(
            string.concat(
                runMode,
                ".nightfall_deployer.proposer_cooling_blocks"
            )
        );
        cfg.rotationBlocks = toml.readUint(
            string.concat(
                runMode,
                ".nightfall_deployer.proposer_rotation_blocks"
            )
        );
    }

    // ---------- X509 local config ----------
    function _readFileIfExists(
        string memory p
    ) internal view returns (bytes memory data, bool ok) {
        try vm.readFileBinary(p) returns (bytes memory b) {
            return (b, true);
        } catch {
            return ("", false);
        }
    }

    function _configureX509locally(
        X509 x509Contract,
        string memory toml
    ) internal {
        console.log("inside _configureX509locally");
        uint256 authorityKeyIdentifier = toml.readUint(
            string.concat(runMode, ".certificates.authority_key_identifier")
        );
        bytes memory modulus = vm.parseBytes(
            toml.readString(string.concat(runMode, ".certificates.modulus"))
        );
        uint256 exponent = toml.readUint(
            string.concat(runMode, ".certificates.exponent")
        );

        X509.RSAPublicKey memory nightfallRootPublicKey = X509.RSAPublicKey({
            modulus: modulus,
            exponent: exponent
        });

        x509Contract.setTrustedPublicKey(
            nightfallRootPublicKey,
            authorityKeyIdentifier
        );
        x509Contract.enableAllowlisting(true);

        _configureExtendedKeyUsages(x509Contract, toml);
        _configureCertificatePolicies(x509Contract, toml);

        string memory pr = vm.projectRoot();
        string memory certPath = string.concat(
            pr,
            "/blockchain_assets/test_contracts/X509/_certificates/intermediate_ca.der"
        );

        (bytes memory interDER, bool ok) = _readFileIfExists(certPath);
        require(ok, "Missing intermediate_ca.der");

        uint256 tlv = x509Contract.computeNumberOfTlvs(interDER, 0);
        X509.CertificateArgs memory args = X509.CertificateArgs({
            certificate: interDER,
            tlvLength: tlv,
            addressSignature: "",
            isEndUser: false,
            checkOnly: false,
            oidGroup: 0,
            addr: address(0)
        });
        x509Contract.validateCertificate(args);
    }

    function _configureExtendedKeyUsages(
        X509 x509Contract,
        string memory toml
    ) internal {
        string[] memory extendedKeyUsages = toml.readStringArray(
            string.concat(runMode, ".certificates.extended_key_usages")
        );
        bytes32[] memory extendedKeyUsageOIDs = new bytes32[](
            extendedKeyUsages.length
        );
        for (uint i = 0; i < extendedKeyUsages.length; i++) {
            extendedKeyUsageOIDs[i] = parseHexStringToBytes32(
                extendedKeyUsages[i]
            );
        }
        x509Contract.addExtendedKeyUsage(extendedKeyUsageOIDs);
    }

    function _configureCertificatePolicies(
        X509 x509Contract,
        string memory toml
    ) internal {
        string[] memory certificatePolicies = toml.readStringArray(
            string.concat(runMode, ".certificates.certificate_policies")
        );
        bytes32[] memory certificatePoliciesOIDs = new bytes32[](
            certificatePolicies.length
        );
        for (uint256 i = 0; i < certificatePolicies.length; i++) {
            certificatePoliciesOIDs[i] = parseHexStringToBytes32(
                certificatePolicies[i]
            );
        }
        x509Contract.addCertificatePolicies(certificatePoliciesOIDs);
    }

    function parseHexStringToBytes32(
        string memory s
    ) internal pure returns (bytes32) {
        bytes memory ss = bytes(s);
        require(ss.length == 66, "Invalid hex string length");

        bytes memory hexData = new bytes(32);
        for (uint256 i = 2; i < 66; i += 2) {
            hexData[(i - 2) / 2] = bytes1(
                parseHexChar(ss[i]) * 16 + parseHexChar(ss[i + 1])
            );
        }
        return bytes32(hexData);
    }

    function parseHexChar(bytes1 c) internal pure returns (uint8) {
        if (c >= bytes1("0") && c <= bytes1("9")) {
            return uint8(c) - uint8(bytes1("0"));
        }
        if (c >= bytes1("a") && c <= bytes1("f")) {
            return 10 + uint8(c) - uint8(bytes1("a"));
        }
        if (c >= bytes1("A") && c <= bytes1("F")) {
            return 10 + uint8(c) - uint8(bytes1("A"));
        }
        revert("Invalid hex character");
    }

    // ---------- logs ----------
    function _log(
        Deployed memory deployed,
        Owners memory owners
    ) internal pure {
        console.log("Nightfall proxy:       ", deployed.nightfallProxy);
        console.log("Nightfall owner:       ", owners.nightfallOwner);
        console.log("RoundRobin proxy:      ", deployed.roundRobinProxy);
        console.log("RoundRobin owner:      ", owners.roundRobinOwner);
        console.log("X509 proxy:            ", deployed.x509Proxy);
        console.log("X509 owner:            ", owners.x509Owner);
        console.log("VK provider proxy:     ", deployed.vkProxy);
        console.log("Verifier proxy:        ", address(deployed.verifier));
        console.log("Verifier owner:        ", owners.verifierOwner);
    }
}

library VKSanity {
    // BN254 field moduli
    uint256 constant P =
        21888242871839275222246405745257275088696311157297823662689037894645226208583; // F_p
    uint256 constant R =
        21888242871839275222246405745257275088548364400416034343698204186575808495617; // F_r

    // --- tiny utils ---
    function _isPow2(uint256 x) private pure returns (bool) {
        return x != 0 && (x & (x - 1)) == 0;
    }

    // Modexp precompile (0x05), for exponentiation mod R
    function _modexp(
        uint256 base,
        uint256 e,
        uint256 m
    ) private view returns (uint256 r) {
        // big-endian lengths (32/32/32) + values
        bytes memory input = abi.encodePacked(
            uint256(32),
            uint256(32),
            uint256(32),
            base,
            e,
            m
        );
        assembly {
            if iszero(
                staticcall(
                    gas(),
                    0x05,
                    add(input, 0x20),
                    mload(input),
                    0x00,
                    0x20
                )
            ) {
                revert(0, 0)
            }
            r := mload(0x00)
        }
    }

    // G1 on-curve check: y^2 == x^3 + 3 (mod P), coordinates < P, disallow (0,0)
    function _isOnCurveG1(uint256 x, uint256 y) private pure returns (bool) {
        if (x == 0 && y == 0) return false; // no “infinity” encoding
        if (x >= P || y >= P) return false;
        uint256 y2 = mulmod(y, y, P);
        uint256 x2 = mulmod(x, x, P);
        uint256 x3 = mulmod(x2, x, P);
        uint256 rhs = addmod(x3, 3, P);
        return y2 == rhs;
    }

    // Pairwise distinct for small fixed arrays
    function _allDistinct(uint256[6] memory a) private pure returns (bool) {
        for (uint256 i = 0; i < 6; ++i) {
            for (uint256 j = i + 1; j < 6; ++j) {
                if (a[i] == a[j]) return false;
            }
        }
        return true;
    }

    function sanityCheckVK(Types.VerificationKey memory vk) internal view {
        // --- Scalars in F_r ---
        require(_isPow2(vk.domain_size), "vk: domain_size !pow2");
        require(vk.domain_size < R, "vk: domain_size >= r");
        require(
            mulmod(vk.size_inv, vk.domain_size % R, R) == 1,
            "vk: size_inv mismatch"
        );
        require(
            mulmod(vk.group_gen, vk.group_gen_inv, R) == 1,
            "vk: group_gen_inv mismatch"
        );

        // primitive n-th root sanity: w^n == 1 and (if n>1) w^(n/2) != 1
        uint256 wN = _modexp(vk.group_gen % R, vk.domain_size, R);
        require(wN == 1, "vk: w^n != 1");
        if (vk.domain_size > 1) {
            uint256 wHalf = _modexp(vk.group_gen % R, vk.domain_size >> 1, R);
            require(wHalf != 1, "vk: w order < n");
        }

        // k1..k6 in (0, r), pairwise distinct
        uint256[6] memory ks = [vk.k1, vk.k2, vk.k3, vk.k4, vk.k5, vk.k6];
        for (uint256 i = 0; i < 6; ++i) {
            require(ks[i] > 0 && ks[i] < R, "vk: k_i out of range");
        }
        require(_allDistinct(ks), "vk: k_i not distinct");

        // --- A few representative G1 points (cheap but effective) ---
        require(
            _isOnCurveG1(vk.sigma_comms_1.x, vk.sigma_comms_1.y),
            "vk: sigma1 !G1"
        );
        require(
            _isOnCurveG1(vk.selector_comms_1.x, vk.selector_comms_1.y),
            "vk: selector1 !G1"
        );
        require(
            _isOnCurveG1(vk.open_key_g.x, vk.open_key_g.y),
            "vk: open_key_g !G1"
        );

        // --- G2 bounds + nonzero (kept simple) ---
        // (Full twisted on-curve check is longer; this catches common encoding/field errors.)
        require(
            vk.h.x0 < P && vk.h.x1 < P && vk.h.y0 < P && vk.h.y1 < P,
            "vk: h out of Fp"
        );
        require(
            vk.beta_h.x0 < P &&
                vk.beta_h.x1 < P &&
                vk.beta_h.y0 < P &&
                vk.beta_h.y1 < P,
            "vk: beta_h out of Fp"
        );
        require((vk.h.x0 | vk.h.x1 | vk.h.y0 | vk.h.y1) != 0, "vk: h zero");
        require(
            (vk.beta_h.x0 | vk.beta_h.x1 | vk.beta_h.y0 | vk.beta_h.y1) != 0,
            "vk: beta_h zero"
        );
    }
}
