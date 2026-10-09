#!/usr/bin/env bash
set -euo pipefail

echo "=== 1. Running cargo test ==="
cargo test

echo "=== 2. Generating/funding deployer key on testnet ==="
stellar keys generate deployer --network testnet --fund || echo "Deployer key already exists, funding..."
stellar keys fund deployer --network testnet || true

echo "=== 3. Building contract wasm ==="
stellar contract build

WASM_PATH="target/wasm32-unknown-unknown/release/gig_escrow_contracts.wasm"
if [ ! -f "$WASM_PATH" ]; then
    WASM_PATH="target/wasm32v1-none/release/gig_escrow_contracts.wasm"
fi

echo "=== 4. Deploying contract to testnet ==="
CONTRACT_ID=$(stellar contract deploy \
  --wasm "$WASM_PATH" \
  --source deployer \
  --network testnet \
  --alias fairlance || echo "")

echo "Contract deployed: $CONTRACT_ID"

echo "=== 5. Getting native XLM contract id ==="
TOKEN_ID=$(stellar contract id asset --asset native --network testnet)
echo "Native XLM contract id: $TOKEN_ID"

DEPLOYER_ADDR=$(stellar keys address deployer)

cat <<EOF > contract-ids.json
{
  "network": "testnet",
  "contractId": "$CONTRACT_ID",
  "contractAlias": "fairlance",
  "nativeTokenContractId": "$TOKEN_ID",
  "deployerAddress": "$DEPLOYER_ADDR",
  "rpcUrl": "https://soroban-testnet.stellar.org",
  "networkPassphrase": "Test SDF Network ; September 2015"
}
EOF

echo "Saved deployment details to contract-ids.json"
