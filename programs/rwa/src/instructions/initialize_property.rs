use anchor_lang::prelude::*;
use crate::{constants::PROPERTY_SEED, PropertyAccount};
use anchor_spl::{token_2022::Token2022, token_interface::Mint};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitializeProperty <'info> {

    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        init,
        payer = maker,
        space = 8 + PropertyAccount::INIT_SPACE,
        seeds = [PROPERTY_SEED, &seed.to_le_bytes()],
        bump
    )]
    pub property_account: Account<'info, PropertyAccount>,

    // we will obtain a usdc mint here
    pub usdc_mint : Account<'info, Mint>,

    pub system_program: Program<'info, System>,

}

impl <'info> InitializeProperty <'info> {

    pub fn init_property (&mut self, manager: Pubkey, token_mint: Pubkey, bump: &InitializePropertyBumps) -> Result<()> {

        self.property_account.set_inner(PropertyAccount { 
            owner: self.maker.key(), 
            manager, 
            token_mint, 
            available_to_claim: 0, 
            current_period: 0,
            bump: bump.property_account
        });

        Ok(())
    }
}

