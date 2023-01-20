use anchor_lang::prelude::*;

declare_id!("5Bh7cdEJWWkrJ45d1rsJmo25wwFfMsjQY7j5nHvn9Ztb");

#[program]
pub mod tut2 {
    use anchor_lang::solana_program::program::{invoke, invoke_signed};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initialized ...");

        Ok(())
    }

    pub fn init_pda_account(ctx: Context<InitPdaAccount>) -> Result<()>{

        Ok(())
    }

    pub fn sol_transfer(ctx: Context<ASolTransfer>, amount: u64) -> Result<()> {
        let sender = ctx.accounts.sender.to_account_info();
        let receiver = ctx.accounts.receiver.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();

        // let ix = system_instruction::transfer(&sender.key(), &receiver.key(), amount);
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &sender.key(),
            &receiver.key(),
            amount,
        );

        invoke(
            &ix,
            &[sender, receiver, system_program], // ).unwrap();
        )?;

        msg!("Sol tranfered : {}", amount as f64 / 1000_000_000.0);

        Ok(())
    }

    pub fn init_account(ctx: Context<InitAccount>) -> Result<()> {
        msg!("Account is initialized ....");
        Ok(())
    }

    pub fn add(ctx: Context<Add>, a: i32, b: i32) -> Result<()> {
        let account = &mut ctx.accounts.account;
        let tmp = a + b;
        account.res = tmp;

        msg!("Addition ....");
        Ok(())
    }

    pub fn add_in_pda(ctx: Context<AddInPda>, a:i32, b:i32) -> Result<()>{
        let account = &mut ctx.accounts.account;
        let tmp = a + b;
        account.res = tmp;

        msg!("Addition ....");
        
        Ok(())
    }



    pub fn airdrop(context: Context<Airdrop>, amount:u64) -> Result<()>{
        let pda  = context.accounts.pda.to_account_info();
        let receiver = context.accounts.receiver.to_account_info();

        let airdrop_seed = "ad".as_bytes();
        let (_pda, bump) = Pubkey::find_program_address(&[airdrop_seed], context.program_id);

        if pda.key() != _pda {
            msg!("Sended Pda MissMatch");
            return Ok(())
        }

        let ix = anchor_lang::solana_program::system_instruction::transfer(&pda.key(), &receiver.key(), amount);
        
        let res = invoke_signed(
            // &anchor_lang::system_program::transfer(ctx, lamports)
            &ix,
            &[
                pda,
                receiver,
            ],
            &[
                &[
                    airdrop_seed,
                    &[bump]
                ],
            ]
        ).unwrap();

        // match res{
        //     Ok(val) =>{msg!("sol transfer Passed");},
        //     Err(_)=> {msg!("sol transfer Failed")}
        // }

        
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct ASolTransfer<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    ///CHECK:
    #[account(mut)]
    pub receiver: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct InitPdaAccount<'info>{
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        // seeds = ["123".as_ref()],
        seeds = [user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + Answer::MAX_SIZE,
    )]
    pub account: Account<'info, Answer>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitAccount<'info> {
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
pub struct AddInPda<'info> {
    #[account()]
    pub user: Signer<'info>,
    
    #[account(
        mut, 
        // seeds = ["123".as_ref()],
        seeds = [user.key().as_ref()],
        bump,    
    )]
    pub account: Account<'info, Answer>,
}


#[derive(Accounts)]
pub struct Airdrop<'info>{
    ///CHECK:
    #[account(mut)]
    pub receiver: AccountInfo<'info>,

    ///CHECK:
    #[account(mut)]
    pub pda: AccountInfo<'info>,
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
