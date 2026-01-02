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

- Rust toolchain
- cargo-odra (`cargo install cargo-odra`)
- CSPR.cloud API key ([Get one here](https://console.cspr.build/))
- Casper testnet account with CSPR

### Setup

1. **Configure environment:**

```bash
cp .env.example .env
# Edit .env with your CSPR.cloud API token:
# Get from: https://console.cspr.build/
```

2. **Run tests:**

```bash
cargo odra test
```

3. **Build contract:**

```bash
cargo odra build
```

4. **Deploy to testnet:**

```bash
# Terminal 1: Start proxy (required for deployment)
node cspr_proxy.js

# Terminal 2: Deploy contract
cargo run --bin deploy_testnet --features=livenet
```

> **Note:** The proxy script (`cspr_proxy.js`) is required because Odra doesn't support custom HTTP headers natively. It adds CSPR.cloud authentication automatically.

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
# Run all tests
cargo odra test

# Run specific test
cargo test test_merchant_registration

# Watch mode
cargo watch -x "odra test"
```

### Test Results

```
✅ 21/21 tests passing

📦 Test Coverage:
  ✓ Initialization (1 test)
  ✓ Merchant Management (3 tests)
  ✓ Product Management (2 tests)
  ✓ Subscription Plans (1 test)
  ✓ Payment Recording (2 tests)
  ✓ Subscriptions (3 tests)
  ✓ Subscription Queries (2 tests)
  ✓ Authorization & Access Control (6 tests)
  ✓ Admin Controls (1 test)
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
