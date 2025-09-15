use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{PropertyAccount, PROPERTY_SEED, VAULT_SEED};

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct InitializeProperty<'info>{

    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        init,
        payer = maker,
        space = 8 + PropertyAccount::INIT_SPACE,
        seeds = [PROPERTY_SEED, seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub property_account : Account<'info, PropertyAccount>,

    pub usdc_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = maker,
        token::mint = usdc_mint,
        token::authority = property_account, 
        token::token_program = token_program,
        seeds = [VAULT_SEED, property_account.key().as_ref()],
        bump
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,

}

impl <'info> InitializeProperty <'info> {

    pub fn init(&mut self, manager: Pubkey, _seed: u64, token_mint: Pubkey, bumps: &InitializePropertyBumps) -> Result<()> {
        self.property_account.set_inner(PropertyAccount { 
            owner: self.maker.key(),
            manager, 
            token_mint , 
            available_to_claim: 0, 
            current_period: 0, 
            bump: bumps.property_account,
            vault_bump: bumps.vault
        });

        Ok(())
    }
}

