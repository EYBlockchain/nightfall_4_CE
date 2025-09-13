// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";

/// Upgrade + deep logging for a UUPS RoundRobin proxy
/// Env:
///   NF4_SIGNING_KEY   - pk of RoundRobin owner (anvil key ok)
///   RPC_URL           - provided via CLI
///   ROUNDROBIN_PROXY  - proxy address (used by run())
///   RR_ARTIFACT       - optional; defaults to "RoundRobinV3.sol:RoundRobinV3"
contract UpgradeRoundRobinWithLogging is Script {
    // EIP-1967 slots
    bytes32 constant _IMPL_SLOT =
        0x360894A13BA1A3210667C828492DB98DCA3E2076CC3735A920A3CA505D382BBC;
    bytes32 constant _ADMIN_SLOT =
        0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103;

    // ---------- helpers ----------
    function _getImpl(address proxy) internal view returns (address impl) {
        bytes32 raw = vm.load(proxy, _IMPL_SLOT);
        impl = address(uint160(uint256(raw)));
    }

    function _getAdmin(address proxy) internal view returns (address admin) {
        bytes32 raw = vm.load(proxy, _ADMIN_SLOT);
        admin = address(uint160(uint256(raw)));
    }

    function _codehash(address a) internal view returns (bytes32 h) {
        assembly {
            h := extcodehash(a)
        }
    }

    function _ownerOf(
        address proxy
    ) internal view returns (address ownerAddr, bool ok) {
        (bool s, bytes memory ret) = proxy.staticcall(
            abi.encodeWithSignature("owner()")
        );
        if (!s || ret.length != 32) return (address(0), false);
        return (abi.decode(ret, (address)), true);
    }

    function _proxiableUUID(
        address proxy
    ) internal view returns (bytes32 uuid, bool ok) {
        // On UUPS proxies this usually reverts (notDelegated), so we treat failure as "not available".
        (bool s, bytes memory ret) = proxy.staticcall(
            abi.encodeWithSignature("proxiableUUID()")
        );
        if (!s || ret.length != 32) return (bytes32(0), false);
        return (abi.decode(ret, (bytes32)), true);
    }

    function _logProxyState(string memory tag, address proxy) internal view {
        address impl = _getImpl(proxy);
        address admin = _getAdmin(proxy);
        (address own, bool hasOwner) = _ownerOf(proxy);
        (bytes32 uuid, bool hasUUID) = _proxiableUUID(proxy);

        console.log("========== %s ==========", tag);
        console.log("Proxy:                 %s", proxy);
        console.log("  code length:         %u", proxy.code.length);
        console.logBytes32(_codehash(proxy));

        console.log("Implementation:        %s", impl);
        console.log("  code length:         %u", impl.code.length);
        console.logBytes32(_codehash(impl));

        console.log("Admin (EIP-1967):      %s", admin);
        if (hasOwner) console.log("owner():               %s", own);
        else console.log("owner():               <call failed or not present>");

        if (hasUUID) {
            console.log("proxiableUUID():");
            console.logBytes32(uuid);
        } else {
            console.log("proxiableUUID():       <reverted or not implemented>");
        }
        console.log("==============================");
    }

    // Helper to safely read optional string env without envOr ambiguity
    function __readEnvString(
        string memory name
    ) external view returns (string memory) {
        return vm.envString(name); // will revert if missing; caller will try/catch
    }

    // ---------- entrypoints ----------

    /// Preferred entrypoint: pass proxy + optional artifact
    function run(address proxy, string memory artifactOverride) public {
        // Set artifacts dir deterministically (avoid vm.envOr ambiguity entirely)
        vm.setEnv("FOUNDRY_OUT", "blockchain_assets/artifacts");

        require(proxy != address(0), "proxy arg is zero");
        require(proxy.code.length > 0, "Proxy has no code on RPC_URL");

        // Resolve artifact: default then optional RR_ARTIFACT env, then optional param override
        string memory artifact = "RoundRobinV3.sol:RoundRobinV3";
        // env override
        try this.__readEnvString("RR_ARTIFACT") returns (string memory rr) {
            if (bytes(rr).length != 0) artifact = rr;
        } catch {}
        // explicit param override
        if (bytes(artifactOverride).length != 0) {
            artifact = artifactOverride;
        }

        uint256 pk = vm.envUint("NF4_SIGNING_KEY"); // must be RoundRobin.owner

        address implBefore = _getImpl(proxy);
        _logProxyState("Before upgrade", proxy);

        vm.startBroadcast(pk);
        Upgrades.upgradeProxy(proxy, artifact, bytes(""));
        vm.stopBroadcast();

        address implAfter = _getImpl(proxy);
        _logProxyState("After upgrade", proxy);

        require(implAfter != address(0), "implAfter is zero");
        require(
            _codehash(implAfter) != _codehash(implBefore),
            "Implementation did not change"
        );
        console.log(
            "Upgrade successful: impl changed from %s to %s",
            implBefore,
            implAfter
        );
    }

    /// Convenience: pass only proxy (artifact from env or default)
    function run(address proxy) public {
        run(proxy, "");
    }

    /// Convenience: read both from env
    ///   ROUNDROBIN_PROXY (required)
    ///   RR_ARTIFACT      (optional)
    function run() external {
        address proxy = vm.envAddress("ROUNDROBIN_PROXY");
        run(proxy, "");
    }
}
