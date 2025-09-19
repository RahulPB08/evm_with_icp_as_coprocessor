evm_with_icp_as_coprocessor

A reproducible developer setup that deploys a Solidity Coprocessor contract to a local Anvil (Foundry) node, then passes the deployed contract address into an Internet Computer (ICP) Rust canister (evm_with_icp_as_coprocessor_backend). The repository integrates Foundry (forge + anvil), dfx canisters, and the alloy/ic_alloy Rust crates used in the canister to interact with EVM primitives.

This README explains project structure, the runtime architecture, and includes a robust deploy.sh that: start/uses Anvil, starts dfx, deploys canisters, runs a Foundry script to deploy the Solidity contract, extracts the contract address, writes it into the ICP canister via a canister call, and starts the canister's interval job.

Architecture (high level)

Anvil (Foundry): Local EVM JSON-RPC endpoint that the Foundry forge script uses to broadcast transactions and deploy the Coprocessor contract.

Foundry script: script/Coprocessor.s.sol:MyScript compiles and deploys the Solidity contract and returns the deployed address.

DFX / ICP canisters:

evm_rpc (local evm-rpc canister used by your project if applicable)

evm_with_icp_as_coprocessor_backend — Rust canister that stores the EVM account/coprocessor address and runs periodic tasks.

Glue: deploy.sh orchestrates all steps, extracts the contract address and calls the canister's set_contract_address and start_with_interval_secs methods.