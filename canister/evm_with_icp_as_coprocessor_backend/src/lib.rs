pub mod services;
use alloy::primitives::Address;
use alloy::primitives::{address, U256};
use alloy::rpc::types::Filter;
use alloy::signers::icp::IcpSigner;
use alloy::{contract, sol};
use alloy::{
    eips::BlockNumberOrTag,
    network::EthereumWallet,
    providers::{Provider, ProviderBuilder},
    rpc::types::Log,
    transports::icp::{IcpConfig, RpcApi, RpcService},
};
use services::get_latest_event::get_latest_event_fn;
use services::job::calculate_fibonacci;
use services::submit_result;
use services::submit_result::submit_result_fn;

sol!(
    #[sol(rpc)]
    "../../src/Coprocessor.sol"
);
use std::cell::{self, RefCell};
use std::time::Duration;

thread_local! {
    static TIMER_ID: RefCell<Option<ic_cdk_timers::TimerId>> = RefCell::new(None);
     static EVM_CONTRACT_ADDRESS: RefCell<Option<String>> = RefCell::new(None);
}

pub fn get_contract_address_as_address() -> Result<Option<Address>, String> {
    let opt_str = EVM_CONTRACT_ADDRESS.with(|cell| cell.borrow().clone());

    if let Some(addr_str) = opt_str {
        // Parse string into Address
        let address = addr_str
            .parse::<Address>()
            .map_err(|e| format!("Invalid Ethereum address: {}", e))?;
        Ok(Some(address))
    } else {
        Ok(None) // No address stored yet
    }
}

fn get_rpc_service_anvil() -> RpcService {
    RpcService::Custom(RpcApi {
        url: "http://127.0.0.1:8545".to_string(),
        headers: None,
    })
}
pub fn get_ecdsa_key_name() -> String {
    #[allow(clippy::option_env_unwrap)]
    let dfx_network = option_env!("DFX_NETWORK").unwrap();
    match dfx_network {
        "ic" => "key_1".to_string(),
        _ => "dfx_test_key".to_string(),
    }
}
#[ic_cdk::update]
async fn call_to_get_latest_event() {
    // Get the address from thread-local storage
    let contract_address = match get_contract_address_as_address() {
        Ok(Some(addr)) => addr, // got valid Address
        Ok(None) => {
            let msg = "No contract address set".to_string();
            ic_cdk::println!("{}", msg);
            return;
        }
        Err(err) => {
            let msg = format!("Failed to parse contract address: {}", err);
            ic_cdk::println!("{}", msg);
            return;
        }
    };

    // Call the async function with a proper Address
    let res = get_latest_event_fn(contract_address).await;
    match res {
        Ok(val) => {
            ic_cdk::println!("Event result: {}", val);
        }
        Err(err) => {
            ic_cdk::println!("Error: {}", err);
        }
    }
}

#[ic_cdk::update]
async fn submit_result_fn_call() -> Result<String, String> {
    let contract_address = match get_contract_address_as_address() {
        Ok(Some(addr)) => addr, // got valid Address
        Ok(None) => {
            let msg = "No contract address set".to_string();
            ic_cdk::println!("{}", msg);
            return Err(msg);
        }
        Err(err) => {
            let msg = format!("Failed to parse contract address: {}", err);
            ic_cdk::println!("{}", msg);
            return Err(msg);
        }
    };
    let res = submit_result_fn(3, contract_address).await;

    match res {
        Ok(val) => {
            ic_cdk::println!("Event result: {}", val);
            Ok(val)
        }
        Err(err) => {
            ic_cdk::println!("Error: {}", err);
            Err(err)
        }
    }
}

async fn create_icp_signer() -> IcpSigner {
    let ecdsa_key_name = get_ecdsa_key_name();
    IcpSigner::new(vec![], &ecdsa_key_name, None).await.unwrap()
}
#[ic_cdk::update]
pub fn set_contract_address(addr: String) {
    EVM_CONTRACT_ADDRESS.with(|cell| {
        *cell.borrow_mut() = Some(addr.to_string());
    });
}

#[ic_cdk::update]
async fn get_address() -> Result<String, String> {
    let signer = create_icp_signer().await;
    let address2 = alloy::signers::Signer::address(&signer);

    Ok(address2.to_string())
}

#[ic_cdk::update]
pub async fn start_with_interval_secs(secs: u64) {
    let secs = Duration::from_secs(secs);

    let id = ic_cdk_timers::set_timer_interval(secs, || {
        ic_cdk::spawn(call_to_get_latest_event()); // async inside timer
    });

    TIMER_ID.with(|t| *t.borrow_mut() = Some(id));
}

#[ic_cdk::update]
pub async fn stop_timers() {
    TIMER_ID.with(|t| {
        if let Some(id) = t.borrow_mut().take() {
            ic_cdk_timers::clear_timer(id);
        }
    });
}
