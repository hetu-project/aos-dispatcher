//! Example of generating code from ABI file using the `sol!` macro to interact with the contract.

use std::str::FromStr;

use alloy::{
    network::{AnyNetwork, EthereumWallet},
    primitives::{Address, U256},
    providers::ProviderBuilder,
    signers::local::{coins_bip39::English, MnemonicBuilder},
    sol,
};
use anyhow::{Context, Result};

use crate::{config::CustomConfig, db::pg::model::Operator};

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    VRFRange,
    "abi/vrf_abi.json"
);

use thiserror::Error;
#[derive(Error, Debug)]
pub enum RegisterError {
    #[error("ConfigNotFound")]
    ConfigNotFound,
}

pub async fn register_operator(
    op: &Operator,
    start: u32,
    end: u32,
    config: &CustomConfig,
) -> Result<()> {
    let register = config
        .register
        .clone()
        .ok_or(RegisterError::ConfigNotFound)
        .context("register config not found")?;
    let endpoint = register
        .endpoint
        .ok_or(RegisterError::ConfigNotFound)
        .context("endpoint is not found")?;
    let account = register
        .account
        .ok_or(RegisterError::ConfigNotFound)
        .context("account is not found")?;
    let contract_address = register
        .contract
        .ok_or(RegisterError::ConfigNotFound)
        .context("contract is not found")?;
    let rpc_url = endpoint.parse()?;
    let singer = MnemonicBuilder::<English>::default()
        .phrase(account)
        // .index(index)?
        // Use this if your mnemonic is encrypted.
        // .password(password)
        .build()?;

    let singer_address = singer.address().to_string();

    tracing::info!("singer_address: {}", singer_address);

    let wallet = EthereumWallet::from(singer);
    // wallet.default_signer().address();
    let owner_address = wallet.default_signer().address();
    tracing::info!("Wallet: {}", owner_address);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .network::<AnyNetwork>()
        .wallet(wallet)
        .on_http(rpc_url);

    let op_addr = Address::from_str(&op.address)?;

    let contract_addr = Address::from_str(&contract_address)?;

    // Create a contract instance.
    let contract = VRFRange::new(contract_addr, provider);

    let total_operators = contract.getNumOperators().call().await?._0;

    tracing::info!("total operators of address {}", total_operators);

    let owner = contract.owner().call().await?._0;

    tracing::info!("owner of address {}", owner);

    // let total_operators = contract().call().await?._0;

    tracing::debug!("start submit register tx on chain");
    match contract
        .registerOperator(op_addr, U256::from(start), U256::from(end))
        // .call().await;
        .send()
        .await
    {
        Ok(r) => {
            r.get_receipt().await.unwrap();
            tracing::debug!("register on chain success");
        }
        Err(e) => {
            tracing::error!("register on chain error {}", e);
            tracing::debug!("start update operator  on chain");
            let _ur = contract
                .updateOperatorRange(op_addr, U256::from(start), U256::from(end))
                .send()
                .await?
                .get_receipt()
                .await?;
            // .call().await?;
            tracing::debug!("update operator  on chain success");
        }
    }

    let total_operators = contract.getNumOperators().call().await?._0;
    tracing::debug!("operator on chain is {}", total_operators);
    Ok(())
}
