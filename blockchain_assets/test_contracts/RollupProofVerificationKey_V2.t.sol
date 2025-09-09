// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "forge-std/Test.sol";
import "../contracts/proof_verification/RollupProofVerificationKey.sol";
import "../contracts/proof_verification/lib/Types.sol";

contract VKProviderTest is Test {
    RollupProofVerificationKey vk;

    function setUp() public {
        vk = new RollupProofVerificationKey();

        // Build a minimal VK struct: set what you care about; the rest defaults to zero
        Types.VerificationKey memory initVK;
        initVK.domain_size = 0x2000000;

        // Initialize with encoded struct
        vk.initialize(abi.encode(initVK));
    }

    function testGetVK() public view {
        Types.VerificationKey memory t = vk.getVerificationKey();
        assertEq(t.domain_size, 0x2000000, "domain_size mismatch after initialize");
    }

    function testReplaceVK_UpdatesAllAndBumpsVersion() public {
        // Prepare a new VK with a different domain size
        Types.VerificationKey memory newVK;
        newVK.domain_size = 0x3000000;

        bytes32 oldHash = vk.vkHash();
        uint64  oldVer  = vk.vkVersion();

        vk.replaceVK(abi.encode(newVK));

        Types.VerificationKey memory t = vk.getVerificationKey();
        assertEq(t.domain_size, 0x3000000, "domain_size not updated by replaceVK");

        // Version increments
        assertEq(vk.vkVersion(), oldVer + 1, "vkVersion did not increment");

        // Hash changes
        assertTrue(vk.vkHash() != oldHash, "vkHash did not change");
    }

    function testOnlyOwnerCanReplace() public {
        // another account tries to replace
        address attacker = address(0xBEEF);
        vm.prank(attacker);

        Types.VerificationKey memory newVK;
        newVK.domain_size = 0x4000000;

        vm.expectRevert(); // OwnableUnauthorizedAccount (OZ v5)
        vk.replaceVK(abi.encode(newVK));
    }

    function testInitializeOnlyOnce() public {
        // Re-initialize must revert due to `initializer`
        Types.VerificationKey memory again;
        again.domain_size = 0x2000000;

        vm.expectRevert(); // Initializable: contract is already initialized
        vk.initialize(abi.encode(again));
    }
}
