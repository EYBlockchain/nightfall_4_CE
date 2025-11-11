// SPDX-License-Identifier: CC0-1.0
pragma solidity ^0.8.4;

// Library for RSA/PKCS#1 v1.5 signature verification used by X509
contract X509SigLib {
    // DigestInfo prefixes for SHA-256 and SHA-512, same as in your contract
    bytes constant DI_PREFIX_SHA256 = hex"3031300d060960864801650304020105000420";
    bytes constant DI_PREFIX_SHA512 = hex"3051300d060960864801650304020305000440";
    

    // ----- internal helpers -----

    // modular exponentiation via precompile at address 0x05
    function modExp(
        bytes memory b,
        uint256 e,
        bytes memory m
    ) private view returns (bytes memory) {
        bool success;
        bytes memory result;
        (success, result) = address(5).staticcall(
            abi.encodePacked(b.length, uint256(32), m.length, b, e, m)
        );
        require(success);
        return result;
    }

    function _stripLeadingZero(bytes memory x) private pure returns (bytes memory) {
        if (x.length > 0 && x[0] == 0x00) {
            bytes memory out = new bytes(x.length - 1);
            for (uint256 i = 0; i < out.length; i++) {
                out[i] = x[i + 1];
            }
            return out;
        }
        return x;
    }

    // a and b are big-endian, same length
    function _lessThan(bytes memory a, bytes memory b) private pure returns (bool) {
        require(a.length == b.length);
        for (uint256 i = 0; i < a.length; i++) {
            if (a[i] < b[i]) return true;
            if (a[i] > b[i]) return false;
        }
        return false; // equal
    }

    function _hasPrefix(bytes memory data, bytes memory prefix)
        private pure
        returns (bool)
    {
        if (data.length < prefix.length) return false;
        for (uint256 i = 0; i < prefix.length; i++) {
            if (data[i] != prefix[i]) return false;
        }
        return true;
    }

    function _slice(
        bytes memory data,
        uint256 offset,
        uint256 len
    ) private pure returns (bytes memory) {
        require(data.length >= offset + len);
        bytes memory out = new bytes(len);
        for (uint256 i = 0; i < len; i++) {
            out[i] = data[offset + i];
        }
        return out;
    }

    /*
     * Parse PKCS#1 v1.5 encoded block and extract the hash from DigestInfo.
     * This is your original validateSignatureAndExtractMessageHash.
     */
    function _validateSignatureAndExtractMessageHash(
        bytes memory decrypt
    ) private pure returns (bytes memory) {
        uint256 k = decrypt.length;

        // Minimal: 0x00 0x01 + 8*0xFF + 0x00 + DigestInfo(...)
        require(k >= 11);

        // EM = 0x00 || 0x01 || PS || 0x00 || T
        require(decrypt[0] == 0x00);
        require(
            decrypt[1] == 0x01
        );

        // padding string of 0xFF bytes, at least 8 of them, starting at decrypt[2]
        uint256 i = 2;
        while (i < k && decrypt[i] == 0xff) {
            i++;
        }

        uint256 psLen = i - 2;
        require(psLen >= 8);

        // Single 0x00 separator
        require(i < k);
        require(decrypt[i] == 0x00);

        // T (DigestInfo) starts after the separator
        uint256 tOffset = i + 1;
        require(tOffset < k);
        uint256 tLen = k - tOffset;

        bytes memory t = _slice(decrypt, tOffset, tLen);

        // DigestInfo = SEQUENCE { AlgorithmIdentifier, OCTET STRING <hash> }
        // Match against SHA-256 or SHA-512 constants and extract hash
        bytes memory hash;
        if (
            tLen == DI_PREFIX_SHA256.length + 32 &&
            _hasPrefix(t, DI_PREFIX_SHA256)
        ) {
            // SHA-256: last 32 bytes are the hash
            hash = _slice(t, DI_PREFIX_SHA256.length, 32);
        } else if (
            tLen == DI_PREFIX_SHA512.length + 64 &&
            _hasPrefix(t, DI_PREFIX_SHA512)
        ) {
            // SHA-512: last 64 bytes are the hash
            hash = _slice(t, DI_PREFIX_SHA512.length, 64);
        } else {
            revert("X509: Unsupported or invalid DigestInfo");
        }

        return hash;
    }

    /**
     * High-level helper: given an RSA signature S, modulus n, exponent e,
     * perform S^e mod n, check PKCS#1 v1.5 structure and return the embedded hash.
     *
     * NOTE: This does *not* compare against the message hash. The caller
     * (X509 contract) will do the sha256/sha512 comparison.
     */
    function extractDigestFromSignature(
        bytes memory signature,
        bytes memory modulusRaw,
        uint256 exponent
    ) internal view returns (bytes memory) {
        // 1. Normalize modulus (drop optional sign pad 0x00)
        bytes memory modulus = _stripLeadingZero(modulusRaw);

        // 2. Length equality: |S| = k
        uint256 k = modulus.length;
        require(signature.length == k);

        // 3. Range check: 0 <= s < n
        require(
            _lessThan(signature, modulus)
        );

        // 4. Compute S^e mod n
        bytes memory signatureDecrypt = modExp(
            signature,
            exponent,
            modulus
        );

        // 5. The EM block must have exactly one leading 0x00 octet and length k
        require(
            signatureDecrypt.length == k
        );
        require(
            signatureDecrypt[0] == 0x00
        );

        // 6. Parse PKCS#1 v1.5 structure and extract hash from DigestInfo
        bytes memory messageHashFromSignature =
            _validateSignatureAndExtractMessageHash(signatureDecrypt);

        return messageHashFromSignature;
    }
}
