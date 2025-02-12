# Solana Delegation Program

A Solana program that enables users to create delegated accounts and manage SOL deposits and withdrawals. Built using the Anchor framework.

## Features

- Create delegated accounts associated with user wallets
- Deposit SOL into delegated accounts
- Withdraw SOL from delegated accounts
- Track deposit timestamps and delegated amounts
- Secure account management using Program Derived Addresses (PDAs)

## Technical Architecture

The program consists of three main components:

1. **State Management** (`state.rs`)
   - `DelegatedAccount`: Stores account information including owner, delegated amount, and last deposit timestamp

2. **Program Instructions** (`lib.rs`)
   - `initialize_delegate`: Creates a new delegated account
   - `deposit`: Transfers SOL to the delegated account
   - `withdraw`: Withdraws SOL from the delegated account

3. **Tests** (`delegation-program.ts`)
   - Integration tests for account initialization and PDA derivation

## Installation

```bash
# Clone the repository
git clone [your-repo-url]

# Install dependencies
yarn install

# Build the program
anchor build
```

## Usage

### Program ID
```
E1bxy4HwKFjPARhVe7NjvoFtynN69C4xNA53uSwruHrP
```

### Initialize a Delegated Account
```typescript
const tx = await program.methods
  .initializeDelegate()
  .accounts({
    owner: yourWallet.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();
```

### Deposit SOL
```typescript
const tx = await program.methods
  .deposit(new BN(amountInLamports))
  .accounts({
    owner: yourWallet.publicKey,
    delegatedAccount: delegatedAccountPDA,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();
```

### Withdraw SOL
```typescript
const tx = await program.methods
  .withdraw(new BN(amountInLamports))
  .accounts({
    owner: yourWallet.publicKey,
    delegatedAccount: delegatedAccountPDA,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();
```

## Testing

```bash
# Run tests
anchor test
```

## Security Considerations

- All withdrawals require owner signature
- PDA-based account system prevents unauthorized access
- Amount overflow checks implemented
- Insufficient funds checks for withdrawals

## Development

### Prerequisites

- Rust
- Solana Tool Suite
- Anchor Framework
- Node.js
- Yarn

### Building

```bash
anchor build
```

### Deploying

```bash
anchor deploy
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

Project Link: [your-repo-url]
