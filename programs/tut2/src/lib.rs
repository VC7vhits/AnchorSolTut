use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// union Res{
//     pass(value),
//     fail(errinfo)
// }

#[program]
pub mod tut2 {
    use anchor_lang::solana_program::{program::invoke, system_instruction, system_program};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        let sender = ctx.accounts.sender.to_account_info();
        let receiver = ctx.accounts.receiver.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();

        // let ix = system_instruction::transfer(&sender.key(), &receiver.key(), amount);
        let ix = system_instruction::transfer(&sender.key(), &receiver.key(), amount);
        invoke(
            &ix,
            &[sender, receiver, system_program], // ).unwrap();
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    ///CHECK:
    #[account(mut)]
    pub receiver: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
