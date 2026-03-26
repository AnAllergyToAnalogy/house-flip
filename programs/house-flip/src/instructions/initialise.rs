use anchor_lang::prelude::*;

use crate::state::{Game, Player, Settings};
use crate::constants::{DEFAULT_TIME_MAX, DEFAULT_TIME_INCREMENT, DEFAULT_PLAY_COST};

#[derive(Accounts)]
#[instruction(vrf: Pubkey)]
pub struct Initialise <'info>{
    #[account(
        init,
        seeds = [
            b"game".as_ref()
        ],
        bump,
        payer = signer,
        space = Game::INIT_SPACE + 16
    )]
    pub game: Account<'info, Game>,

    #[account(
        init,
        seeds = [
            b"settings".as_ref()
        ],
        bump,
        payer = signer,
        space = Settings::INIT_SPACE + 16
    )]
    pub settings: Account<'info, Settings>,

    #[account(
        init,
        seeds = [
            b"empty".as_ref()
        ],
        bump,
        payer = signer,
        space = Player::INIT_SPACE + 16
    )]
    pub empty_player: Account<'info, Player>,



    #[account(mut)]
    pub signer: Signer<'info>,    
    pub system_program: Program<'info, System>
}

pub fn handler(ctx: Context<Initialise>, vrf: Pubkey ) -> Result<()> {
    let game     = &mut ctx.accounts.game;
    let settings = &mut ctx.accounts.settings;
    let signer   = &mut ctx.accounts.signer;
    let empty_player   = &mut ctx.accounts.empty_player;

    // Init Settings
    settings.time_max       = DEFAULT_TIME_MAX;
    settings.time_increment = DEFAULT_TIME_INCREMENT;
    settings.play_cost      = DEFAULT_PLAY_COST;

    // Init Game
    if vrf != game.key(){
        game.vrf = vrf;
    }else{
        game.vrf = ephemeral_vrf_sdk::consts::VRF_PROGRAM_IDENTITY;
    }
    game.creator = signer.key();
    
    // Draw Settings
    game.authorised_draw = signer.key();
    game.authorised_draw_window = DEFAULT_TIME_MAX / 2;

    // Init game settings
    game.time_max       = settings.time_max;
    game.time_increment = settings.time_increment;
    game.play_cost      = settings.play_cost;

    game.index_winner = 255;

    // Init Player Accounts to Empty
    for i in 0..16 {
        game.players[i] = empty_player.key();
    }

    Ok(())
}
