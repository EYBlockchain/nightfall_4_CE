// test/RollupProofVerificationKeyUUPS.t.sol
pragma solidity ^0.8.24;

import "forge-std/Test.sol";
import "../contracts/proof_verification/RollupProofVerificationKeyUUPS.sol";
import "../contracts/proof_verification/Types.sol";

contract VKProviderTest is Test {
    RollupProofVerificationKeyUUPS vk;

    function setUp() public {
        vk = new RollupProofVerificationKeyUUPS();
        // vk.initialize(0x2000000, 3); // example nPublicInputs = 3
        vk.initialize(0x2000000);
    }

    function testGetVK() public view {
        Types.VerificationKey memory t = vk.getVerificationKey();
        assertEq(t.domain_size, 0x2000000);
        // assertEq(t.nPublicInputs, 3);
    }

    function testSetDomainSize() public {
        vk.setDomainSize(0x2000000);
        Types.VerificationKey memory t = vk.getVerificationKey();
        assertEq(t.domain_size, 0x2000000);
    }

    function testRevertsOnNonPowerOfTwo() public {
        vm.expectRevert(RollupProofVerificationKeyUUPS.DomainSizeMustBePowerOfTwo.selector);
        vk.setDomainSize(12345);
    }
}
