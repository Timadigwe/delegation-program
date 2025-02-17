# Solana Delegation Program for SVM Rollup

A Solana program that serves as the bridge between mainnet and the SVM rollup system. This program enables secure fund delegation for rollup operations by allowing users to lock their SOL in Program Derived Address (PDA) accounts, which can then be safely accessed by the rollup for transaction processing.

## Overview

This delegation program is a critical component of the SVM rollup architecture, providing:
- Secure fund locking mechanism on Solana mainnet
- State consistency between mainnet and rollup
- Prevention of double-spending across environments
- Transaction validation through fund verification

## Integration with SVM Rollup

### Rollup Components Interaction
1. **Sequencer Integration**
   - The sequencer monitors delegation accounts for new deposits
   - Verifies available funds before processing rollup transactions
   - Tracks user balances across the rollup system

2. **RollupDB Interaction**
   - Maintains synchronized state with delegation accounts
   - Tracks locked funds and transaction history
   - Ensures consistency between mainnet and rollup states

3. **Settlement Process**
   - Enables state settlement back to mainnet
   - Verifies fund availability during settlement
   - Maintains proof of state transitions

## Technical Architecture

### State Management
```rust
pub struct DelegatedAccount {
    pub owner: Pubkey,           // User's wallet address
    pub delegated_amount: u64,   // Locked SOL amount
    pub last_deposit_time: i64,  // Timestamp for rollup validation
    pub bump: u8,                // PDA bump seed
}
```

### Core Instructions

1. **Initialize Delegate**
   ```typescript
   await program.methods.initializeDelegate().rpc();
   ```
   - Creates PDA for user's delegated funds
   - Enables rollup to track user participation

2. **Deposit**
   ```typescript
   await program.methods.deposit(amount).rpc();
   ```
   - Locks user's SOL in PDA
   - Signals fund availability to rollup

3. **Withdraw**
   ```typescript
   await program.methods.withdraw(amount).rpc();
   ```
   - Releases funds back to user
   - Updates rollup state accordingly

## Security Model

1. **Fund Security**
   - SOL locked in PDAs
   - Only owner can withdraw
   - Rollup can only process within delegated limits

2. **State Consistency**
   - Timestamp tracking for deposit verification
   - Amount overflow protection
   - Synchronized state management

3. **Access Control**
   - PDA-based account derivation
   - Owner-only withdrawal rights
   - Rollup validation checks

## Development Setup

### Prerequisites
- Solana Tool Suite
- Anchor Framework (v0.30.1)
- Node.js & Yarn
- Rust

### Installation
```bash
git clone https://github.com/Timadigwe/delegation-program.git
cd delegation-program
yarn install
anchor build
```

### Testing
```bash
anchor test
```

## Usage in Rollup System

1. **User Perspective**
   ```typescript
   // Find user's delegation account
   const [delegateAccount] = PublicKey.findProgramAddressSync(
       [Buffer.from("delegate"), userWallet.toBuffer()],
       programId
   );
   ```

2. **Rollup Integration**
   ```rust
   // Rollup verification example
   pub async fn verify_delegation(user: Pubkey, amount: u64) -> Result<bool> {
       let delegation_account = find_delegation_account(user);
       delegation_account.delegated_amount >= amount
   }
   ```

## Program Deployment

### Mainnet
- Program ID: `E1bxy4HwKFjPARhVe7NjvoFtynN69C4xNA53uSwruHrP`

### Local Development
```bash
anchor deploy
```

## Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

MIT License

## Links

- [SVM Rollup Repository](https://github.com/Timadigwe/Basic_Rollup_fork)
- [Delegation Program Repository](https://github.com/Timadigwe/delegation-program)

