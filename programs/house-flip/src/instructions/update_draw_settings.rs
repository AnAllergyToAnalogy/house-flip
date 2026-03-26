use anchor_lang::prelude::*;

use crate::state::{Game};
use crate::error::*;
use crate::events::{DrawSettingsUpdated};
use crate::constants::{DEFAULT_TIME_MAX};

#[derive(Accounts)]
#[instruction()]
pub struct UpdateDrawSettings <'info>{
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

}

pub fn handler(ctx: Context<UpdateDrawSettings>, authorised_draw: Pubkey, authorised_draw_window: i64 ) -> Result<()> {
    let game     = &mut ctx.accounts.game;

    if authorised_draw_window < 0 || authorised_draw_window > DEFAULT_TIME_MAX / 2 {
        return err!(Errors::TimeMax);
    }

    game.authorised_draw        = authorised_draw;
    game.authorised_draw_window = authorised_draw_window;
    
    emit!(DrawSettingsUpdated{
        authorised_draw:        authorised_draw,
        authorised_draw_window: authorised_draw_window,
    });

    Ok(())
}