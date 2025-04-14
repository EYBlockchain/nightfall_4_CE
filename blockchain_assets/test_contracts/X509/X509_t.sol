// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.19;

import "forge-std/Test.sol";
import "../../contracts/X509/X509.sol";
contract X509Test is Test {
    X509 x509;
    DERParser derParser;

    function setUp() public {
        x509 = new X509(address(this));
        derParser = new DERParser();
    }

    function test_X509_valid_cert() public {
        uint256 testTimestamp = 1734088001;
        vm.warp(testTimestamp);

        // openssl x509 -in root_ca.crt -text -noout | grep -A 1 "Authority Key Identifier"
        bytes memory authorityKeyIdentifierBytes = hex"CA916116F4F9D80A03D32ED516CF6496408F0A4A";
        uint256 authorityKeyIdentifier = bytesToUint256(authorityKeyIdentifierBytes);
        // openssl rsa -in root_ca_priv_key.pem -modulus -noout
        // get modulus from private key of root_ca, don't forget to add "00" at the beginning
        bytes memory modulus = hex"00BD1824FB96B4764E6115508B73632F8F65884A5BFC391A55C56469D9DB57FF5FCAF15E6DCBECCE2C02E32F7092EEB35727354C46707417CE566796295B1B57604211962C7F96459AA046385972F5A8EF46269B82C987FAF89DF76FDC985031BEE4D110753F028CD91A48D5AC4E4DA76B9AE48A6875AA791E96B3ECE3BF1DC2390866D8B0665FDCD647A2F37FBD3087AEA620F9E69E38EE42EC57530FCF42F64E8E6AD20279E9D74062D0B3B4B997A92471B72BC922AE3CAE6A366C9ACD7BBC93AC8E6DD3FFEA77F03E0941439C2A717C7C45E681EB2ABC30DF7E1EC06B84BA1C5CCB9E40D7E0F96EE6198D1349A085598C7FA3D190434102FD668FAB6B8DDFDA9B15CAD00C7F54BED718FB344323CEB91E91EAF1643FD647803772B80BE4A0EF58350AE128E1E36069F561D1578EBCD867EB6E61252843D860BD5DA195C8ED4D946C1C06AD43DDA1B78F4A4CD73AAA24CB9CE55C0CB968E4475D9E04DBE9A7DB2A7215CF930EA390218E6CDFE719B724F0C1EBC78895223BDB4A2AB595F604D2ABEC4F8180E66512259BDC989DEE4CE3E9F595304BCA6169DC6BE38903071D5D8F56ED78EE8904B881623F884F6FA2CBF262983AABEDDA03255D7CF4CA9C28946FEF77EEF7ACBFF326AD6452EB7B3758DFDBF1B57DD800BDA651F0EB84F1D3C9DB2A693BC7B467E9111D4CC645F2009B26A47F1D803651DC56AF7123C61A5337";

        // openssl x509 -in root_ca.crt -text -noout | grep -A 1 "Exponent"
        // find Exponent: 65537 (0x10001)
        uint256 exponent = 65537;
        X509.RSAPublicKey memory nightfallRootPublicKey = X509.RSAPublicKey(modulus, exponent);

        x509.setTrustedPublicKey(nightfallRootPublicKey, authorityKeyIdentifier);
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
        certificatePoliciesOIDs_0[0] = hex"06032d0607000000000000000000000000000000000000000000000000000000";
        x509.addCertificatePolicies(certificatePoliciesOIDs_0);
    }

    function validateCertificates() internal {
        bytes memory intermediate_ca_derBuffer = vm.readFileBinary("./blockchain_assets/test_contracts/X509/_certificates/intermediate_ca.der");
        uint256 intermediate_ca_tlvLength = x509.computeNumberOfTlvs(intermediate_ca_derBuffer, 0);

        // Intermediate CA certificate validation
        X509.CertificateArgs memory intermediate_certificate_args = X509.CertificateArgs({
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
        bytes memory signature = hex"509a66baca2be2c4988b817b6637364ea2b038149498d548b1b5090e7c46dec69a1ee471ca445e3e89923d9bcade53a39312e6ccbfe82f40b80e02753991fa0ebd0fe7378a49d458eec05db0234a9073bfba6b2cb03beff49eebafe7b0e6374de5abeb8238ca4c4e20fca4d9d0396f6bce2b50c5ee65de65a64e1ba2d640a0d70d7401025b8e606b86627779f89230884ad26723c7a81817944ff3270976441c2b8beb8fe42a0fbfc8a05e5546a76999e17e664b49c406d08d0a00f49c810e71b208ebf9199aab4b8a2600133457fa9a667ccb99c2ce4a8dcfc488964774eb1ea754572abbfde6542a7a532846af120b246e4743800d01157b10a8df0c8d2c4a7e964105a26272674bd2636c45ae13d7e9061939c42edfe6b20857fa1a81e08ff0b9eb958c837df63eefba802e795fcb2d72c3fb03e7c766807300e7e06aad77478bfd4225b1158a51cd4fc3bb18cd0af1d3d279de5a248a92eb8bc612e46177b358480b587162ccf2a4d1ce843dffa46490ddb533e0eba24621704da6d64cad5b4d6e94d9340d51c885e2d7ec3d113e6d48c43e194363cc06d3883f6f887c5880dd2afa53bd38c915baf7bb6dee3a052da9d5a846a0da41d1c030b4a974c50a55c3f172b06f6c7155ac0444e46dd13966e22a0d5c92fbee53f4e3cbbaeede926bde4bde8a130d1f7e263ec37420f98466a02baa91605c064de5f93e109a7c65";

        bytes memory endUserCert_derBuffer = vm.readFileBinary("./blockchain_assets/test_contracts/X509/_certificates/user/user-1.der");
        uint256 endUserCert_tlvLength = x509.computeNumberOfTlvs(endUserCert_derBuffer, 0);

        X509.CertificateArgs memory endUser_certificate_args = X509.CertificateArgs({
            certificate: endUserCert_derBuffer,
            tlvLength: endUserCert_tlvLength,
            addressSignature: signature,
            isEndUser: true,
            checkOnly: false,
            oidGroup: 0,
            addr: msg.sender
        });
        x509.validateCertificate(endUser_certificate_args);
    }

    function bytesToUint256(bytes memory b) internal pure returns (uint256) {
        uint256 number;
        for (uint i = 0; i < b.length; i++) {
            number = number + uint256(uint8(b[i])) * (2**(8 * (b.length - (i + 1))));
        }
        return number;
    }
}
