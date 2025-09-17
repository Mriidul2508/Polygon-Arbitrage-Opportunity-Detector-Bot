# Polygon Arbitrage Opportunity Detector Bot

[cite_start]This is a Rust bot that detects potential arbitrage opportunities on the Polygon network[cite: 1, 3]. [cite_start]It periodically checks the prices of a specific token pair on two different Decentralized Exchanges (DEXes)[cite: 6]. [cite_start]If a profitable opportunity is found (after accounting for simulated gas costs), it logs the details to the console[cite: 7, 10].



## 🎯 Project Goal

The goal is to design and implement a Rust application that can:
- [cite_start]Fetch token prices from multiple DEXes on Polygon (e.g., QuickSwap, SushiSwap)[cite: 12, 19].
- [cite_start]Compare these prices to identify arbitrage opportunities[cite: 13].
- [cite_start]Calculate the simulated profit for a fixed trade size, considering a simplified gas cost[cite: 14].
- [cite_start]Log any profitable opportunities found[cite: 7].

## 🛠️ Technology Stack

- [cite_start]**Language**: Rust [cite: 22]
- [cite_start]**Blockchain**: Polygon Network [cite: 18]
- [cite_start]**DEX Interaction**: Ethers-rs library to interact with Uniswap V2-style router contracts[cite: 20].
- [cite_start]**Tokens**: WETH and USDC on Polygon[cite: 21].

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) toolchain
- A Polygon RPC URL from a provider like Alchemy, Infura, or Ankr.

### Configuration

1.  **Create the `.env` file**:
    Create a file named `.env` in the project's root directory and add your Polygon RPC URL.

    ```dotenv
    POLYGON_RPC_URL="YOUR_POLYGON_RPC_URL_HERE"
    ```

2.  **Review `config/settings.toml`**:
    [cite_start]This file contains the addresses for DEX routers, tokens, and other parameters like the trade amount and profit threshold. The default values are set for WETH/USDC on QuickSwap and SushiSwap on Polygon.

### Installation & Running the Bot

1.  **Clone the repository**:
    ```sh
    git clone <your-repo-url>
    cd <your-repo-name>
    ```

2.  **Build the project**:
    ```sh
    cargo build --release
    ```

3.  **Run the bot**:
    ```sh
    cargo run --release
    ```

The bot will start checking for arbitrage opportunities every 30 seconds (configurable in `settings.toml`) and will print any profitable finds to your console.
