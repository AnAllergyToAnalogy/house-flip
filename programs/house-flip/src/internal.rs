use anchor_lang::prelude::*;

use anchor_spl::{
    token_interface::{transfer_checked, 
        Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::state::{Game};
use crate::error::*;


pub fn _give_token<'info>(
    token_program:           Interface<'info, TokenInterface>, 
    game_token_account:      &mut InterfaceAccount<'info, TokenAccount>,
    recipient_token_account: &mut InterfaceAccount<'info, TokenAccount>,

    mint: &mut InterfaceAccount<'info, Mint>,
    game: &mut Account<'info, Game>, 
    game_bumps: u8,

    amount: u64


) -> Result<()> {

    let seeds = &["game".as_bytes(), &[game_bumps]]; 
    let signer = [&seeds[..]];

    let cpi_signer = game.to_account_info();

    // let game_token_account_key = game_token_account.key();

    if  game_token_account.key() != game.token_account {
        return err!(Errors::TokenAccount);
    }
    if token_program.key() != game.token_program {
        return err!(Errors::TokenProgram);
    }

    let decimals = mint.decimals;

    transfer_checked(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            TransferChecked {

                mint:       mint.to_account_info(),

                from:       game_token_account.to_account_info(),
                to:         recipient_token_account.to_account_info().clone(),

                authority:  cpi_signer,

            },
            &signer,
        ),
        amount, decimals
    )?;


    Ok(())
}



pub fn _take_token<'info>(
    token_program:          Interface<'info, TokenInterface>, 
    game_token_account:     &mut InterfaceAccount<'info, TokenAccount>,
    sender_token_account:   &mut InterfaceAccount<'info, TokenAccount>,

    mint:   &mut InterfaceAccount<'info, Mint>,
    game:   &mut Account<'info, Game>, 
    sender: &mut Signer<'info>,

    amount: u64
) -> Result<()> {

    if  game_token_account.key() != game.token_account {
        return err!(Errors::TokenAccount);
    }
    if token_program.key() != game.token_program {
        return err!(Errors::TokenProgram);
    }

    let decimals = mint.decimals;

    let cpi_ctx = CpiContext::new(
        token_program.to_account_info(),
        TransferChecked {
            mint:       mint.to_account_info(),

            from:       sender_token_account.to_account_info(),
            to:         game_token_account.to_account_info(),

            authority:  sender.to_account_info(),

        }
    );
    transfer_checked(cpi_ctx, amount, decimals)?;

    Ok(())
}