use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("E1bxy4HwKFjPARhVe7NjvoFtynN69C4xNA53uSwruHrP");

pub mod state;
use state::*;

#[program]
pub mod delegation_program {
    use super::*;

    pub fn initialize_delegate(ctx: Context<InitializeDelegate>, amount: u64) -> Result<()> {
        // Get account info before mutable borrow
        let delegated_account_info = ctx.accounts.delegated_account.to_account_info();
        
        let delegated_account = &mut ctx.accounts.delegated_account;
        delegated_account.owner = ctx.accounts.owner.key();
        delegated_account.delegated_amount = 0;
        delegated_account.last_deposit_time = Clock::get()?.unix_timestamp;
        delegated_account.bump = ctx.bumps.delegated_account;

        // Process the initial deposit using stored account info
        let transfer_ix = system_program::Transfer {
            from: ctx.accounts.owner.to_account_info(),
            to: delegated_account_info,
        };

        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                transfer_ix,
            ),
            amount,
        )?;

        delegated_account.delegated_amount = amount;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let delegated_account = &mut ctx.accounts.delegated_account;
        require!(
            amount <= delegated_account.delegated_amount,
            ErrorCode::InsufficientFunds
        );

        // Calculate amount of lamports to transfer
        // let rent = Rent::get()?;
        // let min_rent = rent.minimum_balance(DelegatedAccount::LEN);
        
        // Transfer SOL from PDA to user
        **delegated_account.to_account_info().try_borrow_mut_lamports()? = delegated_account
            .to_account_info()
            .lamports()
            .checked_sub(amount)
            .ok_or(ErrorCode::AmountOverflow)?;
            
        **ctx.accounts.owner.try_borrow_mut_lamports()? = ctx
            .accounts.owner
            .lamports()
            .checked_add(amount)
            .ok_or(ErrorCode::AmountOverflow)?;

        delegated_account.delegated_amount = delegated_account.delegated_amount
            .checked_sub(amount)
            .ok_or(ErrorCode::AmountOverflow)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeDelegate<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        init,
        payer = owner,
        space = DelegatedAccount::LEN,
        seeds = [b"delegate", owner.key().as_ref()],
        bump
    )]
    pub delegated_account: Account<'info, DelegatedAccount>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"delegate", owner.key().as_ref()],
        bump = delegated_account.bump,
        constraint = delegated_account.owner == owner.key()
    )]
    pub delegated_account: Account<'info, DelegatedAccount>,
    
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Amount overflow")]
    AmountOverflow,
    #[msg("Insufficient funds")]
    InsufficientFunds,
}
