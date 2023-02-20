use anchor_lang::prelude::*;

declare_id!("3MGnQXb5wnxnxXLThkaN5Ri3CpZq4PmTQQZujS1VUisU");

// union Res{
//     pass(value),
//     fail(errinfo)
// }

#[program]
pub mod tut2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let account = &mut ctx.accounts.account;
        account.res = 0;

        msg!("Initialization ....");
        Ok(())
    }

    pub fn add(ctx: Context<Add>, a: i32, b: i32) -> Result<()> {
        let account = &mut ctx.accounts.account;
        let tmp = a + b;
        account.res = tmp;

        msg!("Addition ....");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        signer,
        payer = user,
        space = 8 + Answer::MAX_SIZE,
    )]
    pub account: Account<'info, Answer>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut, signer)]
    pub account: Account<'info, Answer>,
}

#[account]
pub struct Answer {
    pub res: i32,
}
impl Answer {
    pub const MAX_SIZE: usize = std::mem::size_of::<Answer>();
}
