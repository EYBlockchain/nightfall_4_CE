// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

import "../contracts/proof_verification/RollupProofVerificationKey.sol";
import "../contracts/proof_verification/lib/Types.sol";

contract VKProviderTest is Test {
    RollupProofVerificationKey vk;

    function setUp() public {
        // Build a minimal VK struct
        Types.VerificationKey memory initVK;
        initVK.domain_size = 0x2000000;

        // Deploy impl
        RollupProofVerificationKey impl = new RollupProofVerificationKey();

        // Encode initializer and deploy proxy
        bytes memory initData = abi.encodeCall(
            RollupProofVerificationKey.initialize,
            (abi.encode(initVK))
        );
        vk = RollupProofVerificationKey(
            address(new ERC1967Proxy(address(impl), initData))
        );
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
        uint64 oldVer = vk.vkVersion();

        vk.replaceVK(abi.encode(newVK));

        Types.VerificationKey memory t = vk.getVerificationKey();
        assertEq(t.domain_size, 0x3000000, "domain_size not updated by replaceVK");
        assertEq(vk.vkVersion(), oldVer + 1, "vkVersion did not increment");
        assertTrue(vk.vkHash() != oldHash, "vkHash did not change");
    }

    function testOnlyOwnerCanReplace() public {
        address attacker = address(0xBEEF);
        vm.prank(attacker);

        Types.VerificationKey memory newVK;
        newVK.domain_size = 0x4000000;

        // OZ v5 reverts with OwnableUnauthorizedAccount; a generic expectRevert is fine
        vm.expectRevert();
        vk.replaceVK(abi.encode(newVK));
    }

    function testInitializeOnlyOnce() public {
        // Re-initializing the *proxy* must revert due to `initializer`
        Types.VerificationKey memory again;
        again.domain_size = 0x2000000;

        vm.expectRevert(); // "Initializable: contract is already initialized"
        vk.initialize(abi.encode(again));
    }
}
