use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PropertyAccount{
    pub owner: Pubkey,
    pub manager: Pubkey,
    pub token_mint: Pubkey,
    pub available_to_claim: u64,
    pub current_period: u64,
    pub bump: u8,
    pub vault_bump: u8,
}