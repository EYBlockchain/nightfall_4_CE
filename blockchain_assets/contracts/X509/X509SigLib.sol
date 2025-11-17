// SPDX-License-Identifier: CC0-1.0
pragma solidity ^0.8.4;

// Library for RSA/PKCS#1 v1.5 + RSA-PSS signature verification used by X509
contract X509SigLib {
    // DigestInfo prefixes for SHA-256 and SHA-512
    bytes constant DI_PREFIX_SHA256 = hex"3031300d060960864801650304020105000420";
    bytes constant DI_PREFIX_SHA512 = hex"3051300d060960864801650304020305000440";

    // ----- internal helpers for PKCS#1 v1.5 -----

    // modular exponentiation via precompile at address 0x05
    // NOTE: visibility changed to internal so X509 can reuse it.
    function modExp(
        bytes memory b,
        uint256 e,
        bytes memory m
    ) internal view returns (bytes memory) {
        bool success;
        bytes memory result;
        // | baseLen | expLen | modLen | base | exp | mod |
        (success, result) = address(5).staticcall(
            abi.encodePacked(b.length, uint256(32), m.length, b, e, m)
        );
        require(success, "X509: modexp failed");
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
        require(a.length == b.length, "X509: length mismatch");
        for (uint256 i = 0; i < a.length; i++) {
            if (a[i] < b[i]) return true;
            if (a[i] > b[i]) return false;
        }
        return false; // equal
    }

    function _hasPrefix(bytes memory data, bytes memory prefix)
        private
        pure
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
        require(data.length >= offset + len, "X509: slice out of range");
        bytes memory out = new bytes(len);
        for (uint256 i = 0; i < len; i++) {
            out[i] = data[offset + i];
        }
        return out;
    }

    /*
     * Parse PKCS#1 v1.5 encoded block and extract the hash from DigestInfo.
     */
    function _validateSignatureAndExtractMessageHash(
        bytes memory decrypt
    ) private pure returns (bytes memory) {
        uint256 k = decrypt.length;

        // Minimal: 0x00 0x01 + 8*0xFF + 0x00 + DigestInfo(...)
        require(k >= 11, "X509: EM too short");

        // EM = 0x00 || 0x01 || PS || 0x00 || T
        require(decrypt[0] == 0x00, "X509: bad leading 0x00");
        require(decrypt[1] == 0x01, "X509: bad block type");

        // padding string of 0xFF bytes, at least 8 of them, starting at decrypt[2]
        uint256 i = 2;
        while (i < k && decrypt[i] == 0xff) {
            i++;
        }

        uint256 psLen = i - 2;
        require(psLen >= 8, "X509: PS too short");

        // Single 0x00 separator
        require(i < k, "X509: no separator");
        require(decrypt[i] == 0x00, "X509: bad separator");

        // T (DigestInfo) starts after the separator
        uint256 tOffset = i + 1;
        require(tOffset < k, "X509: no DigestInfo");
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
        require(signature.length == k, "X509: sig/mod length mismatch");

        // 3. Range check: 0 <= s < n
        // require(_lessThan(signature, modulus), "X509: signature out of range");

        // 4. Compute S^e mod n
        bytes memory signatureDecrypt = modExp(signature, exponent, modulus);

        // 5. The EM block must have exactly one leading 0x00 octet and length k
        require(signatureDecrypt.length == k, "X509: EM length mismatch");
        require(signatureDecrypt[0] == 0x00, "X509: EM leading byte != 0x00");

        // 6. Parse PKCS#1 v1.5 structure and extract hash from DigestInfo
        bytes memory messageHashFromSignature =
            _validateSignatureAndExtractMessageHash(signatureDecrypt);

        return messageHashFromSignature;
    }

    // ========= RSA-PSS helpers (for address binding) =========

     /// @dev Perform EM = signature^exponent mod modulusRaw using modExp.
    ///      Handles a possible leading 0x00 in modulusRaw and enforces basic range checks.
    function rsaRecoverEncodedMessage(
        bytes memory signature,
        bytes memory modulusRaw,
        uint256 exponent
    ) internal view returns (bytes memory) {
        // Normalize modulus (drop optional sign pad)
        bytes memory modulus = _stripLeadingZero(modulusRaw);
        uint256 k = modulus.length;
        require(k > 0, "X509: empty modulus");

        // Require |S| = k
        require(signature.length == k, "X509: sig/mod size mismatch");

        // Range check: 0 <= s < n
        require(_lessThan(signature, modulus), "X509: signature out of range");

        // EM = S^e mod n
        bytes memory em = modExp(signature, exponent, modulus);
        require(em.length == k, "X509: bad EM length");
        return em;
    }

    /// @dev Verify RSA-PSS with SHA-256, MGF1-SHA256 and fixed salt length.
    ///      Returns true if the signature is valid.
    function rsaPssVerifySha256(
        bytes memory signature,
        bytes memory message,
        bytes memory modulusRaw,
        uint256 exponent,
        uint256 saltLen
    ) internal view returns (bool) {
        // Let rsaRecoverEncodedMessage handle stripping leading 0x00 and size checks
        bytes memory EM = rsaRecoverEncodedMessage(signature, modulusRaw, exponent);
        return _rsaPssVerifySha256FromEM(EM, message, saltLen);
    }

    /// @dev Internal helper that interprets EM as an RSASSA-PSS-encoded block.
    function _rsaPssVerifySha256FromEM(
        bytes memory EM,
        bytes memory message,
        uint256 saltLen
    ) internal pure returns (bool) {
        uint256 emLen = EM.length;
        uint256 hLen = 32;

        if (emLen < hLen + saltLen + 2) {
            return false; // modulus too short for given parameters
        }

        // 2. Check trailer byte 0xBC
        if (EM[emLen - 1] != 0xbc) {
            return false;
        }

        // 3. Split EM = DB || H || 0xBC
        uint256 dbLen = emLen - hLen - 1;
        bytes memory DB = new bytes(dbLen);
        bytes memory H = new bytes(hLen);

        for (uint256 i = 0; i < dbLen; i++) {
            DB[i] = EM[i];
        }
        for (uint256 i = 0; i < hLen; i++) {
            H[i] = EM[dbLen + i];
        }

        // (Optional EM[0] high-bit check removed to avoid bitlength edge-cases)

        // 4. dbMask = MGF1(H, dbLen); DB' = DB XOR dbMask
        bytes memory dbMask = mgf1Sha256(H, dbLen);
        for (uint256 i = 0; i < dbLen; i++) {
            DB[i] = DB[i] ^ dbMask[i];
        }

        // 5. Mask top unused bits of DB[0].
        DB[0] = bytes1(uint8(DB[0]) & 0x7f);

        // 6. Parse DB' = PS || 0x01 || salt
        uint256 idx = 0;
        while (idx < dbLen && DB[idx] == 0x00) {
            idx++;
        }
        if (idx >= dbLen || DB[idx] != 0x01) {
            return false;
        }
        idx++;

        if (dbLen - idx != saltLen) {
            return false;
        }
        bytes memory salt = new bytes(saltLen);
        for (uint256 i = 0; i < saltLen; i++) {
            salt[i] = DB[idx + i];
        }

        // 7–8. Compute H' = SHA-256(0x00^8 || SHA-256(message) || salt)
        bytes memory prefix = new bytes(8); // all zeros
        bytes32 hPrime = sha256(
            abi.encodePacked(
                prefix,
                sha256(message),
                salt
            )
        );

        // 9. Compare H and H'
        return keccak256(H) == keccak256(abi.encodePacked(hPrime));
    }

/// @dev MGF1 with SHA-256 as underlying hash.
    function mgf1Sha256(bytes memory seed, uint256 maskLen)
        internal
        pure
        returns (bytes memory)
    {
        uint256 hLen = 32;
        bytes memory mask = new bytes(maskLen);
        uint256 counter = 0;
        uint256 generated = 0;

        while (generated < maskLen) {
            // I2OSP(counter, 4)
            bytes memory c = abi.encodePacked(
                bytes1(uint8(counter >> 24)),
                bytes1(uint8(counter >> 16)),
                bytes1(uint8(counter >> 8)),
                bytes1(uint8(counter))
            );

            bytes32 digest = sha256(abi.encodePacked(seed, c));

            uint256 toCopy = maskLen - generated;
            if (toCopy > hLen) {
                toCopy = hLen;
            }

            for (uint256 i = 0; i < toCopy; i++) {
                mask[generated + i] = digest[i];
            }

            generated += toCopy;
            counter++;
        }

        return mask;
    }
}
