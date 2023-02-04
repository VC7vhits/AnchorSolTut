use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("5Bh7cdEJWWkrJ45d1rsJmo25wwFfMsjQY7j5nHvn9Ztb");

#[program]
pub mod tut2 {
    use super::*;

    pub fn buy_token(context: Context<ABuyToken>, amount: u64) -> Result<()> {
        let pda = context.accounts.pda.to_account_info();
        let pda_ata = context.accounts.pda_ata.to_account_info();
        let buyer = context.accounts.buyer.to_account_info();
        let buyer_ata = context.accounts.buyer_ata.to_account_info();
        let token_program = context.accounts.token_program.to_account_info();
        let sol_collector = context.accounts.sol_collector.to_account_info();
        let system_program = context.accounts.system_program.to_account_info();

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
}

#[derive(Accounts)]
pub struct ABuyToken<'info> {
    ///CHECK:
    #[account(
        seeds = [b"seed"],
        bump,
    )]
    pub pda: AccountInfo<'info>,

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
