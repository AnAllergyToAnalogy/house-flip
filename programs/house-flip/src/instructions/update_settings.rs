use anchor_lang::prelude::*;

use crate::state::{Game, Settings};
use crate::error::*;
use crate::events::{SettingsUpdated};
use crate::constants::{HUNDRED};


#[derive(Accounts)]
#[instruction()]
pub struct UpdateSettings <'info>{
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
            b"settings".as_ref()
        ],
        bump,
    )]
    pub settings: Account<'info, Settings>,


    #[account(mut, address = game.creator)]
    pub signer: Signer<'info>,    
    pub system_program: Program<'info, System>,

}

pub fn handler(ctx: Context<UpdateSettings>, time_max: i64, time_increment: i64, play_cost: u64 ) -> Result<()> {
    let settings = &mut ctx.accounts.settings;
    
    if time_max <= 0 {
        return err!(Errors::TimeMax);
    }
    if time_increment <= 0 {
        return err!(Errors::TimeIncrement);
    }
    if play_cost % HUNDRED != 0 {
        return err!(Errors::CostPlay);
    }
    
    settings.time_max       = time_max;
    settings.time_increment = time_increment;
    settings.play_cost      = play_cost;

    emit!(SettingsUpdated{
        time_max:       time_max,
        time_increment: time_increment,
        play_cost:      play_cost,
    });

    Ok(())
}
