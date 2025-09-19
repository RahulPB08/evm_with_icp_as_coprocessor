use crate::{
    calculate_fibonacci, create_icp_signer, get_address, get_rpc_service_anvil, Address,
    Coprocessor, EthereumWallet, IcpConfig, Provider, ProviderBuilder, RefCell, U256,
};

thread_local! {
    static NONCE: RefCell<Option<u64>> = const { RefCell::new(None) };
}

pub async fn submit_result_fn(jobId: u64, contract_address: Address) -> Result<String, String> {
    let signer = create_icp_signer().await;
    let evm_address_for_canister = get_address().await;
    let address2 = alloy::signers::Signer::address(&signer);
    // let evm_contract_address="0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512";
    let wallet = EthereumWallet::new(signer);
    let rpc_service = get_rpc_service_anvil();
    let chain_id = 31337;
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new()
        .with_gas_estimation()
        .wallet(wallet)
        .on_icp(config);
    // let contract_address = address!("0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512");
    let contract = Coprocessor::new(contract_address, provider.clone());

    let maybe_nonce = NONCE.with_borrow(|maybe_nonce| maybe_nonce.map(|nonce| nonce + 1));
    let nonce = if let Some(nonce) = maybe_nonce {
        nonce
    } else {
        // First time: fetch from provider
        provider.get_transaction_count(address2).await.unwrap_or(0)
    };
    // let nonce = nonce();
    let res_calculate_fibonacci = calculate_fibonacci::fibonacci_iterative(jobId);

    match contract
        .callback_icp(U256::from(res_calculate_fibonacci), U256::from(jobId))
        .nonce(nonce)
        .from(address2)
        .chain_id(chain_id)
        .send()
        .await
    {
        Ok(res) => {
            let node_hash = *res.tx_hash();
            let tx_response = contract.provider().get_transaction_by_hash(node_hash).await;

            match tx_response {
                Ok(Some(_tx)) => {
                    // Update stored nonce
                    NONCE.with(|cell| {
                        *cell.borrow_mut() = Some(nonce);
                    });
                    ic_cdk::println!("Successfully ran tx: {}", res.tx_hash());
                }
                Ok(None) => ic_cdk::println!("Could not get transaction."),
                Err(e) => ic_cdk::println!("Error fetching tx: {}", e),
            }
        }
        Err(e) => {
            ic_cdk::println!("{}", e.to_string());
        }
    }

    match contract.getResult(U256::from(jobId)).call().await {
        Ok(val) => {
            let onchain_result = val._0; // U256
            let local_u256 = U256::from(res_calculate_fibonacci);

            if onchain_result == local_u256 {
                Ok(format!(
                    "Success ✅ | Local = {}, On-chain = {}",
                    local_u256,
                    onchain_result
                ))
            } else {
                Err(format!(
                    "Mismatch ❌ | Local = {}, On-chain = {}",
                    local_u256,
                    onchain_result
                ))
            }
        }
        Err(err) => Err(format!("Failed to fetch result from contract: {}", err)),
    }
}
