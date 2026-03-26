use anchor_lang::prelude::*;

use anchor_spl::{
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::{Game, Player};
use crate::internal::{_give_token}; 
use crate::error::*;
use crate::events::{Withdrawn};


#[derive(Accounts)]
pub struct Withdraw <'info>{
    #[account(
        seeds = [
            b"game".as_ref()
        ],
        bump,
    )]
    pub game: Account<'info, Game>,

    
    #[account(
        mut,
        seeds = [
            b"player".as_ref(),
            signer.key().as_ref()
        ],
        bump,
    )]
    pub player: Account<'info, Player>,

    #[account(mut)]
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

pub fn handler(ctx: Context<Withdraw>) -> Result<()> {
    let game    = &mut ctx.accounts.game;
    let player  = &mut ctx.accounts.player;

    let mint                    = &mut ctx.accounts.mint;
    let game_token_account      = &mut ctx.accounts.game_token_account;
    let recipient_token_account = &mut ctx.accounts.recipient_token_account;
    let token_program           = ctx.accounts.token_program.clone();

    let amount = player.banked;
    player.banked = 0;

    if amount == 0{
        return err!(Errors::TokenAmount);
    }

    _give_token(
        token_program,
        game_token_account,
        recipient_token_account,

        mint,
        game,
        ctx.bumps.game,
        amount
    )?;
    
    emit!(Withdrawn{
        player: player.key(),
        amount: amount,
    });

    Ok(())
}
