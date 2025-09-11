// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";

contract UpgradeX509WithLogging is Script {
    bytes32 constant _IMPL_SLOT  = 0x360894A13BA1A3210667C828492DB98DCA3E2076CC3735A920A3CA505D382BBC;
    bytes32 constant _ADMIN_SLOT = 0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103;
    string constant ARTIFACT = "X509V4.sol:X509V4";

    function _getImpl(address proxy) internal view returns (address impl) {
        bytes32 raw = vm.load(proxy, _IMPL_SLOT);
        impl = address(uint160(uint256(raw)));
    }
    function _getAdmin(address proxy) internal view returns (address admin) {
        bytes32 raw = vm.load(proxy, _ADMIN_SLOT);
        admin = address(uint160(uint256(raw)));
    }
    function _codehash(address a) internal view returns (bytes32 h) { assembly { h := extcodehash(a) } }
    function _proxiableUUID(address proxy) internal view returns (bytes32 uuid, bool ok) {
        (bool success, bytes memory ret) = proxy.staticcall(abi.encodeWithSignature("proxiableUUID()"));
        if (!success || ret.length == 0) return (bytes32(0), false);
        return (abi.decode(ret, (bytes32)), true);
    }
    function _logProxyState(string memory tag, address proxy) internal view {
        address impl  = _getImpl(proxy);
        address admin = _getAdmin(proxy);
        (bytes32 uuid, bool hasUUID) = _proxiableUUID(proxy);

        console.log("========== %s ==========", tag);
        console.log("Proxy:                %s", proxy);
        console.log("  code length:        %u", proxy.code.length);
        console.logBytes32(_codehash(proxy));
        console.log("Implementation:        %s", impl);
        console.log("  code length:        %u", impl.code.length);
        console.logBytes32(_codehash(impl));
        console.log("Admin (EIP-1967):     %s", admin);
        if (hasUUID) { console.log("proxiableUUID():"); console.logBytes32(uuid); }
        else { console.log("proxiableUUID(): <call failed or not implemented>"); }
        console.log("==============================");
    }

    // Preferred entrypoint: pass the proxy address
    function run(address proxy) public {
        vm.setEnv("FOUNDRY_OUT", "blockchain_assets/artifacts");
        require(proxy != address(0), "proxy arg is zero");
        require(proxy.code.length > 0, "X509 proxy has no code on RPC_URL");

        uint256 pk = vm.envUint("NF4_SIGNING_KEY"); // must be proxy owner

        address implBefore = _getImpl(proxy);
        _logProxyState("Before upgrade", proxy);

        vm.startBroadcast(pk);
        Upgrades.upgradeProxy(proxy, ARTIFACT, bytes(""));
        vm.stopBroadcast();

        address implAfter = _getImpl(proxy);
        _logProxyState("After upgrade", proxy);
        require(implAfter != address(0), "implAfter is zero");
        require(implAfter != implBefore, "Implementation did not change");
        console.log("Upgrade successful: impl changed from %s to %s", implBefore, implAfter);
    }

    // Convenience entrypoint: read proxy from env X509_PROXY
    function run() external {
        address proxy = vm.envAddress("X509_PROXY");
        run(proxy);
    }
}
