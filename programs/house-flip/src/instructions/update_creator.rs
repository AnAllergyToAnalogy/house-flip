use anchor_lang::prelude::*;

use crate::state::{Game};

#[derive(Accounts)]
pub struct UpdateCreator <'info>{
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
    pub system_program: Program<'info, System>
}

pub fn handler(ctx: Context<UpdateCreator>, new_creator: Pubkey) -> Result<()> {
    let game = &mut ctx.accounts.game;
    
    game.creator = new_creator;

    Ok(())
}
