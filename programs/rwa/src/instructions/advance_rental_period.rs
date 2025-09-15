use anchor_lang::prelude::*;

use crate::PropertyAccount;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct AdvanceRentalPeriod<'info> {

    pub manager: Signer<'info>,

    #[account(
        mut,
        has_one = manager,
    )]
    pub property_account: Account<'info, PropertyAccount>,

}

impl <'info> AdvanceRentalPeriod <'info> {

    pub fn advance_rental_period(&mut self) -> Result<()>{

        self.property_account.current_period = self.property_account.current_period.checked_add(1).unwrap();

        Ok(())
    }
    
}