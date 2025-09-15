use anchor_lang::prelude::*;
use anchor_spl::{token_2022::TransferChecked, token_interface::{Mint, TokenAccount, TokenInterface,transfer_checked}};
use crate::{PropertyAccount, VAULT_SEED};

#[derive(Accounts)]
pub struct DepositToken <'info> {

    #[account(mut)]
    pub manager : Signer<'info>,

    #[account(mut)]
    pub manager_usdc_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        has_one = manager,
    )]
    pub property_account: Account<'info, PropertyAccount>,

   #[account(
        mut,
        seeds = [VAULT_SEED, property_account.key().as_ref()],
        bump = property_account.vault_bump
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,

}

impl <'info> DepositToken <'info> {

    pub fn deposit_token(&mut self, amount: u64) -> Result<()>{

        let cpi_accounts = TransferChecked {
            from: self.manager_usdc_account.to_account_info(),
            mint: self.usdc_mint.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.manager.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        
        let ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(ctx, amount, self.usdc_mint.decimals)?;

        self.property_account.available_to_claim = self.property_account.available_to_claim.checked_add(amount).unwrap();
        msg!("Deposited {} to the vault. New available balance: {}", amount, self.property_account.available_to_claim);

        Ok(())

    }
}