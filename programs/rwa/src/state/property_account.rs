use anchor_lang::prelude::*;

#[account]
pub struct PropertyAccount{
    pub owner: Pubkey,
    pub manager: Pubkey,
    pub token_mint: Pubkey,
    pub available_to_claim: u64,
    pub current_period: u64
}