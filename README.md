# Soroban Token Contract

A feature-rich token contract implementation for the Stellar blockchain, with advanced functionality including minting, burning, freezing accounts, and granular administrative controls.

> Deployed on Testnet: [View on Stellar Expert](https://stellar.expert/explorer/testnet/contract/CCRNUHL5YMSNC4D3JZIRB22IQEVUXK6OTGI2RHNPU77C3IDTWCWQWXHY)

## Features

- **Standard Token Operations**
  - Transfer tokens between accounts
  - Check account balances
  - Approve and manage allowances for delegated transfers

- **Administrative Functions**
  - Mint new tokens
  - Burn existing tokens
  - Freeze/unfreeze accounts
  - Update contract administrator

- **Security**
  - Built-in authorization checks
  - Expiring allowances
  - Balance overflow protection
  - Account freezing capability

## Project Structure

```plaintext
.
├── contracts/
│   └── token/
│       ├── src/
│       │   ├── contract.rs  # Main contract implementation
│       │   ├── storage.rs   # Contract storage definitions
│       │   └── utils/       # Helper functions and utilities
│       ├── Cargo.toml       # Contract dependencies
│       └── Makefile         # Build and test commands
└── frontend/
    ├── app/                 # Next.js application
    ├── components/          # React components
    └── lib/                 # Utilities and stores
```

## Getting Started

### Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install)
- [Soroban CLI](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup)
- Node.js and npm/yarn

### Building the Contract

```bash
stellar contract build
```

### Running Tests

```bash
cargo test
```

### Optimization and Deployment

```bash
stellar contract optimize --wasm target/wasm32v1-none/release/token.wasm
stellar contract deploy --wasm target/wasm32v1-none/release/token.optimized.wasm --source alice --network testnet --alias token
```

### Generating TypeScript Client

```bash
cd frontend
cp -R ../.stellar .stellar
stellar contract bindings typescript --network testnet --contract-id token --output-dir packages/token
npm install
npm run build
```

### Starting the Frontend

```bash
cd frontend
npm install
npm run dev
```

## Contract Interface

### User Functions

```rust
pub trait Interface {
    // Check token balance for an account
    fn balance(env: Env, id: Address) -> i128;
    
    // Transfer tokens to another account
    fn transfer(env: Env, from: Address, to: Address, amount: i128);
    
    // Check allowance for a spender
    fn allowance(env: Env, from: Address, spender: Address) -> i128;
    
    // Approve tokens for a spender
    fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32);
}
```

### Admin Functions

```rust
impl TokenContract {
    // Initialize the contract
    pub fn initialize(env: Env, admin: Address, decimal: u32, name: String, symbol: String);
    
    // Mint new tokens
    pub fn mint(env: Env, to: Address, amount: i128);
    
    // Update contract administrator
    pub fn set_admin(env: Env, new_admin: Address);
    
    // Freeze an account
    pub fn freeze_account(env: Env, account: Address);
    
    // Unfreeze an account
    pub fn unfreeze_account(env: Env, account: Address);
}
```

## Frontend Features

- Wallet integration
- Token balance display
- Transfer functionality
- Transaction history
- Administrative panel
- Account management

## Development

The project uses:

- Soroban SDK
- Next.js
- React
- TailwindCSS
- shadcn/ui components

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is open source and available under the MIT License.

## Acknowledgments

- Built with [Soroban](https://developers.stellar.org/)
- UI components from [shadcn/ui](https://ui.shadcn.com/)
