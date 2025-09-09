// SPDX-License-Identifier: CC0
pragma solidity ^0.8.20;

import "./Nightfall.sol";
import "forge-std/console.sol";


/// @custom:oz-upgrades-from blockchain_assets/contracts/Nightfall.sol:Nightfall
contract NightfallV3 is Nightfall {
    /// @dev V3: do NOT deal with deposit_fee.
    function escrow_funds(
        uint256 fee,
        address ercAddress,
        uint256 tokenId,
        uint256 value,
        uint256 secretHash,
        TokenType token_type
    ) external virtual override payable onlyCertified {
        uint256 nfTokenId = sha256_and_shift(abi.encode(ercAddress, tokenId));
        tokenIdMapping[nfTokenId] = TokenIdValue(ercAddress, tokenId);

        uint256 nfSlotId = (token_type == TokenType.ERC3525)
            ? uint256(keccak256(abi.encode(ercAddress, IERC3525(ercAddress).slotOf(tokenId)))) >> 4
            : nfTokenId;

        DepositCommitment memory valueCommitment = DepositCommitment(nfTokenId, nfSlotId, value, secretHash);
        uint256 key = sha256_and_shift(abi.encode(valueCommitment));

        require(feeBinding[key].escrowed == 0, "Funds have already been escrowed for this Deposit");

        if (token_type == TokenType.ERC3525) {
            ERC3525(ercAddress).transferFrom(msg.sender, address(this), tokenId);
        } else if (token_type == TokenType.ERC1155) {
            IERC1155(ercAddress).safeTransferFrom(msg.sender, address(this), tokenId, value, "");
        } else if (token_type == TokenType.ERC721) {
            require(value == 0, "ERC721 tokens should have a value of zero");
            IERC721(ercAddress).safeTransferFrom(msg.sender, address(this), tokenId, "");
        } else if (token_type == TokenType.ERC20) {
            require(tokenId == 0, "ERC20 tokens should have a tokenId of 0");
            require(IERC20(ercAddress).transferFrom(msg.sender, address(this), value), "ERC20 transfer failed");
        } else {
            revert escrowFundsError();
        }

        feeBinding[key] = DepositFeeState(fee, 1, 0);
        emit DepositEscrowed(nfSlotId, value);
    }
    // Simple marker for sanity-check behavior after upgrade
    function versionMarker() external pure returns (string memory) {
        return "NightfallV3-no-deposit-commitment-event";
    }
}
