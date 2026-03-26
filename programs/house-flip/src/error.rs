use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("Token:Set")]
    TokenSet,
    #[msg("Token:Account")]
    TokenAccount,
    #[msg("Token:Program")]
    TokenProgram,
    #[msg("Token:Amount")]
    TokenAmount,

    #[msg("Signer:Key")]
    SignerKey,
    #[msg("Player:Key")]
    PlayerKey,


    #[msg("Time:Max")]
    TimeMax,
    #[msg("Time:Increment")]
    TimeIncrement,
    #[msg("Time:Expired")]
    TimeExpired,


    #[msg("Draw:Requested")]
    DrawRequested,
    #[msg("Draw:Resolved")]
    DrawResolved,


    #[msg("Cost:Play")]
    CostPlay,


}
