use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
declare_id!("AdAer5ihhyVQAgxZBTZpgLZG9kFBcFrJ9PEH42AtyFtT");

#[error_code]
pub enum MyError{
    #[msg("Sol Receiver MissMatch")]
    SRMM,
}

#[program]
pub mod tut2 {
    use super::*;

    pub fn init_pda(context: Context<InitPda>, sol_receiver: Pubkey) -> Result<()> {
        let pda = &mut context.accounts.pda;
        let owner = context.accounts.owner.key();
        pda.owner = owner;
        pda.sol_receiver = sol_receiver;
        
        Ok(())
    }

    pub fn change_sol_receiver(context:Context<UpdatePda>, sol_receiver: Pubkey) -> Result<()>{
        let pda = &mut context.accounts.pda;
        pda.sol_receiver = sol_receiver;
        
        Ok(())
    }
    
    pub fn set_price(context: Context<UpdatePda>, price: u64) -> Result<()>{
        let pda = &mut context.accounts.pda;
        pda.price = price as f64 / 1000_000_000f64;

        Ok(())
    }
    
    pub fn buy_token(context: Context<ABuyToken>, amount: u64) -> Result<()> {
        let pda = &mut context.accounts.pda;
        let pda_ata = context.accounts.pda_ata.to_account_info();
        let buyer = context.accounts.buyer.to_account_info();
        let buyer_ata = context.accounts.buyer_ata.to_account_info();
        let token_program = context.accounts.token_program.to_account_info();
        let sol_collector = context.accounts.sol_collector.to_account_info();
        let system_program = context.accounts.system_program.to_account_info();

        if pda.sol_receiver != sol_collector.key(){
            // msg!("MissMatch the sol Receiver");
            // return Ok(())
            return anchor_lang::err!(MyError::SRMM);
        }

        let (_pda, bump) = Pubkey::find_program_address(&[b"_seed"], context.program_id);

        //? Taking Sol:
        let amount_float = (amount as f64) / 10_000 as f64;
        let sol_price = (1000_000_000 as f64 * amount_float) * pda.price;
        pda.sold_amount += amount;

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &buyer.key(),
            &sol_collector.key(),
            sol_price as u64,
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
            authority: pda.to_account_info(),
        };

        token::transfer(
            CpiContext::new_with_signer(token_program, cpi_accounts, &[&[b"_seed", &[bump]]]),
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
        mut,
        seeds = [b"_seed"],
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

#[derive(Accounts)]
pub struct InitPda<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        seeds=[b"_seed"],
        bump,
        payer= owner,
        space = 8 + std::mem::size_of::<PdaInfo>(), 
    )]
    pub pda: Account<'info, PdaInfo>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct UpdatePda<'info>{
    #[account(
        address = pda.owner
    )]
    pub owner: Signer<'info>,
    
    #[account(
        mut,
        seeds=[b"_seed"],
        bump,
    )]
    pub pda: Account<'info, PdaInfo>,
}


#[account]
pub struct PdaInfo{
    owner: Pubkey,
    sol_receiver: Pubkey,
    price: f64,
    sold_amount: u64,
}
