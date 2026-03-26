// take token
// flip the thing
// increment the player account of the previous holder

use anchor_lang::prelude::*;

use anchor_spl::{
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::{Game, Player};
use crate::error::*;
use crate::internal::{_take_token};
use crate::events::{Flipped};
use crate::constants::{PRIZE, PROCESS, HOUSE, HUNDRED, EARN};


#[derive(Accounts)]
// #[instruction(index: u8)]
pub struct Flip <'info>{

    #[account(
        mut,
        seeds = [
            b"game".as_ref()
        ],
        bump,
    )]
    pub game: Box< Account<'info, Game>>, 
    
    #[account(
        init_if_needed,
        seeds = [
            b"player".as_ref(),
            signer.key().as_ref()
        ],
        bump,
        payer = signer,
        space = Player::INIT_SPACE + 16
    )]
    pub flipper: Account<'info, Player>,

    #[account(mut)]
    pub flippee: Account<'info, Player>,


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
    pub player_token_account: InterfaceAccount<'info, TokenAccount>,


}

pub fn handler(ctx: Context<Flip>, index: u8) -> Result<()> {

    let clock: Clock = Clock::get()?;

    let signer  = &mut ctx.accounts.signer;
    let game    = &mut ctx.accounts.game;
    let flipper = &mut ctx.accounts.flipper;
    let flippee = &mut ctx.accounts.flippee;

    let mint                    = &mut ctx.accounts.mint;    
    let game_token_account      = &mut ctx.accounts.game_token_account;
    let player_token_account    = &mut ctx.accounts.player_token_account;
    let token_program           = ctx.accounts.token_program.clone();

    let _index = index as usize; 

    // Check if game in progress, increment accordingly
    if game.end_time != 0 {
        if clock.unix_timestamp > game.end_time {
            // fail
            return err!(Errors::TimeExpired);
        }else{
            // increment time
            game.end_time += game.time_increment;
            if game.end_time > clock.unix_timestamp + game.time_max {
                game.end_time = clock.unix_timestamp + game.time_max;
            }
        }
    }else{
        // Set initial time
        game.end_time = clock.unix_timestamp + game.time_max;
    }


    // Dont need to check winner, time implies everything


    // Calculate cost
    let level = game.levels[_index];
    let player = game.players[_index];

    // Make sure its the right flippee
    if flippee.key() != player {
        return err!(Errors::PlayerKey);
    }


    if game.prize == 0 {
        let initial = game.play_cost * 2;
        // add initial prize from game funds 
        if game.funds >= initial {
            game.prize += initial;
            game.funds -= initial;
        } else{
            game.prize += game.funds;
            game.funds = 0;
        }
    }


    let cost = u64::pow(2,level as u32) * game.play_cost;
    let cost_prize   = cost * PRIZE / HUNDRED;
    let cost_house   = cost * HOUSE / HUNDRED;
    let cost_process = cost * PROCESS / HUNDRED;
    let cost_earn    = cost * EARN / HUNDRED;

    if level == 0 {
        // First flip, special
        game.prize += cost_earn;
    }else{
        // flippee.banked += cost
        flippee.banked += cost_earn;
    }


    game.prize += cost_prize;
    game.funds += cost_house;
    game.process_funds += cost_process;

    game.levels[_index] += 1;
    game.players[_index] = flipper.key();
    game.users[_index] = signer.key();


    emit!(Flipped{
        index: index,
        end_time: game.end_time,
        prize: game.prize,
        funds:          game.funds,

        flippee:        flippee.key(),
        flippee_banked: flippee.banked,
        flipper:        flipper.key(),
        user:           signer.key(),
        process_funds: game.process_funds,

        level: level + 1,
        
    });

    _take_token(
        token_program,
        game_token_account,
        player_token_account,

        mint,
        game,
        signer, 

        cost
    )?;


    Ok(())
}
