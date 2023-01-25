use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, MintTo};

declare_id!("5Bh7cdEJWWkrJ45d1rsJmo25wwFfMsjQY7j5nHvn9Ztb");

#[program]
pub mod tut2 {
    use super::*;

    pub fn mint_token1(context: Context<ATokenMnt>, amount: u64) -> Result<()> {
        let minter = context.accounts.minter.to_account_info();
        let mint = context.accounts.mint.to_account_info();
        let ata = context.accounts.ata.to_account_info();
        let token_program = context.accounts.token_program.to_account_info();

        let cpi_accounts =  MintTo{
            mint: mint,
            to: ata,
            authority: minter,
        };

        let cpi_context = CpiContext::new(token_program, cpi_accounts);

        token::mint_to(cpi_context, amount)?;

        Ok(())
    }

    pub fn token_transfer1(context: Context<ATokenTransfer>, amount: u64) -> Result<()> {
        let sender = context.accounts.sender.to_account_info();
        let sender_ata = context.accounts.sender_ata.to_account_info();
        let receiver_ata = context.accounts.receiver_ata.to_account_info();
        let token_program = context.accounts.token_token.to_account_info();

        let cpi_accounts = Transfer {
            from: sender_ata,
            to: receiver_ata,
            authority: sender,
        };

        let cpi_ctx = CpiContext::new(token_program, cpi_accounts);
        token::transfer(cpi_ctx, amount).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ATokenMnt<'info> {
    #[account()]
    pub minter: Signer<'info>,

    #[account(
        mut,
        mint::authority = minter,
    )]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ATokenTransfer<'info> {
    #[account()]
    pub sender: Signer<'info>,

    #[account(
        // mint::authority = sender
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        token::authority = sender,
        token::mint = mint,
    )]
    pub sender_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint,
    )]
    pub receiver_ata: Account<'info, TokenAccount>,

    pub token_token: Program<'info, Token>,
}
