use anchor_lang::prelude::*;
use anchor_lang::system_program::Transfer as SolanaTransfer;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
declare_id!("5Bh7cdEJWWkrJ45d1rsJmo25wwFfMsjQY7j5nHvn9Ztb");

#[program]
pub mod tut2 {
    use super::*;

    pub fn init_pda(context: Context<InitPda>, sol_receiver: Pubkey) -> Result<()> {
        let pda = &mut context.accounts.pda;
        pda.owner = sol_receiver;
        
        Ok(())
    }
    
    pub fn buy_token(context: Context<ABuyToken>, amount: u64) -> Result<()> {
        let pda = context.accounts.pda.to_account_info();
        let pda_ata = context.accounts.pda_ata.to_account_info();
        let buyer = context.accounts.buyer.to_account_info();
        let buyer_ata = context.accounts.buyer_ata.to_account_info();
        let token_program = context.accounts.token_program.to_account_info();
        let sol_collector = context.accounts.sol_collector.to_account_info();
        let system_program = context.accounts.system_program.to_account_info();

        if context.accounts.pda.owner != sol_collector.key(){
            msg!("MissMatch the sol Receiver");
            return Ok(())
        }

        let (_pda, bump) = Pubkey::find_program_address(&[b"seed"], context.program_id);

        //? Taking Sol:
        let amount_float = (amount as f64) / 1_000 as f64;
        let sol_price = (1000_000_000 as f64 * amount_float) as u64 / 1000;

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &buyer.key(),
            &sol_collector.key(),
            sol_price,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                buyer.to_account_info(),
                sol_collector.to_account_info(),
                system_program.to_account_info(),
            ],
        )?;

        //? Sending the Token:
        let cpi_accounts = Transfer {
            from: pda_ata,
            to: buyer_ata,
            authority: pda,
        };

        token::transfer(
            CpiContext::new_with_signer(token_program, cpi_accounts, &[&[b"seed", &[bump]]]),
            amount,
        )
        .unwrap();

        Ok(())
    }

    // pub fn claim_from_pda(context: Context<ClaimFromPda>) -> Result<()> {
    //     let pda = context.accounts.pda.to_account_info();
    //     let receiver = context.accounts.receiver.to_account_info();
    //     let system_program = context.accounts.system_program.to_account_info();
    //     let lamp = pda.lamports();

    //     let right_sol_receiver = context.accounts.pda.owner;

    //     if receiver.key() != right_sol_receiver{
    //         msg!("Unknown Sol Reciever");
    //         return Ok(());
    //     }

    //     let (_, bump) = Pubkey::find_program_address(&[b"seed"], &context.program_id);

    //     // let cpi_accounts = anchor_lang::system_program::Transfer {
    //     let cpi_accounts = SolanaTransfer {
    //         from: pda,
    //         to: receiver,
    //     };

    //     anchor_lang::system_program::transfer(
    //         CpiContext::new_with_signer(system_program, cpi_accounts, &[&[b"seed", &[bump]]]),
    //         // lamp,
    //         1000,
    //     )?;

    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct ABuyToken<'info> {
    ///CHECK:
    #[account(
        mut,
        seeds = [b"seed"],
        bump,
    )]
    pub pda: Account<'info, PdaInfo>,

    #[account()]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        token::authority = pda,
        token::mint = mint,
    )]
    pub pda_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    ///CHECK:
    #[account(mut)]
    pub sol_collector: AccountInfo<'info>,

    #[account(
        mut,
        token::mint = mint,
    )]
    pub buyer_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

// #[derive(Accounts)]
// pub struct ClaimFromPda<'info> {
//     ///CHECK:
//     #[account(
//         mut,
//         seeds = [b"seed"],
//         bump,
//     )]
//     pda: Account<'info, PdaInfo>,

//     ///CHECK:
//     #[account(mut)]
//     receiver: AccountInfo<'info>,

//     system_program: Program<'info, System>,
// }

#[derive(Accounts)]
pub struct InitPda<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        seeds=[b"seed"],
        bump,
        payer= owner,
        space = 8 + std::mem::size_of::<PdaInfo>(), 
    )]
    pub pda: Account<'info, PdaInfo>,
    system_program: Program<'info, System>
}

#[account]
pub struct PdaInfo{
    owner: Pubkey,
}
