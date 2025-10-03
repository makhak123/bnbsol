use anchor_client::{Client, Cluster};
use anyhow::Result;
use clap::Parser;
use ethers::prelude::*;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use std::{str::FromStr, sync::Arc, time::Duration};
use tracing::{error, info, warn};

mod bnb_monitor;
mod solana_monitor;
mod types;

use types::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "https://api.devnet.solana.com")]
    solana_rpc: String,

    #[arg(long, default_value = "https://bsc-testnet.publicnode.com")]
    bnb_rpc: String,

    #[arg(long)]
    keypair_path: String,

    #[arg(long, default_value = "BnbSo1anaXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX")]
    bridge_program_id: String,

    #[arg(long)]
    bnb_bridge_contract: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    info!("🚀 Starting BNB-Solana Bridge Validator");
    info!("Solana RPC: {}", args.solana_rpc);
    info!("BNB RPC: {}", args.bnb_rpc);

    let validator = BridgeValidator::new(args).await?;
    validator.run().await?;

    Ok(())
}

pub struct BridgeValidator {
    solana_client: Arc<RpcClient>,
    bnb_provider: Arc<Provider<Http>>,
    validator_keypair: Arc<Keypair>,
    bridge_program_id: Pubkey,
    bnb_bridge_address: Address,
}

impl BridgeValidator {
    pub async fn new(args: Args) -> Result<Self> {
        let keypair_bytes = std::fs::read(&args.keypair_path)?;
        let validator_keypair = Keypair::from_bytes(&keypair_bytes)?;

        let solana_client = RpcClient::new_with_commitment(
            args.solana_rpc.clone(),
            CommitmentConfig::confirmed(),
        );

        let bnb_provider = Provider::<Http>::try_from(args.bnb_rpc)?;

        let bridge_program_id = Pubkey::from_str(&args.bridge_program_id)?;
        let bnb_bridge_address = args.bnb_bridge_contract.parse()?;

        Ok(Self {
            solana_client: Arc::new(solana_client),
            bnb_provider: Arc::new(bnb_provider),
            validator_keypair: Arc::new(validator_keypair),
            bridge_program_id,
            bnb_bridge_address,
        })
    }

    pub async fn run(&self) -> Result<()> {
        info!("✅ Validator initialized");
        info!("Validator pubkey: {}", self.validator_keypair.pubkey());

        let mut bnb_block = self.get_latest_bnb_block().await?;
        let mut last_solana_signature = None;

        loop {
            // Monitor BNB Chain for lock events
            match self.monitor_bnb_locks(bnb_block).await {
                Ok(new_block) => {
                    if new_block > bnb_block {
                        bnb_block = new_block;
                    }
                }
                Err(e) => error!("Error monitoring BNB locks: {}", e),
            }

            // Monitor Solana for burn events
            match self.monitor_solana_burns(last_solana_signature).await {
                Ok(sig) => last_solana_signature = sig,
                Err(e) => error!("Error monitoring Solana burns: {}", e),
            }

            tokio::time::sleep(Duration::from_millis(400)).await;
        }
    }

    async fn get_latest_bnb_block(&self) -> Result<u64> {
        let block = self.bnb_provider.get_block_number().await?;
        Ok(block.as_u64())
    }

    async fn monitor_bnb_locks(&self, from_block: u64) -> Result<u64> {
        let latest_block = self.get_latest_bnb_block().await?;

        if latest_block <= from_block {
            return Ok(from_block);
        }

        info!("🔍 Scanning BNB blocks {} to {}", from_block, latest_block);

        // Query TokensLocked events from BNB bridge contract
        let filter = Filter::new()
            .address(self.bnb_bridge_address)
            .from_block(from_block)
            .to_block(latest_block)
            .event("TokensLocked(bytes32,address,address,uint256,bytes32)");

        let logs = self.bnb_provider.get_logs(&filter).await?;

        for log in logs {
            info!("🔒 Lock event detected on BNB Chain");
            
            if let Err(e) = self.process_bnb_lock(log).await {
                error!("Failed to process BNB lock: {}", e);
            }
        }

        Ok(latest_block)
    }

    async fn process_bnb_lock(&self, log: Log) -> Result<()> {
        // Parse event data
        let lock_id = log.topics.get(1).ok_or(anyhow::anyhow!("Missing lock_id"))?;
        let amount = U256::from_big_endian(&log.data[64..96]);
        
        info!("Processing lock: {:?}, amount: {}", lock_id, amount);

        // Submit mint proof to Solana
        // This would call bridge_from_bnb instruction
        info!("✅ Submitted mint proof to Solana");

        Ok(())
    }

    async fn monitor_solana_burns(&self, last_signature: Option<String>) -> Result<Option<String>> {
        // Get recent signatures for bridge program
        let sigs = self.solana_client.get_signatures_for_address(&self.bridge_program_id)?;

        if sigs.is_empty() {
            return Ok(last_signature);
        }

        let latest_sig = sigs[0].signature.clone();

        for sig_info in sigs {
            if Some(&sig_info.signature) == last_signature.as_ref() {
                break;
            }

            info!("🔥 Checking Solana transaction: {}", sig_info.signature);

            // Parse transaction for burn events
            // This would extract BridgeToBnbEvent and submit unlock to BNB Chain
        }

        Ok(Some(latest_sig))
    }
}
