use anchor_lang::prelude::*;

// Economics
pub const PRIZE:    u64 =  25_00;
pub const HOUSE:    u64 =  13_00;
pub const PROCESS:  u64 =   2_00;
pub const EARN:     u64 =  60_00;
pub const HUNDRED:  u64 = 100_00;

// Default Settings
pub const DEFAULT_TIME_MAX:         i64 = 60 * 2; // 2 minutes
pub const DEFAULT_TIME_INCREMENT:   i64 = 10;     // 10 seconds
pub const DEFAULT_PLAY_COST:        u64 = 100_000_000; //100 Tokens (6 Decimals)

pub const PROGRAM_ID:   Pubkey = pubkey!("E2KAtVvm2N5otMYLwXXGySP9ZGEf7aRy4BDarajw3eJa");

pub const ZERO:         Pubkey = pubkey!("11111111111111111111111111111111");


