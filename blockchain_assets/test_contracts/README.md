# Running Smart Contract Tests

To run smart contract tests it's necessary to install Foundry (or at least Forge). The easiest way to do that is:
```sh
curl -L https://foundry.paradigm.xyz | bash
```
Then, in the nightfall_4 root directory, install the forge standard libary (do _not_ omit the `--no-git` argument):
```sh
forge install --no-git foundry-rs/forge-std
```
This will install the forge standard library in the directory `forge`

Both of the above steps need only be done once!

Once installation is complete, you can run all the tests in `nightfall_deployer/test_contracts` with:
```sh
forge test
```
