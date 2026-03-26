// After winer picked, credit winner account, reset all the vars, read settings


use anchor_lang::prelude::*;

use crate::state::{Game, Settings, Player};
use crate::events::{GameReset};
use crate::error::*;
use crate::constants::{ZERO};


#[derive(Accounts)]
pub struct Reset <'info>{

    #[account(
        mut,
        seeds = [
            b"game".as_ref()
        ],
        bump,
    )]
    pub game: Account<'info, Game>, 

    #[account(
        seeds = [
            b"settings".as_ref()
        ],
        bump,
    )]
    pub settings: Account<'info, Settings>, 

    #[account(
        seeds = [
            b"empty".as_ref()
        ],
        bump,
    )]
    pub empty_player: Account<'info, Player>, 

    #[account(
        mut,
        address = game.winner,
    )]
    pub winner: Account<'info, Player>,

    #[account(mut)]
    pub signer: Signer<'info>,    
    pub system_program: Program<'info, System>,

}

pub fn handler(ctx: Context<Reset>) -> Result<()> {
    
    let game        = &mut ctx.accounts.game;
    let settings    = &mut ctx.accounts.settings;
    let empty_player      = &mut ctx.accounts.empty_player;
    let winner      = &mut ctx.accounts.winner;

    
    if !game.draw_resolved {
        return err!(Errors::DrawResolved);
    }

    // Credit Winner if there was one, otherwise refund game 
    if winner.key() == empty_player.key() {
        game.funds += game.prize;
    }else{
        winner.banked += game.prize;
    }



    // Update Settings
    game.time_max       = settings.time_max;
    game.time_increment = settings.time_increment;
    game.play_cost      = settings.play_cost;

    // Reset Vars
    for i in 0..16 {
        game.levels[i] = 0;
        game.players[i] = empty_player.key();
        game.users[i] = ZERO;
    }

    game.prize = 0;
    game.process_funds = 0;
    game.end_time = 0;
    game.round += 1;

    game.draw_requested = false;
    game.draw_resolved = false;

    game.index_winner = 255;
    game.winner = ZERO;



    emit!(GameReset{
        round:          game.round,
        winner:         winner.key(),
        winner_banked:  winner.banked,

        time_max:       game.time_max,
        time_increment: game.time_increment,
        play_cost:      game.play_cost,

        funds:          game.funds,
    });

    Ok(())
}
