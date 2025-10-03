use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockEvent {
    pub lock_id: [u8; 32],
    pub token: [u8; 20],
    pub amount: u64,
    pub solana_recipient: Pubkey,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurnEvent {
    pub nonce: u64,
    pub amount: u64,
    pub bnb_recipient: [u8; 20],
    pub token_mint: Pubkey,
    pub user: Pubkey,
    pub timestamp: i64,
}
