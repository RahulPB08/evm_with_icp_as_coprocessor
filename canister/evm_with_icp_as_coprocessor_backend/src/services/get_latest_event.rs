use crate::{
    create_icp_signer, get_rpc_service_anvil, submit_result, Address, BlockNumberOrTag,
    Coprocessor, EthereumWallet, Filter, IcpConfig, Provider, ProviderBuilder,
};
use alloy::sol_types::SolEvent;
use std::cell::RefCell;

thread_local! {
    static LAST_LOG_ID: RefCell<Option<String>> = RefCell::new(None);
}

pub async fn get_latest_event_fn(contract_address: Address) -> Result<String, String> {
    let signer = create_icp_signer().await;
    let _wallet = EthereumWallet::new(signer);
    let rpc_service = get_rpc_service_anvil();
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new().on_icp(config);

    let filter = Filter::new()
        .address(contract_address)
        .event("NewJob(uint256)")
        .from_block(BlockNumberOrTag::Latest);

    let logs = provider
        .get_logs(&filter)
        .await
        .map_err(|e| format!("Failed to get logs: {}", e))?;

    if let Some(log) = logs.first() {
        // extract unique identifier (tx_hash + log_index is safest)
        let tx_hash = log
            .transaction_hash
            .map(|h| format!("{:?}", h))
            .unwrap_or_default();
        let log_index = log.log_index.map(|i| i.to_string()).unwrap_or_default();
        let unique_id = format!("{}-{}", tx_hash, log_index);

        let is_new = LAST_LOG_ID.with(|last| {
            let mut last_mut = last.borrow_mut();
            if last_mut.as_ref() != Some(&unique_id) {
                *last_mut = Some(unique_id.clone());
                true
            } else {
                false
            }
        });

        if is_new {
            match Coprocessor::NewJob::decode_log(log.as_ref(), true) {
                Ok(decoded_event) => {
                    let jobId = decoded_event.data.job_id;
                    // Pass the extracted uint256 to submit_result_fn
                    let job_id: u64 = jobId.try_into().unwrap_or(0);
                    match submit_result::submit_result_fn(job_id, contract_address).await {
                        Ok(result) => Ok(format!(
                            "New job created with ID: {}. Result: {}",
                            job_id, result
                        )),
                        Err(e) => Err(format!("submit_result_fn failed: {}", e)),
                    }
                }
                Err(e) => Err(format!("Failed to decode event: {}", e)),
            }
        } else {
            Ok("No new logs (same as last one)".to_string())
        }
    } else {
        Ok("No logs found".to_string())
    }
}
