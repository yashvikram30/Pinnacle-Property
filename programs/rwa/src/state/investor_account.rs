use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct InvestorAccount{
    pub last_claimed: u64,
    pub bump: u8,
}