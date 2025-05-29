// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script} from "@forge-std/Script.sol";
import "@forge-std/StdToml.sol";
import "../contracts/Nightfall.sol";
import "../contracts/RoundRobin.sol";
import "../contracts/proof_verification/MockVerifier.sol";
import "../contracts/proof_verification/RollupVerifier.sol";
import "../contracts/proof_verification/INFVerifier.sol";
import "../contracts/X509/X509.sol";
import "forge-std/console2.sol";
import "../contracts/SanctionsListMock.sol";
import "../contracts/X509/Certified.sol";

contract Deployer is Script {
    struct RoundRobinConfig {
    address defaultProposerAddress;
    string defaultProposerUrl;
    uint stake;
    uint ding;
    uint exitPenalty;
    uint coolingBlocks;
    uint rotationBlocks;
}
    using stdToml for string;

    // we adjust the toml stanza that we use depending on the run mode
    string public runMode = string.concat("$.", vm.envString("NF4_RUN_MODE"));

    function run() external {
        uint256 deployerPrivateKey = vm.envUint("NF4_SIGNING_KEY");
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/nightfall.toml");
        string memory toml = vm.readFile(path);
        string memory profile = vm.envString("NF4_RUN_MODE");
        profile = string.concat("$.", profile);

        address deployerAddress = vm.addr(deployerPrivateKey);

        vm.startBroadcast(deployerPrivateKey);

        INFVerifier verifier;

        SanctionsListInterface sanctionsList;

        // Initialize Verifier and Sanctions List
        (verifier, sanctionsList) = initializeVerifierAndSanctions(toml);

        X509 x509Contract = new X509(deployerAddress);
        X509Interface x509 = X509Interface(address(x509Contract));

        Certified certified = new Certified(x509, sanctionsList);
        Nightfall nightfall = new Nightfall(
            verifier,
            address(x509),
            address(sanctionsList)
        );
        
        RoundRobinConfig memory rrConfig = readRoundRobinConfig(toml);

        RoundRobin roundRobin = new RoundRobin{value: rrConfig.stake}(
            rrConfig.defaultProposerAddress,
            rrConfig.defaultProposerUrl,
            rrConfig.stake,
            rrConfig.ding,
            rrConfig.exitPenalty,
            rrConfig.coolingBlocks,
            rrConfig.rotationBlocks
        );


        // We let deployer to configure X509 contract with essential root information such as modulus, exponent, extended key usages, and certificate policies
        // Currently we only have the configuration for local development
        // Todo: add configuration for production, such as adding EY certificate root information or others
        if (toml.readBool(string.concat(runMode, ".test_x509_certificates"))) {
            configureX509locally(x509Contract, toml);
        }
        // Set the authorities for the Certified contract, which are the SanctionsList (I mean Chainalysis sanctions oracle) and X509 contracts
        certified.setAuthorities(address(sanctionsList), address(x509));
        nightfall.set_x509_address(address(x509));
        nightfall.set_sanctions_list(address(sanctionsList));
        nightfall.set_proposer_manager(roundRobin);
        roundRobin.set_nightfall(address(nightfall));

        vm.stopBroadcast();
    }

    function readRoundRobinConfig(string memory toml)
    internal
    view
    returns (RoundRobinConfig memory)
{
    RoundRobinConfig memory config;
    config.defaultProposerAddress = toml.readAddress(string.concat(runMode, ".nightfall_deployer.default_proposer_address"));
    config.defaultProposerUrl = toml.readString(string.concat(runMode, ".nightfall_deployer.default_proposer_url"));
    config.stake = toml.readUint(string.concat(runMode, ".nightfall_deployer.proposer_stake"));
    config.ding = toml.readUint(string.concat(runMode, ".nightfall_deployer.proposer_ding"));
    config.exitPenalty = toml.readUint(string.concat(runMode, ".nightfall_deployer.proposer_exit_penalty"));
    config.coolingBlocks = toml.readUint(string.concat(runMode, ".nightfall_deployer.proposer_cooling_blocks"));
    config.rotationBlocks = toml.readUint(string.concat(runMode, ".nightfall_deployer.proposer_rotation_blocks"));
    console2.log(
        "RoundRobinConfig: %s, %s, %s",
        config.stake,
        config.ding,
        config.exitPenalty
    );
    console2.log(
        "RoundRobinConfig: %s, %s",
        config.coolingBlocks,
        config.rotationBlocks
    );
    return config;
}

    function initializeVerifierAndSanctions(
        string memory toml
    )
        internal
        returns (INFVerifier verifier, SanctionsListInterface sanctionsList)
    {
        // for a local development, we will use MockVerifier and SanctionsListMock,
        // SanctionsListMock is a mock contract that implements SanctionsListInterface by adding a random address to SanctionsList

        // for production, we will use nightfish_CEVerifier and SanctionsListInterface,
        // If it's ethereum mainnet, we will use the address 0x40C57923924B5c5c5455c48D93317139ADDaC8fb of the Chainalysis sanctions oracle
        // for other networks, please change this value accordingly, see https://www.chainalysis.com/solutions/sanctions-screening/
        string memory mockProver = string.concat(runMode, ".mock_prover");
        if (toml.readBool(string.concat(runMode, ".test_x509_certificates"))) {
            sanctionsList = new SanctionsListMock(
                address(0x123456789abcdef1234567890)
            );
        } else {
            sanctionsList = SanctionsListInterface(
                address(0x40C57923924B5c5c5455c48D93317139ADDaC8fb)
            );
        }
        if (toml.readBool(mockProver)) {
            verifier = new MockVerifier();
        } else {
            verifier = new RollupVerifier();
        }
    }

    function configureX509locally(X509 x509Contract, string memory toml) internal {
        uint256 authorityKeyIdentifier = toml.readUint(string.concat(runMode, ".certificates.authority_key_identifier"));
        bytes memory modulus = vm.parseBytes(toml.readString(string.concat(runMode, ".certificates.modulus")));
        uint256 exponent = toml.readUint(string.concat(runMode, ".certificates.exponent"));

        X509.RSAPublicKey memory nightfallRootPublicKey = X509.RSAPublicKey({
            modulus: modulus,
            exponent: exponent
        });

        x509Contract.setTrustedPublicKey(
            nightfallRootPublicKey,
            authorityKeyIdentifier
        );
        x509Contract.enableAllowlisting(true);

        configureExtendedKeyUsages(x509Contract, toml);
        configureCertificatePolicies(x509Contract, toml);

        // Validate intermediate certificate
        bytes memory intermediate_ca_derBuffer = vm.readFileBinary(
            "./blockchain_assets/test_contracts/X509/_certificates/intermediate_ca.der"
        );
        uint256 intermediate_ca_tlvLength = x509Contract.computeNumberOfTlvs(
            intermediate_ca_derBuffer,
            0
        );
        // Intermediate CA certificate validation
        X509.CertificateArgs memory intermediate_certificate_args = X509
            .CertificateArgs({
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
            certificatePoliciesOIDs[i] = parseHexStringToBytes32(
                certificatePolicies[i]
            );
        }
        x509Contract.addCertificatePolicies(certificatePoliciesOIDs);
    }

    // Helper function to parse a hex string to bytes32
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

    // Helper function to parse a single hex character to its decimal value
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
}
