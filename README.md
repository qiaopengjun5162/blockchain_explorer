# Blockchain Explorer

![Rust Version](https://img.shields.io/badge/rust-1.87%2B-orange)
![License: MIT](https://img.shields.io/badge/license-MIT-green)

A modular, async Rust blockchain explorer template, supporting multiple chains and providers (Alchemy, Infura, etc.), designed for extensibility and modern Rust best practices.

## Features

- 🌐 **Multi-chain support** (Ethereum, Polygon, zkSync, etc.)
- 🔌 **Pluggable provider architecture** (Alchemy, Infura, custom)
- ⚡ **Async, multi-threaded runtime** (Tokio)
- 🦀 **Modern Rust patterns** (builder, enums, error handling)
- 📝 **Environment-based configuration**
- 🧩 **Easy to extend for new chains/providers**

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70+ recommended)
- [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Node.js](https://nodejs.org/) (if you want to run a local Ethereum node for testing, optional)
- An [Alchemy](https://alchemy.com/) or [Infura](https://infura.io/) API key

### Installation

```bash
git clone https://github.com/yourusername/blockchain_explorer.git
cd blockchain_explorer
cargo build --release
```

### Configuration

Create a `.env` file in the project root:

```env
ALCHEMY_API_KEY=your_alchemy_api_key_here
```

You can get a free API key from [Alchemy](https://alchemy.com/) or [Infura](https://infura.io/).

## Usage

To run the explorer:

```bash
cargo run
```

**Example output:**

```
blockchain_explorer on  master [?] is 📦 0.1.0 via 🦀 1.87.0 
➜ cargo run  
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/blockchain_explorer`
[2025-05-23T13:02:29Z WARN  blockchain_explorer::app] Blockchain explorer application starting...
[2025-05-23T13:02:29Z INFO  blockchain_explorer::eth_connection_example] Generated URL: wss://eth-mainnet.g.alchemy.com/v2/xxx
Awaiting block headers...
Latest block number: 22545736
Latest block number: 22545737
Latest block number: 22545738
Latest block number: 22545739
[2025-05-23T13:03:13Z WARN  blockchain_explorer::app] Blockchain explorer application started successfully.
[2025-05-23T13:03:13Z INFO  blockchain_explorer] Server stopped
```

## Project Structure

```
src/
├── app.rs                  # Application entrypoint logic
├── main.rs                 # Main function, sets up logging and env
├── chains/
│   ├── chain_provider.rs   # Provider and connection type abstractions
│   └── constants.rs        # Chain/provider endpoint constants
├── eth_connection_example.rs # Example: connect and subscribe to blocks
```

## Supported Chains & Providers

- **Ethereum** (Mainnet, Sepolia, Holesky, Hoodi)
- **Polygon** (Mainnet, Amoy)
- **zkSync** (Mainnet, Sepolia)
- **Providers:** Alchemy, Infura, custom

## Extending

To add a new chain or provider:

1. Add constants in `src/chains/constants.rs`
2. Extend the `ProviderService` and `ConnectionType` enums in `src/chains/chain_provider.rs`
3. Use the builder pattern in `ChainProvider` to configure your new provider

## Dependencies

- [tokio](https://crates.io/crates/tokio) - Async runtime
- [dotenvy](https://crates.io/crates/dotenvy) - .env file loader
- [env_logger](https://crates.io/crates/env_logger) - Logging
- [alloy](https://crates.io/crates/alloy) - Ethereum provider abstraction
- [futures-util](https://crates.io/crates/futures-util) - Async utilities
- [anyhow](https://crates.io/crates/anyhow) - Error handling

## Contributing

Pull requests and issues are welcome!  
Please open an issue to discuss your idea before submitting a PR.

1. Fork the repo
2. Create your feature branch (`git checkout -b feature/foo`)
3. Commit your changes (`git commit -am 'Add foo'`)
4. Push to the branch (`git push origin feature/foo`)
5. Open a Pull Request

## License

MIT

---

**Enjoy hacking on your blockchain explorer! 🚀**
