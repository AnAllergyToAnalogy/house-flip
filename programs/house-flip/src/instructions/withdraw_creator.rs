use anchor_lang::prelude::*;

use anchor_spl::{
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::{Game};
use crate::internal::{_give_token}; 
use crate::error::*;
use crate::events::{WithdrawnCreator};


#[derive(Accounts)]
pub struct WithdrawCreator <'info>{
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

    /// Token
    pub token_program: Interface<'info, TokenInterface>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub game_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_token_account: InterfaceAccount<'info, TokenAccount>,
}

pub fn handler(ctx: Context<WithdrawCreator>, amount: u64) -> Result<()> {
    let game = &mut ctx.accounts.game;

    let mint                    = &mut ctx.accounts.mint;
    let game_token_account      = &mut ctx.accounts.game_token_account;
    let recipient_token_account = &mut ctx.accounts.recipient_token_account;
    let token_program           = ctx.accounts.token_program.clone();

    if amount > game.funds || amount == 0{
        return err!(Errors::TokenAmount);
    }

    game.funds -= amount;

    _give_token(
        token_program,
        game_token_account,
        recipient_token_account,

        mint,
        game,
        ctx.bumps.game,
        amount
    )?;
    
    emit!(WithdrawnCreator{
        amount: amount,
        new_funds: game.funds,
    });

    Ok(())
}
