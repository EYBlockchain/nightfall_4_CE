// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.19;

import "forge-std/Test.sol";
import "../../contracts/X509/X509.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract X509Test is Test {
    X509 x509;
    DERParser derParser;

    // Foundry's default sender EOA
    address constant TEST_EOA = 0x1804c8AB1F12E6bbf3894d4083f33e07309d1f38;

    function setUp() public {
        // Upgradeable contract: deploy then initialize owner
        // IMPORTANT: since the implementation has `constructor(){ _disableInitializers(); }`
        // we must initialize THROUGH THE PROXY, not by calling initialize on the impl.
        X509 x509Impl = new X509();
        bytes memory x509Init = abi.encodeCall(X509.initialize, (address(this)));
        x509 = X509(address(new ERC1967Proxy(address(x509Impl), x509Init)));
        
        derParser = new DERParser();
    }

    function test_X509_valid_cert() public {
        uint256 testTimestamp = 1748995201;
        vm.warp(testTimestamp);

        // openssl x509 -in root_ca.crt -text -noout | grep -A 1 "Authority Key Identifier"
        bytes
            memory authorityKeyIdentifierBytes = hex"A469FF28BFAB9C4DB09220B24038D6F18EA57F75";
        uint256 authorityKeyIdentifier = bytesToUint256(
            authorityKeyIdentifierBytes
        );
        // openssl rsa -in root_ca_priv_key.pem -modulus -noout
        // get modulus from private key of root_ca, don't forget to add "00" at the beginning
        bytes
            memory modulus = hex"009DEA9DCA80BFA87C29232B18D6C0072898922A7E7E224A7FF638F61851B5F36392E7FBFDBFF3A0AE409763E2A04CDD3DC692A6DE447391FFE6722456957DD7F031B8D9A7999579F6F4258490AE6E9D629BC40815F689C58037C03B46502243BFD29B02116454453810D160DE1D8C8DDD624B30A25A011185E60BCA9BF71181DD3256112F1EFDBECF19E77AF9640EDE4DB8FF51855E6B490424FC4F5631DD9551D7CD762420E3AFA0B20E6B403A0CB71FA16861F8C591B2BD7BDD564EC6D5A17A932E310876D1D65AF3F3F213D1C49086F32C7C8A0F53750127DF8709F6035688E02E613F1C57A525A21DD83FA27D0622FC0EFC76ABA114194A7FDA1B0879013D0790F3B8D387ACA238FC37135F9BA6BB0C87A972143568B010B62EE8BA71C78202858170F292596AD95DD4FA2DC8E9ABA359B8F511B5F3894906F3FD0A22CA3DEB2E67B2A97CD2B847AC73BE28F69996A4CF51B6FD87B9F932F6049F886AC5A7725755693842DF00795A9D00C76E2C4446BDDA5E595CBE8CDF51E050632DB110D155343188A57F273B4334E5DA5EC556AD3CADC3327268DC0C528FE41F837A393B5B2F76E476CFA64A2A24BA71F5F7078F5360EBF316D4275AB292B031B9CF8787ACB009D3DC5DCD5038C05E1B2225909E596DFE2E968CFAE077FDFF540E3F78FC464966BB19E280DE34F81079B9DCA111904CDC7C5B6FD5CD44A215B0B5A6A9";

        // openssl x509 -in root_ca.crt -text -noout | grep -A 1 "Exponent"
        // find Exponent: 65537 (0x10001)
        uint256 exponent = 65537;
        X509.RSAPublicKey memory nightfallRootPublicKey = X509.RSAPublicKey(
            modulus,
            exponent
        );

        x509.setTrustedPublicKey(
            nightfallRootPublicKey,
            authorityKeyIdentifier
        );
        x509.enableAllowlisting(true);

        addExtendedKeyUsages();
        addCertificatePolicies();

        validateCertificates();
    }

    function addExtendedKeyUsages() internal {
        // openssl x509 -in user/user-1.der -text -noout | grep -A 1 "Extended Key Usage"
        // The OID for "E-mail Protection" is 1.3.6.1.5.5.7.3.4, and the OID for "Time Stamping" is 1.3.6.1.5.5.7.3.8.
        bytes32[] memory extendedKeyUsageOIDs_0 = new bytes32[](2);
        extendedKeyUsageOIDs_0[0] = hex"06082b06010505070304"; // E-mail Protection
        extendedKeyUsageOIDs_0[1] = hex"06082b06010505070308"; // Time Stamping
        x509.addExtendedKeyUsage(extendedKeyUsageOIDs_0);
    }

    // openssl x509 -noout -text -in user/user-1.der | grep -A 1 "Certificate Policies"
    function addCertificatePolicies() internal {
        bytes32[] memory certificatePoliciesOIDs_0 = new bytes32[](1);
        certificatePoliciesOIDs_0[
            0
        ] = hex"06032d0607000000000000000000000000000000000000000000000000000000";
        x509.addCertificatePolicies(certificatePoliciesOIDs_0);
    }

    function validateCertificates() internal {
        bytes memory intermediate_ca_derBuffer = vm.readFileBinary(
            "./blockchain_assets/test_contracts/X509/_certificates/intermediate_ca.der"
        );
        uint256 intermediate_ca_tlvLength = x509.computeNumberOfTlvs(
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
        x509.validateCertificate(intermediate_certificate_args);

        // Signature is generated seperatelly in lib/src/validate_certificate.rs: fn sign_ethereum_address(der_private_key: &[u8],address: &H160,)
        // foundry default sender address is 0x1804c8AB1F12E6bbf3894d4083f33e07309d1f38.
        // Or, you can just run test_sign_and_verify_ethereum_address to get new signature.
        bytes
            memory signature = hex"4dfeab8f396d2e444e48c9fdd4417737236a25643f869e4538f42fb9cc8f28ededaa26ff07e2f3c66e151ba56fbe8cf480f944f706ca760d9f5c0a1c216679dee03282f3195a42c24aa810926b881c0994fc6acd8b1c8b9aeb7f798e65d0f836e47efb41b5094aa59e8fdb0b0cd830f5fd79db3218068f0a89a1117afe7f1ef8abb1315c212c851a9bc9e05208d7ab176a3ce927a0b0f1baa13a6a8aa7361b8d9a1f137ac43169fbf3c2f025d349200634f6273a771c46b5479f6208dfbfafa28a6c5773145a7f6a2658e4a7f06b82ac55bf4e93a25ec505a48b12854c2e34c0cd5106d8e07082c2ebfb3708965604d4204b468a9eef24d544038113fdecdcef6e60197d88fd6dd7aeb8b0f282d3aab1aa5f2b9b603cde8250159873577d270a48c748392f14e41d9aed32a92e7435d835276a2ac550d8a05c9cce271c76077a6114afed76ed62a75bc8fbd4d5cc3471051955b0258458b71fdf539c6eb48e7f517ec86dc690036cb15de8aa6459b9894b1f0db4055bf0f13aae23897397d8aa0c6530c91926d5bd04d74eeb920f19c0bef187e87c4640474c3b3dfd2d3e6b601c4982cc6fb64d2cb6a834d4500156aa11c7ba0864bcb03affa460ec9f240092b5bb5095966b85fd1ee0b06e52f6f12033a772d01d6269c01a0d72af833180774a8e1ab0a5c984b3f79bccb195f38ccefbac7fc833741daa69a8c72f2dffcc1e";

        bytes memory endUserCert_derBuffer = vm.readFileBinary(
            "./blockchain_assets/test_contracts/X509/_certificates/user/user-2.der"
        );
        uint256 endUserCert_tlvLength = x509.computeNumberOfTlvs(
            endUserCert_derBuffer,
            0
        );
        // test that one can't bind an Arbitrary Ethereum Address to a Certificate Without Proving Control of the Ethereum Account, as the signature will not match
        
        X509.CertificateArgs memory notendUser_certificate_args = X509
            .CertificateArgs({
                certificate: endUserCert_derBuffer,
                tlvLength: endUserCert_tlvLength,
                addressSignature: signature,
                isEndUser: true,
                checkOnly: false,
                oidGroup: 0,
                addr: address(0x1234567890123456789012345678901234567890)
            });
        vm.expectRevert("X509: You can only allowlist your own address");
        x509.validateCertificate(notendUser_certificate_args);
        
        vm.startPrank(TEST_EOA);
        // Check invalid oid group rejection
        X509.CertificateArgs memory invalid_oid_certificate_args = X509
            .CertificateArgs({
                certificate: endUserCert_derBuffer,
                tlvLength: endUserCert_tlvLength,
                addressSignature: signature,
                isEndUser: true,
                checkOnly: false,
                oidGroup: 1, 
                addr: msg.sender
            });
        vm.expectRevert("X509: OID group does not match allowed EKUs and Certificate Policies");
        x509.validateCertificate(invalid_oid_certificate_args);



        X509.CertificateArgs memory endUser_certificate_args = X509
            .CertificateArgs({
                certificate: endUserCert_derBuffer,
                tlvLength: endUserCert_tlvLength,
                addressSignature: signature,
                isEndUser: true,
                checkOnly: false,
                oidGroup: 0,
                addr: msg.sender
            });
        x509.validateCertificate(endUser_certificate_args);

        // test that one address uses multiple certificates
        bytes memory endUserCert_derBuffer_2 = vm.readFileBinary(
            "./blockchain_assets/test_contracts/X509/_certificates/user/user-1.der"
        );
        uint256 endUserCert_tlvLength_2 = x509.computeNumberOfTlvs(
            endUserCert_derBuffer_2,
            0
        );

        bytes
            memory newSignature_2 = hex"049076a0cc62a97e8b224edb32f091196e8d521a097d4a715d6cb508fcfa513f48efa2e6fe2aa1610077a718db6b9872f76ade46ffc1236d12513a6c3d2268420acd1efb5d8555794c3d65b444a6757f09edcbc50b0dbabd31eb3e2faa8e4bcd8c4a297d8646e1cb7aebfcb4069a7c3582560a816b0b8ff338486e9e2110c8562415e35f6896bad1d0168f814566896b87a35748dc32de4de20ff0d2265a55d4688d906b4ac1fffab8725e14aeb4103d9e61d892029f8b3ee9028b2eccc55ef85978c645d2ff3d730426599a41674f70bf9f2074de85c2d951a670b048e5a45ec2b5067b80889ee424f41e0a5e094a4da7716b90b0772166f88fc204089c9b71dbad3aea93206b8c226c5dba8d237655afc6b34d1ff3a2f118cbbf774b84efee3e7fac78838d35dbdb74d405a9225b5c5e2fee1aa2aedf4674d4abf5a41f80d95aa6b9c66198c53a74bcc9d129522afebf63d92982bf51842b383462098b4317fa84daab4865556980642364f205ff99e658d1c58be4781583490f858cd145c3987d55ab5559b260fed2292d04b30e2dfbfa92bf91db99b0977a2d952c01c6bc8b806a3321784ac5a0b2d691485b124378029ac322d466edd1ccf2a7ebbd23561c2725ea029462d6523c51881d1df3c89d7dd34fe5147f7ca4ca6b3e0405bf3e4a38e7b769b1b7b0ad88e45272e45ec50b0083945e39a38fb6f0270eafc8d8e9";

        X509.CertificateArgs memory endUser_certificate_args_4 = X509
            .CertificateArgs({
                certificate: endUserCert_derBuffer_2,
                tlvLength: endUserCert_tlvLength_2,
                addressSignature: newSignature_2,
                isEndUser: true,
                checkOnly: false,
                oidGroup: 0,
                addr: msg.sender
            });
        vm.expectRevert(
            "X509: This address is already linked to a different certificate"
        );
        x509.validateCertificate(endUser_certificate_args_4);

        // test revoking a certificate
        // We get the Subject Key Identifier of the first certificate
        uint256 subjectKeyIdentifier = 54796968659936144882787592062789517993011334687;
        x509.revokeKeyFromUserAddress(subjectKeyIdentifier);
        vm.expectRevert(
            "X509: The subject key of this certificate has been revoked"
        );
        x509.validateCertificate(endUser_certificate_args);
        vm.stopPrank();
    }

    function bytesToUint256(bytes memory b) internal pure returns (uint256) {
        uint256 number;
        for (uint i = 0; i < b.length; i++) {
            number =
                number +
                uint256(uint8(b[i])) * (2 ** (8 * (b.length - (i + 1))));
        }
        return number;
    }
}
