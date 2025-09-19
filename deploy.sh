#!/bin/bash
set -e

echo "🚀 Starting Anvil..."
anvil --port 8545 --silent &
ANVIL_PID=$!
sleep 3

echo "🚀 Starting DFX..."
dfx start --clean --background
sleep 5

echo "🚀 Deploying ICP canisters..."
dfx deploy

echo "📡 Fetching EVM address from ICP canister..."
EVM_ADDRESS=$(dfx canister call evm_with_icp_as_coprocessor_backend get_address | awk -F'"' '{print $2}')
echo "✅ EVM_ADDRESS = $EVM_ADDRESS"
echo "🚀 Deploying Coprocessor contract via Foundry..."
DEPLOY_OUTPUT=$(forge script script/Coprocessor.s.sol:MyScript \
  --fork-url http://127.0.0.1:8545 \
  --broadcast \
  --sig "run(address)" $EVM_ADDRESS)

# Debug raw output (optional)
# echo "RAW FORGE OUTPUT: $DEPLOY_OUTPUT"

# Extract the deployed address
CONTRACT_ADDRESS=$(echo "$DEPLOY_OUTPUT" \
  | grep -Eo "0x[0-9a-fA-F]{40}" \
  | tail -n 1)

echo "✅ CONTRACT_ADDRESS = $CONTRACT_ADDRESS"

# Call canister with just the address
dfx canister call evm_with_icp_as_coprocessor_backend set_contract_address "(\"$CONTRACT_ADDRESS\")"

# Start interval
dfx canister call evm_with_icp_as_coprocessor_backend start_with_interval_secs '(15)'

echo "🎉 Deployment complete!"
echo "👉 EVM contract deployed at: $CONTRACT_ADDRESS"
