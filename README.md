SolVesting - Solana Vesting Contract

A comprehensive, open-source vesting solution for Solana blockchain that enables projects to securely lock and distribute tokens over time.

📋 Overview

SolVesting provides smart contracts for creating and managing token vesting schedules on Solana. Perfect for:

· Team token allocations
· Investor lockups
· Community rewards distribution
· Token launch vesting

✨ Features

Single Vesting

· Create individual vesting schedules with custom parameters
· Set start time, end time, and cliff period
· Optional revoke authority for emergency situations
· Linear vesting calculation

Batch Vesting

· Create up to 25 vesting schedules in one transaction
· Bulk upload recipients and amounts
· Different vesting schedules per recipient
· Saves time and transaction costs

Vesting Types Supported

· Linear (even distribution over time)
· Cliff then linear (wait period then linear)
· Custom schedules

Platform Features

· Owner-controlled fee configuration
· Emergency pause mechanism
· Fee withdrawal to designated collector
· Revocation capability
· Claim fee option (can be enabled/disabled)

🏗️ Architecture

Program ID (Devnet)

```
A9UztnDfy5bSmpg1RUcEUMBtat45aRhv8fF5k7s1JkZ7
```

Account Structures

FeeConfig

Stores platform-wide settings:

· Owner and fee collector addresses
· Single vesting fixed fee
· Batch vesting fee (basis points with min/max)
· Claim fee settings
· Pause status

VestingAccount

Tracks individual vesting:

· Recipient and authority
· Total amount and claimed amount
· Time parameters (start, end, cliff)
· Revocation status
· Created at and last claim time

BatchVestingAccount

Manages batch vesting:

· Batch ID and creator
· Total recipients and amount
· Metadata URI for off-chain data
· Batch status tracking

🚀 Getting Started

Prerequisites

· Solana CLI tools
· Node.js (v16+)
· Anchor framework
· A Solana wallet with devnet SOL

Installation

1. Clone the repository

```bash
git clone https://github.com/azeezdamilarerasheed388-sudo/sol_vesting.git
cd sol_vesting
```

1. Install dependencies

```bash
npm install
```

1. Build the program

```bash
anchor build
```

1. Deploy to devnet

```bash
anchor deploy --provider.cluster devnet
```

Initialize Platform

```bash
# Initialize with your wallet as owner
anchor run initialize --provider.wallet <your-wallet.json>
```

📖 Usage

Create Single Vesting

```javascript
const vestingParams = {
  recipient: new PublicKey("RECIPIENT_ADDRESS"),
  amount: new BN(1000000000), // 1 SOL in lamports
  startTime: new BN(1640995200), // Unix timestamp
  endTime: new BN(1643673600),
  cliffTime: new BN(1640995200),
  revokeAuthority: null // Optional
};

await program.methods
  .createVesting(vestingParams, new BN(12345))
  .accounts({
    vestingAccount: vestingPDA,
    config: configPDA,
    authority: wallet.publicKey,
    recipient: recipientPubkey,
    systemProgram: SystemProgram.programId
  })
  .rpc();
```

Claim Vested Tokens

```javascript
await program.methods
  .claimVested()
  .accounts({
    vestingAccount: vestingPDA,
    config: configPDA,
    recipient: wallet.publicKey
  })
  .rpc();
```

Batch Vesting

```javascript
const batchParams = {
  batchId: new BN(1),
  recipients: [addr1, addr2, addr3],
  amounts: [new BN(1000), new BN(2000), new BN(1500)],
  schedules: [schedule1, schedule2, schedule3],
  metadataUri: "https://example.com/metadata.json"
};

await program.methods
  .createBatchVesting(batchParams)
  .accounts({
    batchAccount: batchPDA,
    config: configPDA,
    authority: wallet.publicKey,
    systemProgram: SystemProgram.programId
  })
  .rpc();
```

🔧 Admin Functions

Update Fees

```javascript
await program.methods
  .updateFeeConfig(
    new BN(2000000), // single vesting fee
    100, // batch fee bps (1%)
    new BN(50000000), // min fee
    new BN(5000000000), // max fee
    new BN(1000000), // claim fee
    true, // claim fee enabled
    new PublicKey("NEW_FEE_COLLECTOR")
  )
  .accounts({
    config: configPDA,
    owner: wallet.publicKey
  })
  .rpc();
```

Withdraw Fees

```javascript
await program.methods
  .withdrawFees(null) // null = withdraw all
  .accounts({
    config: configPDA,
    owner: wallet.publicKey,
    feeCollector: feeCollectorPubkey
  })
  .rpc();
```

Emergency Pause

```javascript
await program.methods
  .setPause(true)
  .accounts({
    config: configPDA,
    owner: wallet.publicKey
  })
  .rpc();
```

🧪 Testing

Run the test suite:

```bash
anchor test
```

Tests cover:

· Claimable amount calculations
· Batch fee calculations
· Vesting creation
· Claims and revocations

📊 Fee Structure

Fee Type Description Range
Single Vesting Fixed fee per vesting Configurable
Batch Vesting Percentage-based 0.01% - 100% (capped)
Claim Fee Optional fixed fee Configurable

🔒 Security Features

· Owner-only admin functions
· Emergency pause mechanism
· Revocation capability
· Time-based access controls
· PDA seeds for deterministic addresses
· Comprehensive error handling

🗺️ Roadmap

Phase 1 (Current)

· ✅ Smart contract development
· ✅ Unit tests
· ✅ Devnet deployment

Phase 2 (In Progress)

· 🔄 Frontend interface
· 🔄 User dashboard
· 🔄 Batch upload feature

https://github.com/azeezdamilarerasheed388-sudo/Vesting

Phase 3 (Planned)

· ⬜ Security audit
· ⬜ Mainnet deployment
· ⬜ Documentation site
· ⬜ Analytics dashboard

🤝 Contributing

Contributions are welcome! Please read our contributing guidelines before submitting PRs.

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Open a Pull Request

📄 License

MIT License - see LICENSE file for details

📞 Contact

· Telegram: @earnwithdammy1
· X: @Earnwithdammy

🙏 Acknowledgments

· Solana Foundation
· Anchor Framework
· Solana Playground
· Superteam Earn

---

Built for the Solana ecosystem 🚀
