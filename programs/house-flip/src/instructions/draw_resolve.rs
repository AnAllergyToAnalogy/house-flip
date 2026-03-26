use anchor_lang::prelude::*;

use crate::state::{Game};
use crate::error::*;
use crate::events::{DrawResolved};

#[derive(Accounts)]
pub struct DrawResolve <'info>{
    /// This check ensure that the vrf_program_identity (which is a PDA) is a singer
    /// enforcing the callback is executed by the VRF program trough CPI
    #[account(address = game.vrf)]
    pub vrf_program_identity: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"game".as_ref()
        ],
        bump,
    )]
    pub game: Account<'info, Game>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DrawResolve>,
    randomness: [u8; 32],
) -> Result<()> {
    let game = &mut ctx.accounts.game;

    if !game.draw_requested {
        return err!(Errors::DrawRequested);
    }
    if game.draw_resolved {
        return err!(Errors::DrawResolved);
    }


    let rnd_u64 = ephemeral_vrf_sdk::rnd::random_u64(&randomness); 


    // Work out RNG max
    let mut rng_max = 0;
    for i in 0..16 {
        let level = game.levels[i] as u32;

        if level == 0 {
            rng_max += 1;
        }else{
            rng_max += u64::pow(2,level - 1);
        }
    }

    // RNG RESULT
    let result = rnd_u64 % rng_max;


    let mut index_winner = 0;
    // Work out who won
    let mut rng_win = 0;
    for i in 0..16 {
        let level = game.levels[i] as u32;

        if level == 0 {
            rng_win += 1;
        }else{
            rng_win += u64::pow(2,level - 1);
        }

        if result < rng_win {
            index_winner = i;
            break;
        }
    }

    game.index_winner = index_winner as u8;
    game.winner = game.players[index_winner];
    game.draw_resolved = true;

    game.debug = rnd_u64;



    emit!(DrawResolved{
        index_winner: game.index_winner,
        winner:       game.winner,

    });

    Ok(())
}
