# Nightfall Allowlisting adaptions

Nightfall now incorporates the ability to manage a allowlist of accounts. It is an abstract contract, intended to be subclassed by a form of Allowlist Manager contract (currentyl `X509.sol` performs this role). When allowlisting is enabled, only accounts that are added to the allowlist are able to move funds from Layer 1 to Layer 2 and to withdraw Layer 1 funds from the Nightfall contract.

## Enabling Allowlisting

To enable allowlisting, the deployer container should have its `ALLOWLISTING` environment variable set to `enable`. Setting the `ALLOWLISTING` variable to anything else will desable whitlisting.

## Operating Allowlisting

All allowlisting functionality is managed by the contract `Allowlist.sol`, the functions therein are self-explanatory.
