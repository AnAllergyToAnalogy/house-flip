use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] 
pub struct Settings {
    pub time_max:       i64,
    pub time_increment: i64,
    pub play_cost:      u64,
}


