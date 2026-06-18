NeuroWealth 💰

AI-Powered DeFi Yield Platform on Stellar

NeuroWealth is an autonomous AI investment agent that automatically manages and grows your crypto assets on the Stellar blockchain. Deposit once, let the AI find the best yield opportunities across Stellar's DeFi ecosystem — and withdraw anytime with no lock-ups.



Overview
Traditional savings accounts offer near-zero interest. Traditional DeFi is too complex for most users. NeuroWealth bridges the gap  a simple chat interface web  powered by an AI agent that autonomously deploys your funds into the highest-yielding, safest opportunities on Stellar.

Why Stellar?

Transaction fees of fractions of a penny — perfect for frequent AI-driven rebalancing
3–5 second finality — the AI can act on market changes instantly
Native DEX + Soroban smart contracts — composable, programmable yield strategies
Native USDC + XLM — borderless capital movement with no friction
Growing DeFi ecosystem — Blend (lending), Templar (borrowing), RWA protocols


Features
FeatureDescription🤖 AI AgentAutonomous 24/7 yield optimization across Stellar DeFi💬 Natural LanguageChat to deposit, withdraw, and check balances📈 Auto-RebalancingAgent shifts funds to best opportunities automatically🔐 Non-CustodialYour funds live in audited Soroban smart contracts⚡ Instant WithdrawalsNo lock-ups, no penalties, withdraw anytime📱 WhatsApp ReadyFull functionality through WhatsApp chat interface🌍 Global AccessNo geographic restrictions, no bank account required🛡️ Security FirstSoroban contracts protected by strict CEI ordering and access controls

How It Works
1. User deposits USDC via web app
       ↓
2. Soroban vault contract receives and records the deposit
       ↓
3. Contract emits a deposit event
       ↓
4. AI agent detects the event and deploys funds to best protocol (e.g. Blend)
       ↓
5. Yield accumulates 24/7 — agent rebalances hourly if better opportunities exist
       ↓
6. User requests withdrawal anytime → agent pulls funds → sends back in seconds
Three Investment Strategies

Conservative — Stablecoin lending on Blend. Low risk, steady 3–6% APY.
Balanced — Mix of lending + DEX liquidity provision. Medium risk, 6–10% APY.
Growth — Aggressive multi-protocol deployment. Higher risk, 10–15% APY.


Tech Stack
Smart Contracts

Language: Rust (Soroban SDK 21.0.0)
Standard: ERC-4626 inspired vault architecture
Network: Stellar Mainnet / Testnet
Security: OpenZeppelin-equivalent patterns (Pausable, Access Control) and strict CEI pattern for reentrancy protection

Backend / AI Agent

Runtime: Node.js or Python
Stellar SDK: @stellar/stellar-sdk
AI: Claude API / OpenAI for natural language intent parsing
Database: PostgreSQL / Supabase for user position tracking
Queue: Bull / Redis for reliable transaction processing

Frontend

Framework: Next.js 15
Blockchain: Stellar SDK + Freighter wallet integration
Styling: Tailwind CSS
Charts: Recharts for portfolio analytics

Integrations

Yield Protocols: Blend Protocol (lending), Stellar DEX (liquidity)
Price Feeds: Stellar anchor price feeds


## Project Structure

```text
NeuroWealth-Smartcontract/
├── neurowealth-vault/          # Soroban smart contracts workspace
│   ├── Cargo.toml
│   ├── contracts/
│   │   └── vault/              # Core vault contract
│   │       ├── Cargo.toml
│   │       └── src/
│   │           ├── lib.rs      # Contract logic, events, error types
│   │           └── topics.rs   # Exported event topic constants
│   └── fuzz/                   # Libfuzzer fuzz targets
├── scripts/                    # Deployment and utility scripts
│   ├── deploy-devnet.sh        # One-command devnet deploy
│   ├── e2e-devnet.sh           # End-to-end devnet tests
│   ├── verify-deployment.sh
│   ├── generate-spec.py        # Generate contract-spec.json
│   ├── validate-spec.py
│   ├── README-E2E.md           # E2E test guide
│   └── README-SPEC.md          # Spec generation guide
├── docs/
│   ├── MAINNET_CHECKLIST.md    # Pre-mainnet sign-off checklist
│   ├── UPGRADE_MIGRATION.md    # Contract upgrade guide
│   └── WASM_SIZE.md            # WASM size tracking
├── .env.devnet.template        # Environment variable template
├── deny.toml                   # cargo-deny dependency audit config
├── ARCHITECTURE.md             # Storage layout, data flows, invariants
├── EVENTS.md                   # Full event schema reference
├── SECURITY.md                 # Trust model and threat analysis
├── CONTRIBUTING.md             # Development setup and PR process
├── CHANGELOG.md
└── README.md
```

### Planned Components

The following are not yet in this repository and will be added as separate
directories once development begins:

| Component | Directory | Status |
|-----------|-----------|--------|
| AI agent backend (Node.js / Python) | `agent/` | Planned |
| Next.js web frontend | `frontend/` | Planned |
| WhatsApp bot handler | `whatsapp/` | Planned |

## Getting Started

### Prerequisites

Install Rust and the WASM target:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
```

Install the Stellar CLI (pinned to 21.2.0):
```bash
cargo install --locked stellar-cli --version 21.2.0 --features opt
```

### Environment Variables

Copy the template and add your secret key:
```bash
cp .env.devnet.template .env.devnet
# Edit .env.devnet and set SOROBAN_SECRET_KEY
```

### Build the Contract

```bash
cd neurowealth-vault
stellar contract build
```

The compiled WASM is output to `target/wasm32v1-none/release/neurowealth_vault.wasm`.

### Run Tests

```bash
cd neurowealth-vault
cargo test
```

### Deploy to Devnet

```bash
./scripts/deploy-devnet.sh
```

See [`scripts/README-E2E.md`](scripts/README-E2E.md) for end-to-end devnet validation.

> For the AI agent, frontend, and WhatsApp bot — see [Planned Components](#planned-components) above.

## Further Reading

| Document | Purpose |
|----------|---------|
| [`ARCHITECTURE.md`](ARCHITECTURE.md) | Storage layout, share accounting math, asset flow diagrams |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | Development setup, CI requirements, PR process |
| [`scripts/README-E2E.md`](scripts/README-E2E.md) | End-to-end devnet test guide |
| [`SECURITY.md`](SECURITY.md) | Trust model and threat analysis |
| [`docs/MAINNET_CHECKLIST.md`](docs/MAINNET_CHECKLIST.md) | Pre-mainnet deployment checklist |

## Smart Contract
The core Soroban vault contract handles all on-chain fund management.
Key Functions

| Function | Who Can Call | Description |
| :--- | :--- | :--- |
| `initialize` | Owner (once) | Set agent address and USDC token |
| `deposit` | Any verified user | Deposit USDC into the vault |
| `withdraw` | User (their own funds) | Withdraw USDC back to wallet |
| `withdraw_all` | User (their own funds) | Withdraw all USDC by burning all shares |
| `rebalance` | AI Agent only | Move funds between yield strategies (`protocol`, `expected_apy`, `min_out`; supported: `blend`, `dex`, `none`) |
| `set_blend_pool` | Owner only | Configure the Blend lending pool address |
| `set_dex_pool` | Owner only | Configure the DEX liquidity pool address |
| `get_balance` | Anyone | Read a user's current balance |
| `get_total_deposits` | Anyone | Read total vault TVL |
| `get_exchange_rate` | Anyone | Read current exchange rate (assets per share * 10,000,000) |
| `transfer_ownership` | Owner only | Initiate two-step ownership transfer |
| `accept_ownership` | Pending owner only | Complete ownership transfer |
| `set_caps` | Owner only | Sets both user deposit cap and TVL cap in a single transaction |
| `set_deposit_limits` | Owner only | Sets minimum and maximum per-transaction deposit limits |
| `set_tvl_cap` | Owner only | Sets the maximum total TVL that can be deposited |
| `set_user_deposit_cap` | Owner only | Sets the maximum deposit amount per user |
| `set_limits` | Owner only | **Deprecated**: Sets user deposit cap and TVL cap (use `set_caps` instead) |
Security Model

Users can only withdraw their own funds — enforced at the contract level via user.require_auth()
Only the designated AI agent keypair can call rebalance — no other address can move funds between protocols
Minimum deposit: 1 USDC. Maximum per user: 10,000 USDC (configurable)
Emergency pause functionality available to contract owner
Two-step ownership transfer prevents accidental ownership loss
Vault balance verification ensures reported assets match actual holdings
Read-only getters have no TTL side effects; call `touch_user_ttl` to extend user share entry TTL
Strict Checks-Effects-Interactions (CEI) pattern prevents reentrancy without needing explicit locks (see [reentrancy protection tests](neurowealth-vault/contracts/vault/src/tests/test_legacy_inline.rs))

Secure Deployment Sequence

`initialize()` is protected against front-running: the contract verifies that the `deployer`
argument + `salt` cryptographically reproduce the deployed contract address, **and** requires
a live authorization signature from that deployer keypair. This means no third party can
seize ownership even if they observe the deployment transaction in the mempool.

Follow these steps in order to safely initialize a new vault:

1. **Generate a deployer keypair** (one-time use, only for initialization):
   ```bash
   stellar keys generate deployer --network testnet
   stellar keys address deployer   # note the deployer address
   ```

2. **Choose a salt** (32 bytes; any fixed value works — must be the same across steps):
   ```bash
   # example: all-zero salt
   SALT="0000000000000000000000000000000000000000000000000000000000000000"
   ```

3. **Deploy the contract** using the deployer keypair and the chosen salt:
   ```bash
   stellar contract deploy \
     --wasm target/wasm32-unknown-unknown/release/neurowealth_vault.wasm \
     --source deployer \
     --network testnet \
     --salt $SALT
   # save the output as VAULT_CONTRACT_ID
   ```

4. **Immediately call `initialize()`** from the same deployer keypair:
   ```bash
   stellar contract invoke \
     --id $VAULT_CONTRACT_ID \
     --source deployer \
     --network testnet \
     -- \
     initialize \
     --deployer $(stellar keys address deployer) \
     --owner  $OWNER_ADDRESS \
     --agent  $AGENT_ADDRESS \
     --usdc_token $USDC_TOKEN_ADDRESS \
     --salt   $SALT
   ```
   The contract rejects any caller whose `deployer` argument does not reproduce
   `VAULT_CONTRACT_ID`, and additionally requires a valid signature from that
   address via `deployer.require_auth()`.

5. **Verify initialization** (read-only, no auth needed):
   ```bash
   stellar contract invoke --id $VAULT_CONTRACT_ID --source deployer \
     --network testnet -- get_owner
   stellar contract invoke --id $VAULT_CONTRACT_ID --source deployer \
     --network testnet -- get_agent
   ```

6. **Secure or discard the deployer keypair** — it has no further privileged role
   after initialization. The `owner` keypair is now the administrator.


AI Agent
The agent runs as a persistent background service with two main loops.
Decision Loop (runs every hour)
1. Fetch current APY from all integrated protocols (Blend, DEX pools)
2. Compare against each user's current deployed strategy
3. If a better opportunity exists (> 0.5% improvement), rebalance
4. Submit rebalance transaction to vault contract
5. Log results to database
Intent Parser (real-time, event-driven)
User message: "deposit 50 USDC into balanced strategy"
       ↓
AI parses intent: { action: "deposit", amount: 50, strategy: "balanced" }
       ↓
Agent builds Stellar transaction
       ↓
Returns confirmation: "Deposited 50 USDC. Earning ~8.2% APY in Balanced strategy."
Supported User Intents

deposit [amount] [optional: strategy]
withdraw [amount or "all"]
balance / how much do I have
earnings / how much have I made
switch to [conservative/balanced/growth]
what is my APY


WhatsApp Integration
NeuroWealth is designed to be fully operable through WhatsApp, making it accessible to anyone with a smartphone — no wallet app or browser extension needed.
User Flow
1. User sends "hi" to NeuroWealth WhatsApp number
2. Bot introduces itself and asks for phone number verification (OTP)
3. OTP verified → agent creates a Stellar keypair for this user (custodial)
4. User can now deposit, withdraw, and check balance entirely through chat
5. Funds are secured in the Soroban vault contract under their wallet address
Setting Up the Webhook
bash# Your webhook endpoint receives WhatsApp messages
POST /api/whatsapp/webhook

# Register your webhook URL with Twilio
# ngrok http 3000  ← for local testing
Example Conversation
User:    deposit 100 USDC
Agent:   Got it! Depositing 100 USDC into your Balanced strategy.
         This should take about 5 seconds on Stellar... ✅ Done!
         You're now earning ~8.4% APY. I'll optimize automatically.

User:    what's my balance?
Agent:   💰 Your NeuroWealth Portfolio
         Balance: 100.23 USDC
         Earnings today: +$0.23
         Current APY: 8.4%
         Strategy: Balanced

User:    withdraw everything
Agent:   Withdrawing 100.23 USDC... ✅ Done!
         Funds sent to your wallet. Arrived in 4 seconds.

Deployment
Quick Start (Devnet)

For testing and development, you can deploy to Stellar devnet in minutes:

1. **Get a funded devnet account**
   ```bash
   # Visit https://laboratory.stellar.org/#account-creator
   # Create an account and copy the secret key
   ```

2. **Set up environment**
   ```bash
   # Copy the template and add your secret key
   cp .env.devnet.template .env.devnet
   # Edit .env.devnet and add your SOROBAN_SECRET_KEY
   ```

3. **Build contracts**
   ```bash
   cd neurowealth-vault
   stellar contract build
   ```

4. **Deploy to devnet**
   ```bash
   ./scripts/deploy-devnet.sh
   ```

5. **Start using the vault**
   ```bash
   # Source the deployed contract addresses
   source scripts/devnet-contracts.env
   
   # Check your balance
   stellar contract invoke \
     --id $VAULT_CONTRACT_ID \
     --source $AGENT_SECRET_KEY \
     --network $SOROBAN_NETWORK_PASSPHRASE \
     --rpc-url $SOROBAN_RPC_URL \
     -- \
     get_balance \
     --user $AGENT_ADDRESS
   
   # Deposit 10 USDC
   stellar contract invoke \
     --id $VAULT_CONTRACT_ID \
     --source $AGENT_SECRET_KEY \
     --network $SOROBAN_NETWORK_PASSPHRASE \
     --rpc-url $SOROBAN_RPC_URL \
     -- \
     deposit \
     --user $AGENT_ADDRESS \
     --amount 10000000
   ```

The deployment script will:
- Deploy the USDC token contract
- Deploy the NeuroWealth vault contract
- Initialize the vault with your account as the AI agent
- Mint 10,000 USDC for testing
- Save all contract addresses to `scripts/devnet-contracts.env`

Testnet
```bash
# Deploy everything to Stellar testnet
./scripts/deploy.sh --network testnet
```

Mainnet
⚠️ **CRITICAL:** Before deploying to Stellar mainnet, you must complete and sign off on all items in the [Mainnet Deployment Checklist](docs/MAINNET_CHECKLIST.md) (including separate keys setup, TVL limits, Blend pool verification, pause drills, and multisig governance plans).

```bash
# Ensure all tests pass first
cargo test
npm test

# Deploy to mainnet
./scripts/deploy.sh --network mainnet
```
Infrastructure (Recommended)

Agent: Railway, Render, or a VPS (needs to run 24/7)
Frontend: Vercel
Database: Supabase (managed PostgreSQL)
Webhook: Same server as agent, or a separate serverless function


Roadmap
Phase 1 — Foundation (Current)

 Soroban vault contract (deposit, withdraw, rebalance)
 Basic AI agent with Blend protocol integration
 Natural language intent parsing
 Web frontend with portfolio dashboard
 WhatsApp bot MVP

Phase 2 — Intelligence

 Multi-protocol yield aggregation (Blend + DEX liquidity pools)
 Strategy backtesting and risk scoring
 Personalized risk profiles per user
 Earnings history and projection charts

Phase 3 — Scale

 Real-world asset (RWA) yield strategies
 Cross-chain bridging (Stellar ↔ Ethereum via Axelar)
 Social trading — follow top-performing AI strategies
 NeuroWealth token for governance and fee sharing


## Contributing
Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details on our code of conduct, development setup, and the process for submitting pull requests.

### Quick Start for Contributors
1. **Fork the repo**, then:
   ```bash
   git checkout -b feature/your-feature-name
   git commit -m "feat: add your feature"
   git push origin feature/your-feature-name
   ```
2. **Open a Pull Request** against the `develop` branch.
3. Please make sure to run `cargo test` and `npm test` before submitting.

