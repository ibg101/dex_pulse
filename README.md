## DEX Pulse - Solana Liquidity Pool Scanner

**DEX Pulse** is a highly optimized scanner for new liquidity pools on the **Solana blockchain**. 

Built with **Rust**, this tool leverages the language’s performance to ensure efficient detection of liquidity pool creation transactions, often before the transaction achieves a **finalized commitment**. The project is designed to be **lightweight** and **efficient**, with **minimal external dependencies**, providing **real-time** liquidity pool data for Solana-based decentralized exchanges (DEX).

## Features

- **Real-Time Liquidity Pool Detection**: DEX Pulse scans for newly created liquidity pools on the Solana blockchain and provides critical metadata in real-time.
- **Optimized Performance**: The bot is optimized using Rust’s performance capabilities, enabling early detection of transactions before they are finalized.
- **Custom Parsers**: The project uses custom-built parsers to analyze transaction instructions. This minimizes code size, reduces the reliance on external crates, and enhances performance by using **base58** encoding in RPC requests **instead of the heavier jsonParsed** format.
- **Custom Lightweight RpcClient**: This project uses a custom, lightweight RpcClient, which allows for minimal overhead and maximum performance. Unlike most projects, it does not rely on the Solana official crates, which ensures better control and efficiency when interacting with Solana RPC.
- **No External Services or Aggregators**: This project doesn't rely on aggregators or other third-party services, except for plain **Solana RPC**. This ensures maximum control, security, and reliability.
- **Multi-DEX Support**: The architecture is designed to support unlimited DEX platforms. Currently, **Raydium** and **Meteora** are implemented, with plans to extend support to other platforms in the future.
- **Telegram Integration**: The bot can post real-time liquidity pool data to a [Telegram channel](https://t.me/dex_pulse_scanner), providing insights on new pools immediately after they are created. Additionally, you can configure the bot to send updates to **your own Telegram channel**. To do this, simply set the `CHANNEL_USERNAME` variable in the `.env` file to `@your_channel` (replace with your actual Telegram channel username). This requires setting up the project locally or [getting in touch with me](https://t.me/ivn_bets) to configure it for you.

## Metadata Provided

For every new liquidity pool detected, the bot extracts and displays the following useful information:

- **Market ID**: The address of the newly created liquidity pool.
- **Creator Wallets**: The addresses of the creators (DEVs) of the liquidity pool.
- **Mint Addresses**: The addresses of the base and quote token mints.
- **Liquidity Information**:
  - **Provided Liquidity Percentage**: Calculated using the formula `provided / supply * 100`, indicating how much of the total supply has been added to the pool.
  - **Provided Liquidity Amount**: Displays the amount of SOL or USDC pooled in the liquidity pool.
- **Token Authority Information**:
  - **Mint Authority**: The address responsible for minting new tokens.
  - **Freeze Authority**: The address that can freeze the bought tokens.
- **Raydium-Specific Data**: For pools created through Raydium DEX, additional fields are provided:
  - **LP Token Mint**: The mint address of the LP token, which will be used in future ws burn subscriptions in order to track locked liquidity.
  - **LP Token Minted Amount**: The amount of LP tokens minted.

## Architecture

- **Rust-Based**: The project is written in Rust to leverage its performance and memory safety features.
- **Custom Transaction Parsers**: Transaction instruction parsers are custom-built and located in the utils/parser/ directory. These parsers handle the analysis of transactions without relying on external libraries, which reduces overhead and increases speed.
- **Custom Lightweight RpcClient**: The project uses a custom, lightweight RpcClient and located in the rpc/ directory, which avoids the need for Solana crates and provides minimal overhead for maximum performance when interacting with Solana RPC.
- **Support for Multiple DEXs**: While the current implementation supports Raydium and Meteora, the architecture allows for easy extension to additional Solana-based DEX platforms in the future.
- **Modular Architecture**: The project is divided into several modular components, each designed to handle specific tasks. This structure allows for clean separation of concerns and scalability.
  - **Bot** Module: The heart of the project, responsible for elegantly connecting all functions and orchestrating the flow of data between different components.
  - **Observations** Module: Contains the logic for filtering WebSocket (WS) events and orchestrator functions that handle WSS.
  - **Processing** Module: Handles transaction processing and constructs the PairMeta structure, which contains all the metadata related to liquidity pools.
  - **RPC** Module:
    - **HTTP** Module: Contains the RPC HTTP methods used to interact with Solana nodes.
    - **WS** Module: Contains the WebSocket methods for listening to real-time updates from the Solana network.
  - **Types** Module: Contains structs, enums, traits, and error definitions used across the project.
  - **Utils** Module: Contains common utilities frequently used throughout the project.

## Setup & Installation

1. **Clone the repository**:
```
git clone https://github.com/ibg101/dex_pulse.git
cd dex_pulse
```
2. **Install dependencies**:

This project uses minimal external crates, but make sure you have **Rust** installed on your machine.
After installing Rust, run:
```
cargo build --release
```
3. **Configure the `.env` file**:
todo!

4. **Running the Bot**:

You can run the bot with:
```
cargo run --release
```
The bot will start scanning for new liquidity pools and post real-time updates to the configured **Telegram channel**.
