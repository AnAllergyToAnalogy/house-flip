use anchor_lang::prelude::*;

use anchor_spl::{
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::{Game};
use crate::internal::{_take_token, _give_token};
use crate::error::*;

#[derive(Accounts)]
#[instruction()]
pub struct SetToken <'info>{
    #[account(
        mut,
        seeds = [
            b"game".as_ref()
        ],
        bump,
    )]
    pub game: Account<'info, Game>,


    #[account(mut, address = game.creator)]
    pub signer: Signer<'info>,    
    pub system_program: Program<'info, System>,


    /// Token Accounts
    pub token_program: Interface<'info, TokenInterface>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub game_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub creator_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

}

pub fn handler(ctx: Context<SetToken>, token_account: Pubkey, token_program: Pubkey ) -> Result<()> {
    let game     = &mut ctx.accounts.game;
    let signer   = &mut ctx.accounts.signer;
    
    let mint                    = &mut ctx.accounts.mint;    
    let game_token_account      = &mut ctx.accounts.game_token_account;
    let creator_token_account   = &mut ctx.accounts.creator_token_account;
    let token_program_account   = ctx.accounts.token_program.clone();
    let token_program_account2  = ctx.accounts.token_program.clone();
    

    if game.token_set {
        return err!(Errors::TokenSet);
    }

    game.token_account = token_account;
    game.token_program = token_program;
    game.token_set = true;

    // Take 1 Token and give it back to make sure accounts are all good
    _take_token(
        token_program_account,
        game_token_account,
        creator_token_account,

        mint,
        game,
        signer, 

        1
    )?;
    _give_token(
        token_program_account2,
        game_token_account,
        creator_token_account,

        mint,
        game,
        ctx.bumps.game,
        1
    )?;


    Ok(())
}
