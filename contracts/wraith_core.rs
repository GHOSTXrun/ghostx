// WRAITH Protocol — Core Contract
// Network: Solana | Framework: Anchor
// Phase: 01 — RESIDUE

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};

declare_id!("WRAiTHxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");

#[program]
pub mod wraith_core {
    use super::*;

    /// Initialize the WRAITH protocol state
    pub fn initialize(ctx: Context<Initialize>, params: InitParams) -> Result<()> {
        let state = &mut ctx.accounts.protocol_state;
        state.authority      = ctx.accounts.authority.key();
        state.total_supply   = params.total_supply;
        state.phase          = Phase::Residue;
        state.residue_count  = 0;
        state.burn_total     = 0;
        state.reward_pool    = 0;
        state.bump           = *ctx.bumps.get("protocol_state").unwrap();

        emit!(ProtocolInitialized {
            timestamp: Clock::get()?.unix_timestamp,
            phase: Phase::Residue,
        });

        Ok(())
    }

    /// Register a new residue holder
    pub fn register_residue(ctx: Context<RegisterResidue>) -> Result<()> {
        let state  = &mut ctx.accounts.protocol_state;
        let holder = &mut ctx.accounts.holder_account;

        holder.wallet          = ctx.accounts.user.key();
        holder.registered_at   = Clock::get()?.unix_timestamp;
        holder.residue_level   = ResidueLevel::Residue;
        holder.bump            = *ctx.bumps.get("holder_account").unwrap();

        state.residue_count   += 1;

        // Check phase advancement
        state.check_phase_advance();

        emit!(ResidueRegistered {
            wallet:    holder.wallet,
            count:     state.residue_count,
            timestamp: holder.registered_at,
        });

        Ok(())
    }

    /// Distribute AI revenue: 50% burn, 50% rewards
    pub fn distribute_revenue(ctx: Context<DistributeRevenue>, amount: u64) -> Result<()> {
        require!(amount > 0, WraithError::ZeroAmount);

        let burn_amount   = amount / 2;
        let reward_amount = amount - burn_amount;

        // Burn half
        token::burn(
            ctx.accounts.burn_context(),
            burn_amount,
        )?;

        // Add rest to reward pool
        let state       = &mut ctx.accounts.protocol_state;
        state.burn_total  += burn_amount;
        state.reward_pool += reward_amount;

        emit!(RevenueDistributed {
            total:   amount,
            burned:  burn_amount,
            rewards: reward_amount,
        });

        Ok(())
    }
}

// ── State ──────────────────────────────────────────────

#[account]
pub struct ProtocolState {
    pub authority:     Pubkey,
    pub total_supply:  u64,
    pub phase:         Phase,
    pub residue_count: u64,
    pub burn_total:    u64,
    pub reward_pool:   u64,
    pub bump:          u8,
}

impl ProtocolState {
    pub const LEN: usize = 8 + 32 + 8 + 1 + 8 + 8 + 8 + 1;

    fn check_phase_advance(&mut self) {
        self.phase = match self.residue_count {
            0..=99    => Phase::Residue,
            100..=499 => Phase::Condensation,
            500..=999 => Phase::Manifestation,
            _         => Phase::Entanglement,
        };
    }
}

#[account]
pub struct HolderAccount {
    pub wallet:        Pubkey,
    pub registered_at: i64,
    pub residue_level: ResidueLevel,
    pub bump:          u8,
}

impl HolderAccount {
    pub const LEN: usize = 8 + 32 + 8 + 1 + 1;
}

// ── Enums ──────────────────────────────────────────────

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum Phase {
    Residue,
    Condensation,
    Manifestation,
    Entanglement,
    Permanence,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ResidueLevel {
    Residue,      // 1,000+
    Condensed,    // 10,000+
    Manifested,   // 100,000+
    Entangled,    // 1,000,000+
}

// ── Params ─────────────────────────────────────────────

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitParams {
    pub total_supply: u64,
}

// ── Events ─────────────────────────────────────────────

#[event]
pub struct ProtocolInitialized { pub timestamp: i64, pub phase: Phase }

#[event]
pub struct ResidueRegistered   { pub wallet: Pubkey, pub count: u64, pub timestamp: i64 }

#[event]
pub struct RevenueDistributed  { pub total: u64, pub burned: u64, pub rewards: u64 }

// ── Errors ─────────────────────────────────────────────

#[error_code]
pub enum WraithError {
    #[msg("Amount must be greater than zero")]
    ZeroAmount,
    #[msg("Phase advancement condition not met")]
    PhaseNotReady,
}
