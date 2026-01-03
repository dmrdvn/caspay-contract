# CasPay Core Contract

Smart contract for CasPay payment platform built with Odra Framework for Casper Network.

## Features

- ✅ Merchant registration
- ✅ Product management
- ✅ Subscription plans
- ✅ Payment recording
- ✅ Admin access control

## Quick Start

### Prerequisites

- Rust nightly toolchain (nightly-2025-01-01)
- cargo-odra (`cargo install cargo-odra`)
- Node.js 18+ (for deployment proxy)
- CSPR.cloud API key ([Get one here](https://console.cspr.build/))
- Casper testnet account with CSPR ([Get from faucet](https://testnet.cspr.live/tools/faucet))

### 1. Generate Casper Testnet Keys

Create a keypair for contract deployment:

```bash
# Install Casper client
cargo install casper-client

# Generate keys
mkdir -p ~/casper-keys/testnet
cd ~/casper-keys/testnet
casper-client keygen .

# Output:
# ✅ secret_key.pem  (keep private!)
# ✅ public_key.pem
# ✅ public_key_hex  (your account hash)
```

**Fund your account:**
1. Copy your public key hex: `cat ~/casper-keys/testnet/public_key_hex`
2. Visit https://testnet.cspr.live/tools/faucet
3. Request testnet CSPR (you'll need ~500 CSPR for deployment)

### 2. Configure Environment

```bash
cp .env.example .env
```

Edit `.env` file:

```bash
# Local proxy for deployment
ODRA_CASPER_LIVENET_NODE_ADDRESS=http://127.0.0.1:7778
ODRA_CASPER_LIVENET_EVENTS_URL=http://127.0.0.1:7778/events

# Testnet configuration
ODRA_CASPER_LIVENET_CHAIN_NAME=casper-test

# Your CSPR.cloud API token (get from https://console.cspr.build/)
CSPR_CLOUD_AUTH_TOKEN=your-api-token-here

# Path to your deployment private key
ODRA_CASPER_LIVENET_SECRET_KEY_PATH=yourkeypath/secret_key.pem

# Transaction TTL
ODRA_CASPER_LIVENET_TTL=30m
```

### 3. Build & Test

```bash
# Run all tests
cargo odra test

# Or run only integration tests
cargo test --test integration_tests

# Build WASM contract
cargo odra build

# Output: wasm/CasPay.wasm
```

### 4. Deploy to Testnet

**Two terminals required:**

**Terminal 1 - Start Deployment Proxy:**
```bash
node cspr_proxy.js

# Output:
# CSPR.cloud Proxy running on http://127.0.0.1:7778
# Forwarding to: https://node.testnet.cspr.cloud
# Ready for contract deployment!
```

> **Why proxy?** Odra framework doesn't support custom HTTP headers. The proxy adds CSPR.cloud authentication automatically.

**Terminal 2 - Deploy Contract:**
```bash
cargo run --bin deploy_testnet --features livenet

# Deployment steps:
# 1. Deploys contract (~18M gas)
# 2. Registers first test merchant
# 3. Verifies initialization
# 4. Shows contract address

# Expected output:
# Contract deployed successfully!
# Contract address: contract-package-...
# Transaction: https://testnet.cspr.live/transaction/...
```

**Save your contract address:**
```bash
# Copy the contract-package-... hash
# Add to backend .env:
CASPER_CONTRACT_HASH=contract-package-811a7609239faa0d5b52552b366c0cc325f049109e8833f3d564ac98071ba5d5
```

## Project Structure

```
├── bin/
│   └── deploy_testnet.rs    # Deployment script
├── src/
│   ├── lib.rs               # Library root
│   ├── caspay.rs            # Main contract
│   ├── types.rs             # Data structures
│   ├── storage.rs           # State management
│   ├── events.rs            # Contract events
│   └── errors.rs            # Error definitions
├── tests/
│   └── integration_tests.rs # Integration tests
└── wasm/                    # Compiled WASM files
```

## Testing

```bash
# Run all tests with Odra
cargo odra test

# Run only integration tests
cargo test --test integration_tests

# Run specific test
cargo test --test integration_tests test_register_merchant

# Watch mode (requires cargo-watch)
cargo watch -x "odra test"
```

### Test Results

```
✅ 23/23 tests passing

📦 Test Coverage:
  ✓ Initialization (1 test)
  ✓ Merchant Management (4 tests)
  ✓ Product Management (3 tests)
  ✓ Subscription Plans (2 tests)
  ✓ Subscription Lifecycle (4 tests)
  ✓ Payment Recording (2 tests)
  ✓ Authorization & Access Control (7 tests)
```

## Network Info

- **Chain:** casper-test
- **Explorer:** https://testnet.cspr.live/
- **Faucet:** https://testnet.cspr.live/tools/faucet

## Documentation

- [Odra Framework](https://odra.dev/docs)
- [Casper Documentation](https://docs.casper.network/)
- [CSPR.cloud API](https://docs.cspr.cloud/)

## License

MIT
