use anchor_lang::prelude::*;
use ephemeral_vrf_sdk::anchor::vrf;
use ephemeral_vrf_sdk::instructions::{create_request_randomness_ix, RequestRandomnessParams};
use ephemeral_vrf_sdk::types::SerializableAccountMeta;

use anchor_spl::{
    token_interface::{Mint, TokenAccount, TokenInterface},
};


use crate::state::{Game };
use crate::internal::{_give_token};
use crate::error::*;
use crate::instruction::{DrawResolve};
use crate::events::{DrawRequested};

use crate::constants::{PROGRAM_ID};

#[vrf]
#[derive(Accounts)]
pub struct DrawRequest <'info>{
    #[account(
        mut,
        seeds = [
            b"game".as_ref()
        ],
        bump,
    )]
    pub game: Account<'info, Game>, 

    /// CHECK: Oracle Queue
    #[account(mut, address = ephemeral_vrf_sdk::consts::DEFAULT_QUEUE)]
    pub oracle_queue: AccountInfo<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,    
    pub system_program: Program<'info, System>,

    /// Token Accounts
    pub token_program: Interface<'info, TokenInterface>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub game_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub recipient_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

}


pub fn handler(ctx: Context<DrawRequest>) -> Result<()> {
    
    let clock: Clock = Clock::get()?;

    let signer = &mut ctx.accounts.signer;
    let signer_key = signer.key();
    let game = &mut ctx.accounts.game;
    let game_key = game.key();

    let mint                    = &mut ctx.accounts.mint;    
    let game_token_account      = &mut ctx.accounts.game_token_account;
    let recipient_token_account = &mut ctx.accounts.recipient_token_account;
    let token_program_account   = ctx.accounts.token_program.clone();

    let system_program_key = ctx.accounts.system_program.key();

    let use_vrf = game.vrf == ephemeral_vrf_sdk::consts::VRF_PROGRAM_IDENTITY;

    // Check if game over
    if game.end_time == 0 || clock.unix_timestamp <= game.end_time {
        // Game not over, fail
        return err!(Errors::TimeExpired);
    }else if game.draw_requested {
        return err!(Errors::DrawRequested);
    }

    // Check if allowed to call based on time window
    if clock.unix_timestamp <= game.end_time + game.authorised_draw_window &&
        signer_key != game.authorised_draw {
            //In Authorised Draw window but not allowed signer
            return err!(Errors::SignerKey);
    }


    // Pay Requester
    _give_token(
        token_program_account,
        game_token_account,
        recipient_token_account,

        mint,
        game,
        ctx.bumps.game,
        game.process_funds
    )?;

    game.draw_requested = true;
    game.process_funds = 0;


    emit!(DrawRequested{
        requester: signer.key(),
    });
    
    let client_seed: u8 = 0;
    if use_vrf {
        let ix = create_request_randomness_ix(RequestRandomnessParams {
            payer: signer_key,
            oracle_queue: ctx.accounts.oracle_queue.key(),
            callback_program_id: PROGRAM_ID,
            callback_discriminator: DrawResolve::DISCRIMINATOR.to_vec(),
            caller_seed: [client_seed; 32],

            accounts_metas: Some(vec![
                SerializableAccountMeta { //Game Account
                    pubkey: game_key,
                    is_signer: false,
                    is_writable: true,
                },

                SerializableAccountMeta { //System program
                    pubkey: system_program_key,
                    is_signer: false,
                    is_writable: false,
                },

            ]),
            ..Default::default()
        });
        ctx.accounts.invoke_signed_vrf(
            &ctx.accounts.signer.to_account_info(), 
            &ix)?;
    }

    Ok(())

}
