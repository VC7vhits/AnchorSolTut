use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("5Bh7cdEJWWkrJ45d1rsJmo25wwFfMsjQY7j5nHvn9Ztb");

#[program]
pub mod tut2 {
    use super::*;

    pub fn token_airdrop_from_pda(
        context: Context<ATokenAirdropFromPda>,
        amount: u64,
    ) -> Result<()> {
        let pda = context.accounts.pda.to_account_info();
        let pda_ata = context.accounts.pda_ata.to_account_info();
        let receiver_ata = context.accounts.receiver_ata.to_account_info();
        let token_program = context.accounts.token_token.to_account_info();

        let (_pda, bump) = Pubkey::find_program_address(&[b"seed"], context.program_id);

        let cpi_accounts = Transfer {
            from: pda_ata,
            to: receiver_ata,
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
pub struct ATokenAirdropFromPda<'info> {
    ///CHECK:
    #[account(
        seeds = [b"seed"],
        bump,
    )]
    pub pda: AccountInfo<'info>,

    #[account(
        // mint::authority = sender
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        token::authority = pda,
        token::mint = mint,
    )]
    pub pda_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint,
    )]
    pub receiver_ata: Account<'info, TokenAccount>,

    pub token_token: Program<'info, Token>,
}
