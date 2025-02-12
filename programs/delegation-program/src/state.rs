use anchor_lang::prelude::*;

#[account]
pub struct DelegatedAccount {
    /// The owner of this delegated account
    pub owner: Pubkey,
    /// Total amount of SOL currently delegated
    pub delegated_amount: u64,
    /// Timestamp of the last deposit
    pub last_deposit_time: i64,
    /// Bump seed for PDA derivation
    pub bump: u8,
}

impl DelegatedAccount {
    pub const LEN: usize = 8 + // discriminator
        32 + // owner pubkey
        8 +  // delegated_amount
        8 +  // last_deposit_time
        1;   // bump
} 