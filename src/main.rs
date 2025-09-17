use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use ethers::core::utils::parse_units;
use eyre::Result;
use std::sync::Arc;
use std::time::Duration;
use serde::Deserialize;

// Generate typesafe bindings to the Uniswap V2 Router ABI
abigen!(
    IUniswapV2Router02,
    "./src/abi/IUniswapV2Router02.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// Configuration structs to hold settings from settings.toml
#[derive(Debug, Deserialize)]
struct Tokens {
    weth: Address,
    usdc: Address,
    decimals_a: u32,
    decimals_b: u32,
}

#[derive(Debug, Deserialize)]
struct Dex {
    name: String,
    router_address: Address,
}

#[derive(Debug, Deserialize)]
struct Settings {
    check_interval_seconds: u64,
    minimum_profit_threshold: f64,
    amount_in: f64,
    simulated_gas_cost_usdc: f64,
    tokens: Tokens,
    dexes: Vec<Dex>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    println!("Starting Polygon Arbitrage Bot...");

    // 1. CONFIGURATION MANAGEMENT 
    let config_builder = config::Config::builder()
        .add_source(config::File::with_name("./config/settings"))
        .build()?;
    let settings: Settings = config_builder.try_deserialize()?;

    if settings.dexes.len() < 2 {
        panic!("Configuration must include at least two DEXes.");
    }
    
    // 2. CONNECT TO POLYGON RPC NODE [cite: 12]
    let rpc_url = std::env::var("POLYGON_RPC_URL")?;
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let client = Arc::new(provider);

    // Create contract instances for the two DEXes
    let dex_a = &settings.dexes[0];
    let dex_b = &settings.dexes[1];
    let contract_a = IUniswapV2Router02::new(dex_a.router_address, client.clone());
    let contract_b = IUniswapV2Router02::new(dex_b.router_address, client.clone());

    // Main application loop
    let mut interval = tokio::time::interval(Duration::from_secs(settings.check_interval_seconds));
    loop {
        interval.tick().await;
        println!("\nChecking for arbitrage opportunities...");
        
        // Use a fixed amount of Token A (WETH) for the simulation
        let amount_in = parse_units(settings.amount_in, settings.tokens.decimals_a)?.into();

        // 3. MULTI-DEX PRICE FETCHING [cite: 12]
        let price_on_dex_a = get_price(&contract_a, amount_in, &settings.tokens).await;
        let price_on_dex_b = get_price(&contract_b, amount_in, &settings.tokens).await;

        if let (Ok(price_a), Ok(price_b)) = (price_on_dex_a, price_on_dex_b) {
            println!("Price on {}: 1 WETH -> {:.4} USDC", dex_a.name, price_a);
            println!("Price on {}: 1 WETH -> {:.4} USDC", dex_b.name, price_b);

            // 4. ARBITRAGE OPPORTUNITY DETECTION [cite: 13] & PROFIT CALCULATION [cite: 14]
            check_opportunity(&settings, dex_a, dex_b, price_a, price_b);
        } else {
            eprintln!("Error fetching prices from one or both DEXes.");
        }
    }
}

/// Fetches the price of Token A in terms of Token B from a single DEX
async fn get_price(contract: &IUniswapV2Router02<Provider<Http>>, amount_in: U256, tokens: &Tokens) -> Result<f64> {
    let path = vec![tokens.weth, tokens.usdc];
    let amounts_out = contract.get_amounts_out(amount_in, path).call().await?;
    
    // The second element in the returned array is the output amount
    if let Some(amount) = amounts_out.get(1) {
        // Convert from WEI/Satoshi format to a readable float
        Ok(amount.as_u128() as f64 / 10f64.powi(tokens.decimals_b as i32))
    } else {
        Err(eyre::eyre!("Could not get amount out from DEX"))
    }
}

/// Compares prices and logs a potential arbitrage opportunity if one exists
fn check_opportunity(settings: &Settings, dex_a: &Dex, dex_b: &Dex, price_a: f64, price_b: f64) {
    // Opportunity: Buy on the cheaper DEX, sell on the more expensive one
    let (buy_dex, sell_dex, buy_price, sell_price) = if price_a < price_b {
        (dex_a, dex_b, price_a, price_b)
    } else {
        (dex_b, dex_a, price_b, price_a)
    };
    
    let gross_profit = (sell_price - buy_price) * settings.amount_in;
    
    // 5. SIMULATED PROFIT CALCULATION (including gas cost) [cite: 10]
    let simulated_profit = gross_profit - settings.simulated_gas_cost_usdc;

    if simulated_profit > settings.minimum_profit_threshold {
        println!("\n!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        println!("!!! Arbitrage Opportunity Detected!           !!!");
        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        println!("  - Action: BUY {} WETH on {}", settings.amount_in, buy_dex.name);
        println!("  - Action: SELL {} WETH on {}", settings.amount_in, sell_dex.name);
        println!("  - Est. Gross Profit: {:.4} USDC", gross_profit);
        println!("  - Simplified Gas Cost: -{:.4} USDC", settings.simulated_gas_cost_usdc);
        println!("  - SIMULATED NET PROFIT: {:.4} USDC", simulated_profit);
        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n");
    }
}
