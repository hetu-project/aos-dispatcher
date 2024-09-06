//! Example of generating code from ABI file using the `sol!` macro to interact with the contract.

use std::str::FromStr;

use alloy::{network::{EthereumWallet, NetworkWallet}, primitives::{address, Address, U256}, providers::ProviderBuilder, signers::local::{coins_bip39::English, MnemonicBuilder}, sol};
use anyhow::Result;

use crate::db::pg::model::Operator;

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    VRFRange,
    "abi/vrf_abi.json"
);

pub async fn register_operator(op: &Operator, start: u32, end: u32) -> Result<()> {
  let rpc_url = "https://1rpc.io/holesky".parse()?;
  let singer = MnemonicBuilder::<English>::default()
  .phrase("equal dragon fabric refuse stable cherry smoke allow alley easy never medal attend together lumber movie what sad siege weather matrix buffalo state shoot")
  // .index(index)?
  // Use this if your mnemonic is encrypted.
  // .password(password)
  .build()?;

  let wallet = EthereumWallet::from(singer);
  // wallet.default_signer().address();
  let owner_address = wallet.default_signer().address();
  tracing::info!("Wallet: {}", wallet.default_signer().address());
  let provider = ProviderBuilder::new()
    .with_recommended_fillers()
    .wallet(wallet)
    .on_http(rpc_url);

  let op_addr = Address::from_str(&op.address)?;

  let contract_addr = Address::from_str("0x27e4384ecc11810c2F49914390052b22c4e3CcC0")?;

  // Create a contract instance.
  let mut contract = VRFRange::new(contract_addr, provider);

  let total_operators = contract.getNumOperators().call().await?._0;

  tracing::info!("total operators of address {}", total_operators);


  let owner = contract.owner().call().await?._0;


  tracing::info!("owner of address {}", owner);

  // let total_operators = contract().call().await?._0;


  tracing::debug!("start submit register tx on chain");
  let register_r = contract.registerOperator(op_addr, U256::from(start), U256::from(end)).call().await;
  match register_r {
    Ok(r) => {
      tracing::debug!("register on chain success");

    },
    Err(e) => {
      tracing::error!("register on chain error {}", e);
      tracing::debug!("start update operator  on chain");
      let _ur = contract.updateOperatorRange(op_addr, U256::from(start), U256::from(end)).call().await?;
      tracing::debug!("update operator  on chain success");

    },
  }


  let total_operators = contract.getNumOperators().call().await?._0;
  tracing::debug!("operator on chain is {}", total_operators);
  Ok(())
}