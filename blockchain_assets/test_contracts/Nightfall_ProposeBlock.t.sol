// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

import "../contracts/Nightfall.sol";
import "../contracts/ProposerManager.sol";
import "../contracts/SanctionsListMock.sol";
import "../contracts/X509/X509.sol";
import "../contracts/proof_verification/MockVerifier.sol";

contract PMMock {
    address private _current;

    constructor(address initial) {
        _current = initial;
    }

    function setCurrent(address a) external {
        _current = a;
    }

    function get_current_proposer_address() external view returns (address) {
        return _current;
    }
}

contract NightfallProposeBlockTest is Test {
    Nightfall private nf;
    X509 private x509;
    SanctionsListMock private sanctions;
    MockVerifier private verifier;
    PMMock private pm;

    function setUp() public {
        // Deploy X509 through proxy because implementation disables initializers in constructor.
        X509 x509Impl = new X509();
        bytes memory x509Init = abi.encodeCall(X509.initialize, (address(this)));
        x509 = X509(address(new ERC1967Proxy(address(x509Impl), x509Init)));

        // Disable allowlisting so onlyCertified checks pass for the test caller.
        x509.enableAllowlisting(false);

        sanctions = new SanctionsListMock(address(0x1234));
        verifier = new MockVerifier();

        Nightfall impl = new Nightfall();
        uint256 initialNullifierRoot = 5626012003977595441102792096342856268135928990590954181023475305010363075697;
        bytes memory init = abi.encodeCall(
            Nightfall.initialize,
            (
                initialNullifierRoot,
                uint256(0),
                uint256(0),
                int256(0),
                verifier,
                address(x509),
                address(sanctions)
            )
        );

        nf = Nightfall(payable(address(new ERC1967Proxy(address(impl), init))));
        pm = new PMMock(address(this));
        nf.set_proposer_manager(ProposerManager(address(pm)));
    }

    function test_propose_block_reverts_on_block_number_mismatch() public {
        OnChainTransaction[] memory transactions = new OnChainTransaction[](0);
        Block memory blk = Block({
            commitments_root: 0,
            nullifier_root: 0,
            commitments_root_root: 0,
            transactions: transactions,
            rollup_proof: bytes(""),
            block_number: 1
        });

        vm.expectRevert("Nightfall: block number mismatch");
        nf.propose_block(blk);
    }
}
