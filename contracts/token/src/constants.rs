// Ledger sequences (aka "Ledgers") is the unit of time in Stellar
pub const DAY_IN_LEDGERS: u32 = 17280;

pub const INSTANCE_TTL_FULL: u32 = 7 * DAY_IN_LEDGERS;
pub const INSTANCE_TTL_THRESHOLD: u32 = INSTANCE_TTL_FULL - DAY_IN_LEDGERS;

pub const BALANCE_TTL_FULL: u32 = 30 * DAY_IN_LEDGERS;
pub const BALANCE_TTL_THRESHOLD: u32 = BALANCE_TTL_FULL - DAY_IN_LEDGERS;
