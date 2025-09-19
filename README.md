# EVM with ICP as Coprocessor

A reproducible developer setup that deploys a Solidity Coprocessor contract to a local Anvil (Foundry) node, then passes the deployed contract address into an Internet Computer (ICP) Rust canister (evm_with_icp_as_coprocessor_backend). The repository integrates Foundry (forge + anvil), dfx canisters, and the alloy/ic_alloy Rust crates used in the canister to interact with EVM primitives.

This README explains project structure, the runtime architecture, and includes a robust deploy.sh that: start/uses Anvil, starts dfx, deploys canisters, runs a Foundry script to deploy the Solidity contract, extracts the contract address, writes it into the ICP canister via a canister call, and starts the canister's interval job.

---

## Libraries and Tools Used

### 🦀 Rust & ICP

* **`ic-cdk`**: Core library for writing Internet Computer canisters in Rust. It provides macros and APIs to expose update/query functions and handle inter-canister calls.
* **`ic-cdk-timers`**: Used to schedule recurring tasks inside the canister (e.g., interval-based processing).
* **`candid`**: Interface description language and serialization format for ICP. It defines the canister’s public methods and types.
* **`alloy`**: Modern Ethereum Rust library. Provides support for:

  * **ABI generation** (`sol!` macro for binding Solidity contracts)
  * **EVM primitives** (address, U256, logs, events)
  * **Providers** for making JSON-RPC calls to Ethereum nodes
  * **Signers** for handling Ethereum transactions 

### ⚡ Ethereum & Foundry

* **`foundry (forge)`**: Ethereum development toolkit for building, testing, and deploying smart contracts. Used here to deploy contracts to Anvil.
* **`anvil`**: Local Ethereum testnet (similar to Ganache/Hardhat) for running contracts and testing transactions.

### 🛠 Shell Tools

* **`bash`**: For scripting deployment steps (`deploy.sh`). Automates starting Anvil, running `dfx`, deploying canisters, and deploying Ethereum contracts.
* **`awk` / `grep` / `tail` / `tr`**: Standard Unix text processing tools used to extract contract addresses from Foundry deployment logs.

### 📦 DFX (Dfinity SDK)

* **`dfx`**: Command-line interface for managing Internet Computer projects. Used to start the replica, deploy canisters, and call canister methods.

---

## How These Work Together

* **ICP libraries (`ic-cdk`, `candid`)**: Allow Rust canisters to define methods for interacting with Ethereum via JSON-RPC.
* **Alloy**: Bridges Rust with Ethereum by providing primitives and contract bindings.
* **Foundry (forge)**: Deploys Solidity contracts locally to Anvil.
* **Anvil**: Provides a local Ethereum RPC endpoint for testing.
* **dfx**: Handles canister deployment and execution on the ICP side.
* **Shell script**: Orchestrates everything end-to-end with a single `./deploy.sh`.

---

## Usage

Run:

```bash
./deploy.sh
```

This starts Anvil, runs DFX, deploys ICP canisters, deploys the EVM contract, and wires them together.

---

## Notes

* Make sure **Foundry** and **DFX** are installed before running the script.
* Requires Rust (nightly recommended), Node.js, and ICP SDK setup.
