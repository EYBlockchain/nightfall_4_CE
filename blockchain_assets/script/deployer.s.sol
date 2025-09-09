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
    using stdToml for string;

    struct Owners {
        address deployer;
        address verifierOwner;
        address xOwner;
        address rrOwner;
        address nightfallOwner;
        uint256 deployerPk;
        uint256 nightfallOwnerPk; // == deployerPk if same owner
    }

    struct Deployed {
        address vkProxy;
        INFVerifier verifier;
        SanctionsListInterface sanctionsList;
        address x509Proxy;
        X509Interface x509;
        address nightfallProxy;
        Nightfall nightfall;
        address rrProxy;
        RoundRobin roundRobin;
    }

    // e.g. NF4_RUN_MODE=local -> "$.local"
    string public runMode = string.concat("$.", vm.envString("NF4_RUN_MODE"));

    // ---------------- entrypoint ----------------
    function run() external {
        vm.setEnv("FOUNDRY_OUT", "blockchain_assets/artifacts");

        string memory toml = _loadToml();
        Owners memory o = _owners(toml);

        // 1) VK + Verifier + X509 (deployer)
        Deployed memory d;
        (d.vkProxy, d.verifier, d.sanctionsList) = _deployVerifierStack(toml, o);

        (d.x509Proxy, d.x509) = _deployX509(toml, o);

        // 2) Nightfall (deployer), then transfer to nightfallOwner
        (d.nightfallProxy, d.nightfall) = _deployNightfall(o, d.verifier, d.x509, d.sanctionsList);
        _maybeTransferNightfallOwnership(o, d.nightfall);

        // 3) RoundRobin + bootstrap (deployer owns RR)
        (d.rrProxy, d.roundRobin) = _deployRoundRobin(toml, o, d.x509, d.sanctionsList, d.nightfall);

        // 4) Wire Nightfall -> RR (must be Nightfall owner)
        _wireNightfallToRR(o, d.nightfall, d.roundRobin);

        _log(d, o);
    }

    // ---------------- helpers ----------------

    function _loadToml() internal view returns (string memory toml) {
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/nightfall.toml");
        toml = vm.readFile(path);
    }

    function _owners(string memory toml) internal view returns (Owners memory o) {
        o.deployerPk = vm.envUint("NF4_SIGNING_KEY");
        o.deployer   = vm.addr(o.deployerPk);

        // read owners (fallback to deployer)
        address verifierOwner = toml.readAddress(string.concat(runMode, ".owners.verifier_owner"));
        address xOwner        = toml.readAddress(string.concat(runMode, ".owners.x509_owner"));
        address rrOwner       = toml.readAddress(string.concat(runMode, ".owners.round_robin_owner"));
        address nfOwner       = toml.readAddress(string.concat(runMode, ".owners.nightfall_owner"));

        o.verifierOwner  = (verifierOwner == address(0)) ? o.deployer : verifierOwner;
        o.xOwner         = (xOwner        == address(0)) ? o.deployer : xOwner;
        o.rrOwner        = (rrOwner       == address(0)) ? o.deployer : rrOwner;
        o.nightfallOwner = (nfOwner       == address(0)) ? o.deployer : nfOwner;

        // pick a key for Nightfall-owner-only calls
        if (o.nightfallOwner == o.deployer) {
            o.nightfallOwnerPk = o.deployerPk;
        } else {
            uint256 nfPk = vm.envUint("NF4_NIGHTFALL_OWNER_KEY");
            require(vm.addr(nfPk) == o.nightfallOwner, "NF4_NIGHTFALL_OWNER_KEY != nightfall_owner");
            o.nightfallOwnerPk = nfPk;
        }
    }

    function _deployVerifierStack(
        string memory toml,
        Owners memory o
    ) internal returns (
        address vkProxy,
        INFVerifier verifier,
        SanctionsListInterface sanctionsList
    ) {
        vm.startBroadcast(o.deployerPk);

        vkProxy = _deployVKProvider(toml);

        // sanctions
        if (toml.readBool(string.concat(runMode, ".test_x509_certificates"))) {
            sanctionsList = new SanctionsListMock(address(0x123456789abcdef1234567890));
        } else {
            sanctionsList = SanctionsListInterface(address(0x40C57923924B5c5c5455c48D93317139ADDaC8fb));
        }

        // verifier
        if (toml.readBool(string.concat(runMode, ".mock_prover"))) {
            verifier = new MockVerifier();
        } else {
            address verifierProxy = Upgrades.deployUUPSProxy(
                "proof_verification/RollupProofVerifierUUPS.sol:RollupProofVerifier",
                abi.encodeCall(RollupProofVerifier.initialize, (vkProxy, o.verifierOwner))
            );
            verifier = INFVerifier(verifierProxy);
        }

        vm.stopBroadcast();
    }

    function _deployX509(
        string memory toml,
        Owners memory o
    ) internal returns (address x509Proxy, X509Interface x509) {
        vm.startBroadcast(o.deployerPk);

        x509Proxy = Upgrades.deployUUPSProxy(
            "X509.sol:X509",
            abi.encodeCall(X509.initialize, (o.deployer))
        );

        X509 x509Impl = X509(x509Proxy);
        x509 = X509Interface(x509Proxy);

        if (toml.readBool(string.concat(runMode, ".test_x509_certificates"))) {
            _configureX509locally(x509Impl, toml);
        }

        if (o.xOwner != o.deployer) {
            x509Impl.transferOwnership(o.xOwner);
        }

        vm.stopBroadcast();
    }

    function _deployNightfall(
        Owners memory o,
        INFVerifier verifier,
        X509Interface x509,
        SanctionsListInterface sanctionsList
    ) internal returns (address nightfallProxy, Nightfall nightfall) {
        vm.startBroadcast(o.deployerPk);

        uint256 initialNullifierRoot =
            5626012003977595441102792096342856268135928990590954181023475305010363075697;

        nightfallProxy = Upgrades.deployUUPSProxy(
            "Nightfall.sol:Nightfall",
            abi.encodeCall(
                Nightfall.initialize,
                (initialNullifierRoot, uint256(0), uint256(0), int256(0), verifier, address(x509), address(sanctionsList))
            )
        );
        nightfall = Nightfall(nightfallProxy);

        vm.stopBroadcast();
    }

    function _maybeTransferNightfallOwnership(Owners memory o, Nightfall nightfall) internal {
        if (o.nightfallOwner != o.deployer) {
            vm.startBroadcast(o.deployerPk);
            nightfall.transferOwnership(o.nightfallOwner);
            vm.stopBroadcast();
        }
    }

    function _deployRoundRobin(
        string memory toml,
        Owners memory o,
        X509Interface x509,
        SanctionsListInterface sanctionsList,
        Nightfall nightfall
    ) internal returns (address rrProxy, RoundRobin rr) {
        RoundRobinConfig memory cfg = _readRR(toml);

        vm.startBroadcast(o.deployerPk);

        rrProxy = Upgrades.deployUUPSProxy(
            "RoundRobinUUPS.sol:RoundRobin",
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
        rr = RoundRobin(payable(rrProxy));
        rr.set_nightfall(address(nightfall));
        rr.bootstrapDefaultProposer{value: cfg.stake}(
            cfg.defaultProposerAddress,
            cfg.defaultProposerUrl,
            address(nightfall)
        );

        address cp = rr.get_current_proposer_address();
        require(cp != address(0), "RoundRobin bootstrap failed: current proposer is zero");

        vm.stopBroadcast();
    }

    function _wireNightfallToRR(
        Owners memory o,
        Nightfall nightfall,
        RoundRobin rr
    ) internal {
        // owner-only on Nightfall; sign with nightfallOwner key
        vm.startBroadcast(o.nightfallOwnerPk);
        nightfall.set_proposer_manager(rr);
        vm.stopBroadcast();
    }

    // ---------- VK provider ----------
    function _deployVKProvider(string memory toml) internal returns (address vkProxy) {
        Types.VerificationKey memory vk = _readVK(toml);
        bytes memory init = abi.encodeWithSignature("initialize(bytes)", abi.encode(vk));
        vkProxy = Upgrades.deployUUPSProxy(
            "RollupProofVerificationKeyUUPS.sol:RollupProofVerificationKeyUUPS",
            init
        );

        // optional transfer (if specified)
        address newOwner = toml.readAddress(string.concat(runMode, ".owners.vk_provider_owner"));
        if (newOwner != address(0) && newOwner != msg.sender) {
            RollupProofVerificationKeyUUPS(vkProxy).transferOwnership(newOwner);
        }
    }

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

        vk.size_inv      = toml.readUint(string.concat(runMode, ".verifier.size_inv"));
        vk.group_gen     = toml.readUint(string.concat(runMode, ".verifier.group_gen"));
        vk.group_gen_inv = toml.readUint(string.concat(runMode, ".verifier.group_gen_inv"));

        vk.open_key_g = _g1(toml, ".verifier.open_key_g");
        vk.h      = _g2(toml, ".verifier.h");
        vk.beta_h = _g2(toml, ".verifier.beta_h");
    }

    function _g1(string memory toml, string memory key) internal view returns (Types.G1Point memory p) {
        string[] memory arr = toml.readStringArray(string.concat(runMode, key));
        require(arr.length == 2, "bad G1 array");
        p.x = _hexToUint(arr[0]);
        p.y = _hexToUint(arr[1]);
    }

    function _g2(string memory toml, string memory key) internal view returns (Types.G2Point memory p) {
        string[] memory arr = toml.readStringArray(string.concat(runMode, key));
        require(arr.length == 4, "bad G2 array");
        p.x0 = _hexToUint(arr[0]);
        p.x1 = _hexToUint(arr[1]);
        p.y0 = _hexToUint(arr[2]);
        p.y1 = _hexToUint(arr[3]);
    }

    function _hexToUint(string memory s) internal pure returns (uint256 out) {
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

    // ---------- RoundRobin ----------
    struct RoundRobinConfig {
        address defaultProposerAddress;
        string  defaultProposerUrl;
        uint    stake;
        uint    ding;
        uint    exitPenalty;
        uint    coolingBlocks;
        uint    rotationBlocks;
    }

    function _readRR(string memory toml) internal view returns (RoundRobinConfig memory cfg) {
        cfg.defaultProposerAddress = toml.readAddress(string.concat(runMode, ".nightfall_deployer.default_proposer_address"));
        cfg.defaultProposerUrl     = toml.readString(string.concat(runMode, ".nightfall_deployer.default_proposer_url"));
        cfg.stake                  = toml.readUint(string.concat(runMode, ".nightfall_deployer.proposer_stake"));
        cfg.ding                   = toml.readUint(string.concat(runMode, ".nightfall_deployer.proposer_ding"));
        cfg.exitPenalty            = toml.readUint(string.concat(runMode, ".nightfall_deployer.proposer_exit_penalty"));
        cfg.coolingBlocks          = toml.readUint(string.concat(runMode, ".nightfall_deployer.proposer_cooling_blocks"));
        cfg.rotationBlocks         = toml.readUint(string.concat(runMode, ".nightfall_deployer.proposer_rotation_blocks"));
    }

    // ---------- X509 local config ----------
    function _readFileIfExists(string memory p) internal view returns (bytes memory data, bool ok) {
        try vm.readFileBinary(p) returns (bytes memory b) { return (b, true); }
        catch { return ("", false); }
    }

    function _configureX509locally(X509 x509Contract, string memory toml) internal {
        uint256 authorityKeyIdentifier = toml.readUint(string.concat(runMode, ".certificates.authority_key_identifier"));
        bytes memory modulus = vm.parseBytes(toml.readString(string.concat(runMode, ".certificates.modulus")));
        uint256 exponent = toml.readUint(string.concat(runMode, ".certificates.exponent"));

        X509.RSAPublicKey memory nightfallRootPublicKey = X509.RSAPublicKey({
            modulus: modulus,
            exponent: exponent
        });

        x509Contract.setTrustedPublicKey(nightfallRootPublicKey, authorityKeyIdentifier);
        x509Contract.enableAllowlisting(true);

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

    // ---------- logs ----------
    function _log(Deployed memory d, Owners memory o) internal pure {
        console.log("Nightfall proxy:       ", d.nightfallProxy);
        console.log("Nightfall owner:       ", o.nightfallOwner);
        console.log("RoundRobin proxy:      ", d.rrProxy);
        console.log("X509 proxy:            ", d.x509Proxy);
        console.log("VK provider proxy:     ", d.vkProxy);
        console.log("Verifier owner:        ", o.verifierOwner);
        console.log("X509 owner:            ", o.xOwner);
        console.log("RoundRobin owner:      ", o.rrOwner);
    }
}
