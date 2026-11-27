#!/bin/sh
set -e

# Start anvil
anvil --base-fee 58000000000 --block-time 5 --gas-limit 500000000 &
ANVIL_PID=$!

# Wait for anvil to be ready (JSON-RPC POST)
for i in $(seq 1 30); do
    if curl -s -X POST --data '{"jsonrpc":"2.0","id":1,"method":"eth_chainId","params":[]}' \
        -H "Content-Type: application/json" http://localhost:8545 | grep -q '"result"'; then
        break
    fi
    echo "Waiting for anvil to be ready... ($i)"
    sleep 1
done

# The host user has no root, but this container does. Leave the bind-mounted
# broadcast directory writable so a later host forge can create chain-id folders.
mkdir -p /blockchain_assets/logs
chmod -R a+rwX /blockchain_assets/logs || true
umask 000

# Run the deployment script with Foundry
forge script MockDeployer \
    --fork-url ws://localhost:8545 \
    --broadcast \
    --force

chmod -R a+rwX /blockchain_assets/logs || true

wait $ANVIL_PID

exec "$@"
