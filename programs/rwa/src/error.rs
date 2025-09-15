use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient Funds for Expense")]
    InsufficientFundsForExpense,
    #[msg("Error")]
    InvalidTokenMint,
    #[msg("error")]
    AlreadyClaimed,
    #[msg("error")]
    NoPayoutAvailable,

}
