use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] 
pub struct Game {
    pub vrf:        Pubkey,
    pub creator:    Pubkey,

    pub levels:  [u8;16],
    pub players: [Pubkey;16],
    pub users:   [Pubkey;16],
    
    pub prize:          u64,
    pub funds:          u64,
    pub process_funds:  u64,
    pub end_time:       i64,
    pub round:          u32,

    pub draw_requested: bool,
    pub draw_resolved:  bool,


    pub index_winner:   u8,
    pub winner:         Pubkey,

    pub authorised_draw:     Pubkey,
    pub authorised_draw_window: i64,

    // Read each round
    pub time_max:       i64,
    pub time_increment: i64,
    pub play_cost:      u64,

    // Token Settings
    pub token_account:  Pubkey,
    pub token_program:  Pubkey,
    pub token_set:      bool,

    pub debug:  u64,
}