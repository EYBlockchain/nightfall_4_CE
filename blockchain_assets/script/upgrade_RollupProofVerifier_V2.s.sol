// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
import {RollupProofVerifierV2} from "../contracts/proof_verification/RollupProofVerifierV2.sol";

// Minimal UUPS interface
interface IUUPS {
    function upgradeTo(address newImplementation) external;
}

// Read proxiable UUID to assert UUPS-compat
interface IProxiable {
    function proxiableUUID() external view returns (bytes32);
}

contract UpgradeRollupProofVerifier is Script {
    // EIP-1967 impl slot = keccak256("eip1967.proxy.implementation") - 1
    bytes32 constant _IMPL_SLOT =
        0x360894A13BA1A3210667C828492DB98DCA3E2076CC3735A920A3CA505D382BBC;

    function run() external {
        // ---------------------------------------------------------------------
        // Env:
        //   VERIFIER_PROXY  = address of the existing RollupProofVerifier proxy
        //   DEPLOYER_PK     = private key of the proxy owner (uint)
        // ---------------------------------------------------------------------
        address proxy = vm.envAddress("VERIFIER_PROXY");
        uint256 pk    = vm.envUint("DEPLOYER_PK");

        vm.startBroadcast(pk);

        // 1) Deploy new implementation
        RollupProofVerifierV2 impl = new RollupProofVerifierV2();

        // 2) Sanity-check: V2 is UUPS-compatible (proxiableUUID matches EIP-1967 slot)
        bytes32 uuid = IProxiable(address(impl)).proxiableUUID();
        require(uuid == _IMPL_SLOT, "V2 impl not UUPS-compatible");

        // 3) Perform the upgrade (owner-only on the proxy)
        IUUPS(proxy).upgradeTo(address(impl));

        vm.stopBroadcast();
    }
}
