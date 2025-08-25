use anchor_lang::prelude::*;

#[account]
pub struct InvestorAccount{
    pub last_claimed: u64,
    pub bump: u8,
}