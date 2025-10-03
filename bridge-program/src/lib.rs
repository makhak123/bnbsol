use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, MintTo, Token, TokenAccount};

declare_id!("BnbSo1anaXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");

#[program]
pub mod bnb_solana_bridge {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        bnb_chain_id: u64,
        validator_threshold: u8,
    ) -> Result<()> {
        let bridge = &mut ctx.accounts.bridge_state;
        bridge.authority = ctx.accounts.authority.key();
        bridge.bnb_chain_id = bnb_chain_id;
        bridge.validator_threshold = validator_threshold;
        bridge.is_active = true;
        bridge.bump = ctx.bumps.bridge_state;
        bridge.authority_bump = ctx.bumps.bridge_authority;
        bridge.validators = Vec::new();
        bridge.nonce = 0;
        
        msg!("BNB-Solana Bridge initialized with chain_id: {}", bnb_chain_id);
        Ok(())
    }

    pub fn bridge_from_bnb(
        ctx: Context<BridgeFromBnb>,
        amount: u64,
        bnb_tx_hash: [u8; 32],
        token_address: [u8; 20],
        validator_signatures: Vec<[u8; 64]>,
    ) -> Result<()> {
        let bridge = &ctx.accounts.bridge_state;
        
        require!(bridge.is_active, BridgeError::BridgeInactive);
        require!(
            validator_signatures.len() >= bridge.validator_threshold as usize,
            BridgeError::InsufficientSignatures
        );

        // Verify this transaction hasn't been processed
        let processed_key = derive_processed_key(&bnb_tx_hash);
        require!(
            !ctx.accounts.processed_tx.is_processed,
            BridgeError::AlreadyProcessed
        );

        // Mark as processed
        ctx.accounts.processed_tx.is_processed = true;
        ctx.accounts.processed_tx.bnb_tx_hash = bnb_tx_hash;
        ctx.accounts.processed_tx.timestamp = Clock::get()?.unix_timestamp;

        // Mint SPL tokens
        let seeds = &[
            b"bridge-authority",
            &[bridge.authority_bump],
        ];
        let signer = &[&seeds[..]];
        
        let cpi_accounts = MintTo {
            mint: ctx.accounts.spl_token_mint.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.bridge_authority.to_account_info(),
        };
        
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer,
        );
        
        token::mint_to(cpi_ctx, amount)?;

        emit!(BridgeFromBnbEvent {
            amount,
            bnb_tx_hash,
            token_address,
            user: ctx.accounts.user.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        msg!("Bridged {} tokens from BNB Chain", amount);
        Ok(())
    }

    pub fn bridge_to_bnb(
        ctx: Context<BridgeToBnb>,
        amount: u64,
        bnb_recipient: [u8; 20],
    ) -> Result<()> {
        let bridge = &mut ctx.accounts.bridge_state;
        
        require!(bridge.is_active, BridgeError::BridgeInactive);
        require!(amount > 0, BridgeError::InvalidAmount);

        // Burn SPL tokens
        let cpi_accounts = Burn {
            mint: ctx.accounts.spl_token_mint.to_account_info(),
            from: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );
        
        token::burn(cpi_ctx, amount)?;

        // Increment nonce for unique event tracking
        bridge.nonce += 1;

        emit!(BridgeToBnbEvent {
            nonce: bridge.nonce,
            amount,
            bnb_recipient,
            token_mint: ctx.accounts.spl_token_mint.key(),
            user: ctx.accounts.user.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        msg!("Burned {} tokens, unlock on BNB Chain for {:?}", amount, bnb_recipient);
        Ok(())
    }

    pub fn add_validator(
        ctx: Context<UpdateValidators>,
        validator: Pubkey,
    ) -> Result<()> {
        let bridge = &mut ctx.accounts.bridge_state;
        
        require!(
            ctx.accounts.authority.key() == bridge.authority,
            BridgeError::Unauthorized
        );

        require!(
            !bridge.validators.contains(&validator),
            BridgeError::ValidatorExists
        );

        bridge.validators.push(validator);
        
        msg!("Validator added: {}", validator);
        Ok(())
    }

    pub fn remove_validator(
        ctx: Context<UpdateValidators>,
        validator: Pubkey,
    ) -> Result<()> {
        let bridge = &mut ctx.accounts.bridge_state;
        
        require!(
            ctx.accounts.authority.key() == bridge.authority,
            BridgeError::Unauthorized
        );

        bridge.validators.retain(|v| v != &validator);
        
        msg!("Validator removed: {}", validator);
        Ok(())
    }

    pub fn set_bridge_active(
        ctx: Context<UpdateValidators>,
        is_active: bool,
    ) -> Result<()> {
        let bridge = &mut ctx.accounts.bridge_state;
        
        require!(
            ctx.accounts.authority.key() == bridge.authority,
            BridgeError::Unauthorized
        );

        bridge.is_active = is_active;
        
        msg!("Bridge active status set to: {}", is_active);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + BridgeState::INIT_SPACE,
        seeds = [b"bridge-state"],
        bump
    )]
    pub bridge_state: Account<'info, BridgeState>,
    
    /// CHECK: PDA authority for minting
    #[account(
        seeds = [b"bridge-authority"],
        bump
    )]
    pub bridge_authority: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(bnb_tx_hash: [u8; 32])]
pub struct BridgeFromBnb<'info> {
    #[account(
        mut,
        seeds = [b"bridge-state"],
        bump = bridge_state.bump
    )]
    pub bridge_state: Account<'info, BridgeState>,
    
    #[account(
        init,
        payer = user,
        space = 8 + ProcessedTx::INIT_SPACE,
        seeds = [b"processed-tx", &bnb_tx_hash],
        bump
    )]
    pub processed_tx: Account<'info, ProcessedTx>,
    
    #[account(mut)]
    pub spl_token_mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: PDA authority
    #[account(
        seeds = [b"bridge-authority"],
        bump = bridge_state.authority_bump
    )]
    pub bridge_authority: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BridgeToBnb<'info> {
    #[account(
        mut,
        seeds = [b"bridge-state"],
        bump = bridge_state.bump
    )]
    pub bridge_state: Account<'info, BridgeState>,
    
    #[account(mut)]
    pub spl_token_mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UpdateValidators<'info> {
    #[account(
        mut,
        seeds = [b"bridge-state"],
        bump = bridge_state.bump
    )]
    pub bridge_state: Account<'info, BridgeState>,
    
    pub authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct BridgeState {
    pub authority: Pubkey,
    pub bnb_chain_id: u64,
    pub validator_threshold: u8,
    pub is_active: bool,
    pub authority_bump: u8,
    pub bump: u8,
    pub nonce: u64,
    #[max_len(100)]
    pub validators: Vec<Pubkey>,
}

#[account]
#[derive(InitSpace)]
pub struct ProcessedTx {
    pub is_processed: bool,
    pub bnb_tx_hash: [u8; 32],
    pub timestamp: i64,
}

#[event]
pub struct BridgeFromBnbEvent {
    pub amount: u64,
    pub bnb_tx_hash: [u8; 32],
    pub token_address: [u8; 20],
    pub user: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct BridgeToBnbEvent {
    pub nonce: u64,
    pub amount: u64,
    pub bnb_recipient: [u8; 20],
    pub token_mint: Pubkey,
    pub user: Pubkey,
    pub timestamp: i64,
}

#[error_code]
pub enum BridgeError {
    #[msg("Bridge is currently inactive")]
    BridgeInactive,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Invalid BNB transaction proof")]
    InvalidProof,
    #[msg("Insufficient validator signatures")]
    InsufficientSignatures,
    #[msg("Transaction already processed")]
    AlreadyProcessed,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Validator already exists")]
    ValidatorExists,
}

fn derive_processed_key(tx_hash: &[u8; 32]) -> Pubkey {
    Pubkey::find_program_address(
        &[b"processed-tx", tx_hash],
        &crate::ID,
    ).0
}
