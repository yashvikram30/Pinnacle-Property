pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("BH9H8itSUwNLicRiymWqF6xdQP6QH2zq18mzWCm42bV1");

#[program]
pub mod rwa {
    use super::*;

    
}
