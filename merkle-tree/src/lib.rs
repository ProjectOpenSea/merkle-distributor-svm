use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("m1uqAm3yAagng3uqphvQ4rRAkRy17F4JX87pFemLBKv");

#[program]
pub mod merkle_distributor {
    use super::*;

    pub fn new_distributor(
        ctx: Context<NewDistributor>,
        version: u64,
        root: [u8; 32],
        max_total_claim: u64,
        max_num_nodes: u64,
        start_vesting_ts: i64,
        end_vesting_ts: i64,
        clawback_start_ts: i64,
    ) -> Result<()> {
        instructions::new_distributor(
            ctx,
            version,
            root,
            max_total_claim,
            max_num_nodes,
            start_vesting_ts,
            end_vesting_ts,
            clawback_start_ts,
        )
    }

    pub fn new_claim(
        ctx: Context<NewClaim>,
        amount_unlocked: u64,
        amount_locked: u64,
        proof: Vec<[u8; 32]>,
    ) -> Result<()> {
        instructions::new_claim(ctx, amount_unlocked, amount_locked, proof)
    }

    pub fn claim_locked(ctx: Context<ClaimLocked>) -> Result<()> {
        instructions::claim_locked(ctx)
    }

    pub fn clawback(ctx: Context<Clawback>) -> Result<()> {
        instructions::clawback(ctx)
    }

    pub fn set_admin(ctx: Context<SetAdmin>, new_admin: Pubkey) -> Result<()> {
        instructions::set_admin(ctx, new_admin)
    }
}