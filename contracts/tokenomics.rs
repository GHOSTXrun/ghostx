// WRAITH Protocol — Tokenomics Module
// Handles: burn mechanics, reward distribution, supply tracking

use anchor_lang::prelude::*;

pub const TOTAL_SUPPLY:        u64 = 1_000_000_000_000_000_000; // 1B with 9 decimals
pub const AI_FUND_BPS:         u64 = 500;   // 5%
pub const LIQUIDITY_BPS:       u64 = 9_500; // 95%
pub const BURN_SHARE_BPS:      u64 = 5_000; // 50% of revenue
pub const REWARD_SHARE_BPS:    u64 = 5_000; // 50% of revenue
pub const BASIS_POINTS:        u64 = 10_000;

pub struct TokenomicsEngine;

impl TokenomicsEngine {
    /// Split revenue into burn and reward amounts
    pub fn split_revenue(amount: u64) -> (u64, u64) {
        let burn   = amount * BURN_SHARE_BPS   / BASIS_POINTS;
        let reward = amount * REWARD_SHARE_BPS / BASIS_POINTS;
        (burn, reward)
    }

    /// Calculate governance weight from token balance
    pub fn governance_weight(balance: u64) -> u64 {
        // Square root weighting to prevent whale dominance
        (balance as f64).sqrt() as u64
    }

    /// Determine residue level from balance
    pub fn residue_level(balance: u64) -> &'static str {
        let normalized = balance / 1_000_000_000; // remove decimals
        match normalized {
            0..=999         => "None",
            1_000..=9_999   => "Residue",
            10_000..=99_999 => "Condensed",
            100_000..=999_999 => "Manifested",
            _               => "Entangled",
        }
    }
}
