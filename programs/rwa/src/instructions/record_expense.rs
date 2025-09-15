use anchor_lang::prelude::*;

use crate::PropertyAccount;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct RecordExpense<'info> {

    pub manager: Signer<'info>,

    #[account(
        mut,
        has_one = manager,
    )]
    pub property_account: Account<'info, PropertyAccount>,

}

impl <'info> RecordExpense <'info> {

    pub fn record_expense(&mut self, amount: u64) -> Result<()>{

        require!(
            self.property_account.available_to_claim >= amount,
            ErrorCode::InsufficientFundsForExpense
        );
        self.property_account.available_to_claim = self.property_account.available_to_claim.checked_sub(amount).unwrap();

        Ok(())
    }
    
}