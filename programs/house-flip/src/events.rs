use anchor_lang::prelude::*;

// UpdateSettings
#[event]
pub struct SettingsUpdated {
    pub time_max:       i64,
    pub time_increment: i64,
    pub play_cost:      u64,
}

// UpdateDrawSettings
#[event]
pub struct DrawSettingsUpdated {
    pub authorised_draw:        Pubkey,
    pub authorised_draw_window: i64,
}

// Flip
#[event]
pub struct Flipped {
    pub index:      u8,
    pub end_time:   i64,
    pub prize:      u64,
    pub funds:      u64,

    pub flippee:    Pubkey,
    pub flippee_banked: u64,
    pub flipper:    Pubkey,
    pub user:       Pubkey,
    
    pub process_funds: u64,

    pub level: u8
}

// DrawRequested
#[event]
pub struct DrawRequested {
    pub requester: Pubkey,
}

// DrawResolved
#[event]
pub struct DrawResolved {
    pub index_winner: u8,
    pub winner: Pubkey,
}

// Reset
#[event]
pub struct GameReset {
    pub round:          u32,
    pub winner:         Pubkey,
    pub winner_banked:  u64,

    pub time_max:       i64,
    pub time_increment: i64,
    pub play_cost:      u64,

    pub funds:          u64,
}

// WithdrawCreator
#[event]
pub struct WithdrawnCreator {
    pub amount:    u64,
    pub new_funds: u64,
}

// Withdraw
#[event]
pub struct Withdrawn {
    //todo
    pub player: Pubkey,
    pub amount: u64,
    // pub requester: Pubkey,
}