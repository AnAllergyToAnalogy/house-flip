pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod events;
mod internal;



use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("E2KAtVvm2N5otMYLwXXGySP9ZGEf7aRy4BDarajw3eJa");

#[program]
pub mod house_flip {
    use super::*;

    // Admin
    pub fn initialise(ctx: Context<Initialise>, vrf: Pubkey) -> Result<()> {
        initialise::handler(ctx, vrf)
    }
    pub fn set_token(ctx: Context<SetToken>, token_account: Pubkey, token_program: Pubkey) -> Result<()> {
        set_token::handler(ctx, token_account, token_program)
    }
    pub fn update_settings(ctx: Context<UpdateSettings>, time_max: i64, time_increment: i64, play_cost: u64 ) -> Result<()> {
        update_settings::handler(ctx, time_max, time_increment, play_cost)
    }
    pub fn update_draw_settings(ctx: Context<UpdateDrawSettings>, authorised_draw: Pubkey, authorised_draw_window: i64 ) -> Result<()> {
        update_draw_settings::handler(ctx, authorised_draw, authorised_draw_window)
    }
    pub fn update_creator(ctx: Context<UpdateCreator>, new_creator: Pubkey) -> Result<()> {
        update_creator::handler(ctx, new_creator)
    }
    pub fn withdraw_creator(ctx: Context<WithdrawCreator>, amount: u64) -> Result<()> {
        withdraw_creator::handler(ctx, amount)
    }


    // // Gameplay
    pub fn flip(ctx: Context<Flip>, index: u8) -> Result<()> {
        flip::handler(ctx, index)
    }
    pub fn draw_request(ctx: Context<DrawRequest>) -> Result<()> {
        draw_request::handler(ctx)
    }
    pub fn draw_resolve(ctx: Context<DrawResolve>, randomness: [u8; 32]) -> Result<()> {
        draw_resolve::handler(ctx, randomness)
    }
    

    // Housekeeping
    pub fn reset(ctx: Context<Reset>) -> Result<()> {
        reset::handler(ctx)
    }
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        withdraw::handler(ctx)
    }
    
    
}
