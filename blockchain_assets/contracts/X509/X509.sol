// SPDX-License-Identifier: CC0-1.0

pragma solidity ^0.8.4;

// This contract can parse a suitably encoded SSL certificate

// OZ upgradeable base
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

import "./DerParser.sol";
import "./Allowlist.sol";
import "./X509Interface.sol";
import "./X509SigLib.sol";
import "./Sha.sol";
import "forge-std/console.sol";
/**
 * @title X509 (upgradeable)
 * @notice Upgrade-safe version of X509 validator. Constructor removed; use initialize().
 *         Storage layout preserved; future fields should be appended above the __gap.
 */
contract X509 is
    Initializable,
    UUPSUpgradeable,
    DERParser,
    Allowlist,
    Sha,
    X509SigLib,
    X509Interface
{
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    uint256 constant SECONDS_PER_DAY = 24 * 60 * 60;
    int256 constant OFFSET19700101 = 2440588;

    struct RSAPublicKey {
        bytes modulus;
        uint256 exponent;
    }

    struct CertificateArgs {
        bytes certificate;
        uint256 tlvLength;
        bytes addressSignature;
        bool isEndUser;
        bool checkOnly;
        uint256 oidGroup;
        address addr;
    }

    // ========= Storage =========
    mapping(address => uint256) private expires;
    mapping(bytes32 => RSAPublicKey) private trustedPublicKeys;
    mapping(bytes32 => bool) private revokedKeys;
    mapping(address => bytes32) private keysByUser;
    // Reverse mapping to ensure one certificate is tied to one address
    mapping(bytes32 => address) private addressByKey;
    mapping(bytes32 => uint256) public oidGroupByAuthorityKeyIdentifier;

    // OID groups (per-CA)
    bytes32[][] private extendedKeyUsageOIDs; // array-of-arrays: each CA has its own set of OIDs
    bytes32[][] private certificatePoliciesOIDs;

    // Key usage bitmasks
    bytes1 private usageBitMaskEndUser;
    bytes1 private usageBitMaskIntermediate;

    // ========= Initializer =========
    function initialize(address owner_) external initializer {
        __UUPSUpgradeable_init();
        __Allowlist_init(owner_); // sets owner + allowlisting = true

        // default masks
        usageBitMaskEndUser = 0x80;
        usageBitMaskIntermediate = 0x06;
    }

    // ========= Owner / config =========
    function setUsageBitMaskEndUser(bytes1 _usageBitMask) external onlyOwner {
        usageBitMaskEndUser = _usageBitMask;
    }

    function setUsageBitMaskIntermediate(
        bytes1 _usageBitMask
    ) external onlyOwner {
        usageBitMaskIntermediate = _usageBitMask;
    }

    function addExtendedKeyUsage(bytes32[] calldata oids) external onlyOwner {
        extendedKeyUsageOIDs.push(oids);
    }

    function addCertificatePolicies(
        bytes32[] calldata oids
    ) external onlyOwner {
        certificatePoliciesOIDs.push(oids);
    }

    function setTrustedCA(
        uint256 _authorityKeyIdentifier,
        uint256 oidGroup
    ) external onlyOwner {
        bytes32 authorityKeyIdentifier = bytes32(_authorityKeyIdentifier);
        oidGroupByAuthorityKeyIdentifier[authorityKeyIdentifier] = oidGroup;
    }

    // NB this function removes everything. You need to re-add all OIDs if you call this
    // but removing everything avoids a sparse array.
    function removeExtendedKeyUsage() external onlyOwner {
        delete extendedKeyUsageOIDs;
    }

    function removeCertificatePolicies() external onlyOwner {
        delete certificatePoliciesOIDs;
    }

    function setTrustedPublicKey(
        RSAPublicKey calldata trustedPublicKey,
        uint256 _authorityKeyIdentifier
    ) external onlyOwner {
        bytes32 authorityKeyIdentifier = bytes32(_authorityKeyIdentifier);
        trustedPublicKeys[authorityKeyIdentifier] = trustedPublicKey;
    }

    // ========= Internal helpers =========
    function getSignature(
        DecodedTlv[] memory tlvs,
        uint256 maxId
    ) private pure returns (bytes memory) {
        DecodedTlv memory signatureTlv = tlvs[maxId - 1];
        require(
            signatureTlv.depth == 1,
            "X509: Signature tlv depth is incorrect"
        );
        require(
            signatureTlv.tag.tagType == 0x03,
            "X509: Signature tlv should have a tag type of BIT STRING"
        );
        bytes memory bitString = signatureTlv.value;
        require(bitString.length > 1, "X509: Signature BIT STRING too short");

        // First content octet is "unused bits" and must be 0x00 for X.509
        require(bitString[0] == 0x00, "X509: Signature unused bits must be 0");

        // Drop the unused-bits octet
        bytes memory signature = new bytes(bitString.length - 1);
        for (uint256 i = 0; i < signature.length; i++) {
            signature[i] = bitString[i + 1];
        }

        return signature;
    }

    function getMessage(
        DecodedTlv[] memory tlvs
    ) private pure returns (bytes memory) {
        DecodedTlv memory messageTlv = tlvs[1];
        require(messageTlv.depth == 1, "X509: Message tlv depth is incorrect");
        require(
            messageTlv.tag.tagType == 0x10,
            "X509: Message tlv should have a tag type of SEQUENCE"
        );
        bytes memory message = messageTlv.octets;
        return message;
    }

    // note: this function is from an MIT licensed library, with appreciation to
    // https://github.com/bokkypoobah/BokkyPooBahsDateTimeLibrary
    function timestampFromDate(
        bytes memory utcTime
    ) private pure returns (uint256 _seconds) {
        uint256 year = uint256(uint8(utcTime[0]) - 48) * 10 +
            uint256(uint8(utcTime[1]) - 48) +
            2000;
        uint256 month = uint256(uint8(utcTime[2]) - 48) * 10 +
            uint256(uint8(utcTime[3]) - 48);
        uint256 day = uint256(uint8(utcTime[4]) - 48) * 10 +
            uint256(uint8(utcTime[5]) - 48);
        require(year >= 1970, "X509: year < 1970");
        int256 _year = int256(year);
        int256 _month = int256(month);
        int256 _day = int256(day);

        int256 __days = _day -
            32075 +
            (1461 * (_year + 4800 + (_month - 14) / 12)) / 4 +
            (367 * (_month - 2 - ((_month - 14) / 12) * 12)) / 12 -
            (3 * ((_year + 4900 + (_month - 14) / 12) / 100)) / 4 -
            OFFSET19700101;

        _seconds = uint256(__days) * SECONDS_PER_DAY;
    }

    // this function finds and checks the Not Before and Not After tlvs
    function checkDates(
        DecodedTlv[] memory tlvs
    ) private view returns (uint256) {
        // The Not Before and Not After dates are the third SEQUENCE at depth 2
        uint256 i;
        uint256 j;
        for (i = 0; i < tlvs.length; i++) {
            if (tlvs[i].tag.tagType == 0x10 && tlvs[i].depth == 2) j++;
            if (j == 3) break;
        }
        require(
            tlvs[i + 1].tag.tagType == 0x17,
            "X509: First tag was not in fact a UTC time"
        );
        require(
            tlvs[i + 2].tag.tagType == 0x17,
            "X509: Second tag was not in fact a UTC time"
        );
        require(
            block.timestamp > timestampFromDate(tlvs[i + 1].value),
            "X509: It is too early to use this certificate"
        );
        uint256 expiry = timestampFromDate(tlvs[i + 2].value);
        require(block.timestamp < expiry, "X509: This certificate has expired");
        return expiry;
    }

    function extractPublicKey(
        DecodedTlv[] memory tlvs
    ) private view returns (RSAPublicKey memory) {
        // The public key data begins at the 5th SEQUENCE at depth 2
        uint256 i;
        uint256 j;
        for (i = 0; i < tlvs.length; i++) {
            if (tlvs[i].tag.tagType == 0x10 && tlvs[i].depth == 2) j++;
            if (j == 5) break;
        }
        // check we have RSA encryption. We use the keccak hash to check equality of the byte arrays
        require(
            keccak256(tlvs[i + 2].value) ==
                keccak256(abi.encodePacked(bytes9(0x2a864886f70d010101))),
            "X509: Only RSA encryption keys are supported, OID indicates different key type"
        );
        bytes memory keyBytes = tlvs[i + 4].value;
        // extract the public key tlvs
        DecodedTlv[] memory keyTlvs = new DecodedTlv[](10);
        keyTlvs = this.parseDER(keyBytes, 1, 10);
        bytes memory modulus = keyTlvs[1].value;
        uint256 exponent = uint256(
            bytes32(keyTlvs[2].value) >> ((32 - keyTlvs[2].value.length) * 8)
        );

        return RSAPublicKey(modulus, exponent);
    }

    function extractSubjectKeyIdentifier(
        DecodedTlv[] memory tlvs
    ) private view returns (bytes32) {
        // The SKID begins after the Subject Key Identifier OID at depth 5
        uint256 i;
        for (i = 0; i < tlvs.length; i++) {
            if (tlvs[i].depth != 5) continue;
            if (
                bytes32(tlvs[i].value) ==
                bytes32(
                    0x551d0e0000000000000000000000000000000000000000000000000000000000
                )
            ) break; // OID for the SKID
        }
        require(i < tlvs.length, "X509: SKID OID not found");
        bytes memory skidBytes = tlvs[i + 1].value;
        require(skidBytes.length < 33, "X509: SKID too long");
        DecodedTlv[] memory skidTlvs = new DecodedTlv[](1);
        skidTlvs = this.parseDER(skidBytes, 0, 2);
        bytes32 skid = bytes32(skidTlvs[0].value) >>
            ((32 - skidTlvs[0].length) * 8);
        return skid;
    }

    function extractAuthorityKeyIdentifier(
        DecodedTlv[] memory tlvs
    ) private view returns (bytes32) {
        // The AKID begins after the Authority Key Identifier OID at depth 5
        uint256 i;
        for (i = 0; i < tlvs.length; i++) {
            if (tlvs[i].depth != 5) continue;
            if (
                bytes32(tlvs[i].value) ==
                bytes32(
                    0x551d230000000000000000000000000000000000000000000000000000000000
                )
            ) break; // OID for the AKID
        }
        require(i < tlvs.length, "X509: AKID OID not found");
        bytes memory akidBytes = tlvs[i + 1].value;
        require(akidBytes.length < 33, "X509: AKID too long");
        DecodedTlv[] memory akidTlvs = new DecodedTlv[](3);
        akidTlvs = this.parseDER(akidBytes, 0, 2);
        bytes32 akid = bytes32(akidTlvs[1].value) >>
            ((32 - akidTlvs[1].value.length) * 8);
        return akid;
    }

    function checkKeyUsage(
        DecodedTlv[] memory tlvs,
        bytes1 _usageBitMask
    ) private view {
        // The key usage sequence begins after the Key Usage OID at depth 5
        uint256 i;
        for (i = 0; i < tlvs.length; i++) {
            if (tlvs[i].depth != 5) continue;
            if (
                bytes32(tlvs[i].value) ==
                bytes32(
                    0x551d0f0000000000000000000000000000000000000000000000000000000000
                )
            ) break; // OID for keyUsage
        }
        require(i < tlvs.length, "X509: OID for Key Usage not found");
        bytes memory usageBytes = tlvs[i + 1].value;
        // usageBytes could be an octet string containing a bit string that needs decoding,
        // or a boolean (0x01) indicating criticality (ignored).
        if (tlvs[i + 1].octets[0] == 0x01) usageBytes = tlvs[i + 2].value;
        DecodedTlv[] memory usageTlvs = new DecodedTlv[](1);
        usageTlvs = this.parseDER(usageBytes, 0, 1);
        require(
            usageTlvs[0].length == 2,
            "X509: Key usage bytes must be length 2"
        );
        // First byte tells how many bits to ignore in the second byte
        bytes1 usageFlags = (usageTlvs[0].value[1] >>
            uint8(usageTlvs[0].value[0])) << uint8(usageTlvs[0].value[0]);
        // little-endian mask
        require(
            (usageFlags & _usageBitMask) == _usageBitMask,
            "X509: Key usage is not as required"
        );
    }

    function checkExtendedKeyUsage(
        DecodedTlv[] memory tlvs,
        uint256 oidGroup
    ) private view {
        // The extended key usage sequence begins after the Extended Key Usage OID at depth 5
        uint256 i;
        for (i = 0; i < tlvs.length; i++) {
            if (tlvs[i].depth != 5) continue;
            if (
                bytes32(tlvs[i].value) ==
                bytes32(
                    0x551d250000000000000000000000000000000000000000000000000000000000
                )
            ) break; // OID for extendedKeyUsage
        }
        require(i < tlvs.length, "X509: OID for Extended Key Usage not found");
        bytes memory extendedUsageBytes = tlvs[i + 1].value;
        if (tlvs[i + 1].octets[0] == 0x01)
            extendedUsageBytes = tlvs[i + 2].value; // boolean criticality
        uint256 tlvLength = this.computeNumberOfTlvs(extendedUsageBytes, 0);
        DecodedTlv[] memory extendedUsageTlvs = new DecodedTlv[](tlvLength);
        extendedUsageTlvs = this.parseDER(extendedUsageBytes, 0, tlvLength);
        // Now we need to loop through the extendedKeyUsageOIDs, and check we have every one of them in the cert
        for (uint256 j = 0; j < extendedKeyUsageOIDs[oidGroup].length; j++) {
            bool oidFound = false;
            for (uint256 k = 0; k < tlvLength; k++) {
                if (
                    bytes32(extendedUsageTlvs[k].octets) ==
                    extendedKeyUsageOIDs[oidGroup][j]
                ) {
                    oidFound = true;
                    break;
                }
            }
            require(
                oidFound,
                "X509: Required Extended Key Usage OID not found"
            );
        }
    }

    function checkCertificatePolicies(
        DecodedTlv[] memory tlvs,
        uint256 oidGroup
    ) private view {
        // The certificate policies sequence begins after the Certificate Policies OID at depth 5
        uint256 i;
        for (i = 0; i < tlvs.length; i++) {
            if (tlvs[i].depth != 5) continue;
            if (
                bytes32(tlvs[i].value) ==
                bytes32(
                    0x551d200000000000000000000000000000000000000000000000000000000000
                )
            ) break; // OID for certificate policies
        }
        require(
            i < tlvs.length,
            "X509: OID for Certificate Policies not found"
        );
        bytes memory extendedUsageBytes = tlvs[i + 1].value;
        if (tlvs[i + 1].octets[0] == 0x01)
            extendedUsageBytes = tlvs[i + 2].value; // boolean criticality
        uint256 tlvLength = this.computeNumberOfTlvs(extendedUsageBytes, 0);
        DecodedTlv[] memory extendedUsageTlvs = new DecodedTlv[](tlvLength);
        extendedUsageTlvs = this.parseDER(extendedUsageBytes, 0, tlvLength);
        // certificate policies are, unfortunately not a simple oid but an octet string containing a sequence of sequences. The oids we want are in each sequence.
        // Thus extendedUsageTlvs is an array of sequences. We have to loop through it, collecting the first OID inside each.  We can ignore
        // the rest of the sequence which will be yet another sequence of policy qualifiers.  We don't care about those for this purpose.
        // We just need to ensure the policy exists.
        bytes32[] memory policyOIDs = new bytes32[](extendedUsageTlvs.length); // upper bound
        uint256 count = 0;
        for (uint256 j = 0; j < extendedUsageTlvs.length; j++) {
            if (extendedUsageTlvs[j].depth == 2)
                policyOIDs[count++] = bytes32(extendedUsageTlvs[j].octets);
        }
        // Now we have an array containing the policy OIDs we need to loop through
        // the certificate policie OIDs, and check we have every one of them in the cert
        for (uint256 j = 0; j < certificatePoliciesOIDs[oidGroup].length; j++) {
            bool oidFound = false;
            for (uint256 k = 0; k < count; k++) {
                if (policyOIDs[k] == certificatePoliciesOIDs[oidGroup][j]) {
                    oidFound = true;
                    break;
                }
            }
            require(
                oidFound,
                "X509: Required Certificate Policy OID not found"
            );
        }
    }

    // ========= RSA signature verification wrappers =========

    // PKCS#1 v1.5 for certificate signatures (unchanged behaviour)
    function checkCertificateSignature(
        bytes memory signature,
        bytes memory message,
        RSAPublicKey memory publicKey
    ) private view {
        // Use library to extract digest from RSA signature (PKCS#1 v1.5)
        bytes memory sigHash = extractDigestFromSignature(
            signature,
            publicKey.modulus,
            publicKey.exponent
        );

        // Compare digest against sha256/sha512(message)
        require(
            keccak256(sigHash) == keccak256(abi.encode(sha256(message))) ||
                keccak256(sigHash) ==
                keccak256(this.sha512(message)),
            "X509: Certificate signature is invalid"
        );
    }

    // RSASSA-PSS (SHA-256, MGF1-SHA256, saltLen=32) for address binding
    function checkAddressSignaturePSS(
        bytes memory signature,
        bytes memory message,
        RSAPublicKey memory publicKey
    ) private view {
        console.log("Checking address signature PSS modulus: ");
        console.logBytes(publicKey.modulus);
        console.log("Checking address signature PSS exponent: ", publicKey.exponent);
        bool ok = rsaPssVerifySha256(
            signature,
            message,
            publicKey.modulus,
            publicKey.exponent,
            32 // saltLen = SHA-256 digest length
        );
        require(ok, "X509: Address signature is invalid");
    }

    // ========= External API =========
    /**
    This function is the main one in the module. It calls all of the subsidiary functions necessary to validate an RSA cert
    If the validation is successful (and it's an endUserCert), it will add the sender to the allowlist contract, provided they
    are able to sign their ethereum address with the private key corresponding to the certificate.
     */
    function validateCertificate(CertificateArgs calldata args) external {
        bytes calldata certificate = args.certificate;
        uint256 tlvLength = args.tlvLength;
        bytes calldata addressSignature = args.addressSignature;
        bool isEndUser = args.isEndUser;
        bool checkOnly = args.checkOnly;
        uint256 oidGroup = args.oidGroup;
        address addr = args.addr;

        // If addr == 0, allowlist msg.sender; otherwise ensure it matches
        if (addr == address(0)) {
            addr = msg.sender;
        } else {
            require(
                addr == msg.sender,
                "X509: You can only allowlist your own address"
            );
        }

        // Decode the DER-encoded certificate into TLVs
        DecodedTlv[] memory tlvs = new DecodedTlv[](tlvLength);
        tlvs = walkDerTree(certificate, 0, tlvLength);

        // extract the data from the certificate necessary for checking the signature and (hopefully) find the Authority public key in
        // the smart contract's list of trusted keys
        bytes32 authorityKeyIdentifier = extractAuthorityKeyIdentifier(tlvs);
        uint256 expectedOidGroup =
            oidGroupByAuthorityKeyIdentifier[authorityKeyIdentifier];
        require(
            expectedOidGroup == oidGroup,
            "X509: OID group does not match allowed EKUs and Certificate Policies"
        );

        bytes memory signature = getSignature(tlvs, tlvLength);
        bytes memory message = getMessage(tlvs);
        RSAPublicKey memory publicKey = trustedPublicKeys[
            authorityKeyIdentifier
        ];
        // validate the cert's signature and check that the cert is in date, and not revoked nor signed by a revoked cert,
        // checkCertificateSignature(signature, message, publicKey);

        uint256 expiry = checkDates(tlvs);
        RSAPublicKey memory certificatePublicKey = extractPublicKey(tlvs);
        bytes32 subjectKeyIdentifier = extractSubjectKeyIdentifier(tlvs);

        require(
            !revokedKeys[subjectKeyIdentifier],
            "X509: The subject key of this certificate has been revoked"
        );
        require(
            !revokedKeys[authorityKeyIdentifier],
            "X509: The authority key of this certificate has been revoked"
        );

        // If not an end-user cert, treat as intermediate CA
        if (!isEndUser) {
            // Check this certificate can sign certs
            checkKeyUsage(tlvs, usageBitMaskIntermediate);
            // If OK and not check-only, trust this intermediate
            if (!checkOnly) {
                trustedPublicKeys[subjectKeyIdentifier] = certificatePublicKey;
            }
            return;
        }

        // For end-user certs, check key usage, EKUs, and policies
        checkKeyUsage(tlvs, usageBitMaskEndUser);
        checkExtendedKeyUsage(tlvs, oidGroup);
        checkCertificatePolicies(tlvs, oidGroup);

        if (!checkOnly) {
            // Ensure one certificate is tied to one address and vice versa
            require(
                keysByUser[addr] == bytes32(0) ||
                    keysByUser[addr] == subjectKeyIdentifier,
                "X509: This address is already linked to a different certificate"
            );
            require(
                addressByKey[subjectKeyIdentifier] == address(0) ||
                    addressByKey[subjectKeyIdentifier] == addr,
                "X509: This certificate is already linked to a different address"
            );
            // Before we finally add the address to the allowlist, just check that the sender of the allowlist request actually owns the
            // end user cert.  We do this by getting them to sign the Ethereum address they want allowlisted (RSASSA-PSS).
            checkAddressSignaturePSS(
                addressSignature,
                abi.encodePacked(uint160(addr)),
                certificatePublicKey
            );
            expires[addr] = expiry;
            keysByUser[addr] = subjectKeyIdentifier;
            // RECORD reverse mapping for one-to-one binding
            addressByKey[subjectKeyIdentifier] = addr;

            addUserToAllowlist(addr); // all checks passed
        }
    }

    // performs an ongoing X509 check (is the user still in the allowlist?
    // Has the public key been revoked? Is the cert in date?)
    function x509Check(address user) external view returns (bool) {
        if (
            !allowlisting ||
            (!revokedKeys[keysByUser[user]] &&
                expires[user] > block.timestamp &&
                isAllowlisted(user))
        ) return true;
        return false;
    }

    /** 
    This function allows a certificate to be revoked from a allowlisted address (or by the contract owner). This cannot be undone!
    It is useful if the private key is compromised.  The owner of the allowlisted address can revoke the certificate.
    Once this is done, they will lose their allowlisted status.
    @param _subjectKeyIdentifier - the subject key identifier for the certificate that is to be revoked.
    */
    function revokeKeyFromUserAddress(uint256 _subjectKeyIdentifier) external {
        bytes32 subjectKeyIdentifier = bytes32(_subjectKeyIdentifier);
        require(
            keysByUser[msg.sender] == subjectKeyIdentifier ||
                msg.sender == owner,
            "X509: You are not the owner of this key"
        );
        revokedKeys[subjectKeyIdentifier] = true;
        delete trustedPublicKeys[subjectKeyIdentifier];

        // CLEANUP: remove bidirectional binding when revoked
        address addr = addressByKey[subjectKeyIdentifier];
        delete keysByUser[addr];
        delete addressByKey[subjectKeyIdentifier];
    }

    // ========= UUPS authorization =========
    function _authorizeUpgrade(address) internal override onlyOwner {}

    // ========= Storage gap =========
    uint256[50] private __gap;
}
