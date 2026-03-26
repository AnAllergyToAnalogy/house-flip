// Need to use player accounts 

use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] 
pub struct Player {
    pub banked: u64,
}


