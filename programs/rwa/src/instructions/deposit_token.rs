use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DepositToken <'info> {

    #[account(mut)]
    pub manager : Signer<'info>,

    // when we define a token account, we do not need the seeds
    // #[account(
    //     init,
    //     payer = manager,
    //     token::mint = usdc_mint
    // )]

}