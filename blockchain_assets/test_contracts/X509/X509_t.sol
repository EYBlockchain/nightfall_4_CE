// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.19;

import "forge-std/Test.sol";
import "../../contracts/X509/X509.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "../../contracts/X509/Sha.sol";

contract X509Test is Test {
    X509 x509;
    DERParser derParser;
    Sha sha512Impl;

    // Foundry's default sender EOA
    address constant TEST_EOA = 0x1804c8AB1F12E6bbf3894d4083f33e07309d1f38;

    function setUp() public {
        // Deploy the SHA-512 helper contract
        sha512Impl = new Sha();
        // Upgradeable contract: deploy then initialize owner
        // IMPORTANT: since the implementation has `constructor(){ _disableInitializers(); }`
        // we must initialize THROUGH THE PROXY, not by calling initialize on the impl.
        X509 x509Impl = new X509();
        bytes memory x509Init = abi.encodeCall(X509.initialize, (address(this)));
        x509 = X509(address(new ERC1967Proxy(address(x509Impl), x509Init)));
        x509.setSha512Impl(address(sha512Impl));
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
            memory signature = hex"24e7a91caccd63be870b86f8adfb25e420e296b9bf147886c316599925492cf3ed246ce9f32d58b5b09f9b2f892ef669e42d0bfc9e3f3737e65ac31bbc6f102f5ffe08611a1aea16aa4e24b3c4aea0690c96ac99ffdf5104a5a6fad767020852fea7ddfe8f11b5197bc44549338500166855e448f47a515bcbfbcaa892cfe7d759fd324408b41338ce21cf24d2aea5cac510085c466def2a4417804b34e455b3356fdbbd3ace52bf64e2022d4eb2c165ff1868a7b926911326ddec74e0c4f12fab260d2b4547247b877e79385a9a66b6fefc29f730cf9d59dddb183afb4b3e36fdb173b01ee0dc5166fc8ccbb8d5022570751b6710515bb1d6a3c46973ae60f3c45bac43d2644aee21def90f05b12c04bda71afa19ee5db04ce6dfb5cc3dbe5db7842520926e3784cfdaa8c72ac14ce5e38d66ae1086ed6bd98a63f0f21c93cae3f69e9e88abb96b08f901d172009b6bd18f8ce0bce68335ad5edd487703a11a2c48c8932eee698149740a8fba84bd4c9ed8f1861e52a07331c58f2bc95fe8c8a270b7bc64ed89a08cda6ba358513973b68f655d7b5676047b834dbd39b1c4eb8572c57a02c7c2ddad3c4089a32416f02f812e9cc91b1e5b3381b0985e83d56de8abd1bbecbddf0c36cd1663615046fa12fd0f5f1912c84dfccbf2654e3abe5c691a992a1f389129575d79f7ab115f514d436113f88cb08f58d9e0d982edea60";

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
            memory newSignature_2 = hex"1a9130475ef4834d5a842ba62ccea33d27be9fa32039f5d6d5d31076aca0383b763a54d4ccdaa8e273d4098885677ceb2d4aad7b13bd79e32b2f7fa90a5e570dc62c6fee9e8bdc6cd4912fe3d93fcfb0ecfc45edaadc1a8d27999a2355b7667fe87cd420c1942bd7b2c15f1552dcb71062f70e70c230b4921736afabf5f4bd494f8cf21d487e5b5cdc8cc9e425091d913ba886b781acd74bc47fc4bdaecd75192d3df226fdee44349203ab686f11434b4711041ebb5d1777e691725bc43abb58f39230f535506c9ca2cb40fde3ed075f49e02e02be6b2c029eccf340d376c1a2c0195582497241b054bf5123dd39d31d35fb3461248dba642dd8baed8ccd7bb31be703ff517a6fc50b082bafe185d12bec14e342fae6c3cb33b349e309830130b2ee5d94d4d84d507268e2c06690836c0ce732985c57b8d4bf3b2f5d8e494768a2a5b89a5952c3f1de645ae0b13d171c6890d96067614d96757a684a3dfac722cc612a6dc7b5402b337419bc63627b0f176cf366985956309730a1641e8baf2e4e27b02c3c137e528aba63a677e328cb87d30751da22ee9d1bd4be70d85efdc3ac9330f728096fdeac2cfb6cb7efe60dcf4c492c7057791428005adf7ab8975b2182dc8e4b4866ad7c5717aa352114c57533fd21df04b829d2903e83131dee40fbe8e70a2bccc54de34f289724ca70421fb7671135fa0e13c6b2e450e8ce4d37";

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
