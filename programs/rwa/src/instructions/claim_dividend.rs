use anchor_lang::prelude::*;
use anchor_spl::token_2022::TransferChecked;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, transfer_checked};
use crate::{InvestorAccount, PropertyAccount};
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct ClaimDividend<'info>{

    #[account(mut)]
    pub investor: Signer<'info>,

    #[account(mut)]
    pub property_account: Account<'info, PropertyAccount>,

    #[account(
        init_if_needed,
        payer = investor,
        space = 8 + InvestorAccount::INIT_SPACE,
        seeds = [b"investor", investor.key().as_ref(), property_account.key().as_ref()],
        bump
    )]
    pub investor_claim_account: Account<'info, InvestorAccount>,

    #[account(
        constraint = investor_rwa_token_account.mint == property_account.token_mint
        @ ErrorCode::InvalidTokenMint
    )]
    pub investor_rwa_token_account: InterfaceAccount<'info, TokenAccount>,

    pub rwa_token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [b"vault", property_account.key().as_ref()],
        bump
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub usdc_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub investor_usdc_account: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl <'info> ClaimDividend <'info> {

    pub fn claim_dividend(&mut self)-> Result<()>{

        require!(
            self.investor_claim_account.last_claimed < self.property_account.current_period,
            ErrorCode::AlreadyClaimed
        );

        let total_supply = self.rwa_token_mint.supply as u128;
        let investor_balance = self.investor_rwa_token_account.amount as u128;
        let available_to_claim = self.property_account.available_to_claim as u128;

        let payout_amount = investor_balance
        .checked_mul(available_to_claim)
        .unwrap()
        .checked_div(total_supply)
        .unwrap() as u64;

        require!(payout_amount > 0, ErrorCode::NoPayoutAvailable);

        let property_key = self.property_account.key();
        let vault_seeds = &[
            b"vault".as_ref(),
            property_key.as_ref(),
            &[self.property_account.vault_bump],
        ];
        let signer_seeds = &[&vault_seeds[..]];

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.investor_usdc_account.to_account_info(),
            authority: self.property_account.to_account_info(), // The PDA is the authority
            mint: self.usdc_mint.to_account_info(),
        };
        
        let cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        transfer_checked(cpi_context, payout_amount, self.usdc_mint.decimals)?;

        self.property_account.available_to_claim = self.property_account
            .available_to_claim
            .checked_sub(payout_amount)
            .unwrap();

        self.investor_claim_account.last_claimed = self.property_account.current_period;
        self.investor_claim_account.bump = self.investor_claim_account.bump;

        msg!("Investor {} claimed {} tokens.", self.investor.key(), payout_amount);

        Ok(())

       
    }
}
