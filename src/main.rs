use clap::{CommandFactory, Parser};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(name = "blocktimes", about = "Fetch Ethereum block timestamps")]
struct Args {
    /// Ethereum JSON-RPC endpoint (can also be set via ETH_RPC_URL)
    #[arg(long, env = "ETH_RPC_URL")]
    rpc_url: String,

    /// Starting block number (decimal)
    #[arg(long)]
    start_block: u64,

    /// Number of blocks to fetch
    #[arg(long)]
    block_count: u64,

    /// Interval between sampled blocks
    #[arg(long, default_value_t = 1)]
    block_interval: u64,
}

#[derive(Serialize)]
struct JsonRpcRequest<'a, T> {
    jsonrpc: &'static str,
    id: u64,
    method: &'a str,
    params: T,
}

#[derive(Deserialize)]
struct JsonRpcResponse<T> {
    result: T,
}

#[derive(Deserialize)]
struct BlockResult {
    number: String,
    timestamp: String,
}

struct BlockInfo {
    number: u64,
    timestamp: u64,
    timestamp_utc: String,
}

fn parse_hex_u64(input: &str) -> Result<u64, Box<dyn Error>> {
    let trimmed = input.strip_prefix("0x").unwrap_or(input);
    Ok(u64::from_str_radix(trimmed, 16)?)
}

async fn fetch_block(
    client: &reqwest::Client,
    rpc_url: &str,
    block_number: u64,
) -> Result<BlockInfo, Box<dyn Error>> {
    let params = json!([format!("0x{:x}", block_number), false]);
    let request = JsonRpcRequest {
        jsonrpc: "2.0",
        id: block_number,
        method: "eth_getBlockByNumber",
        params,
    };

    let response: JsonRpcResponse<Option<BlockResult>> = client
        .post(rpc_url)
        .json(&request)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let result = response
        .result
        .ok_or("block not found for requested number")?;

    let number = parse_hex_u64(&result.number)?;
    let timestamp = parse_hex_u64(&result.timestamp)?;
    let date_time = DateTime::<Utc>::from_timestamp(timestamp as i64, 0)
        .ok_or("invalid block timestamp")?;

    Ok(BlockInfo {
        number,
        timestamp,
        timestamp_utc: date_time.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if std::env::args().len() == 1 {
        let mut cmd = Args::command();
        cmd.print_help()?;
        println!();
        return Ok(());
    }

    let args = Args::parse();
    let client = reqwest::Client::new();

    let mut previous_timestamp: Option<u64> = None;
    for offset in 0..args.block_count {
        let block_number = args.start_block + offset * args.block_interval;
        let info = fetch_block(&client, &args.rpc_url, block_number).await?;
        let delta = match previous_timestamp {
            Some(prev) => info.timestamp.saturating_sub(prev).to_string(),
            None => String::new(),
        };
        println!(
            "{},{},{},{}",
            info.number, info.timestamp, info.timestamp_utc, delta
        );
        previous_timestamp = Some(info.timestamp);
        tokio::time::sleep(Duration::from_millis(1)).await;
    }

    Ok(())
}
