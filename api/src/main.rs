mod error;
mod router;

use std::{fmt::Debug, net::SocketAddr, path::PathBuf, str::FromStr, sync::Arc};

use clap::Parser;
use jito_merkle_tree::{airdrop_merkle_tree::AirdropMerkleTree, utils::get_merkle_distributor_pda};
use router::RouterState;
use solana_program::pubkey::Pubkey;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use tracing::{info, instrument};

use crate::{error::ApiError, router::read_distributor};

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Bind address for the HTTP server
    #[clap(long, env, default_value_t = SocketAddr::from_str("0.0.0.0:7001").unwrap())]
    bind_addr: SocketAddr,

    /// Path to the Merkle tree data file
    #[clap(long, env)]
    merkle_tree_path: PathBuf,

    /// RPC endpoint URL (omitted from raw debug logs to protect potential provider API keys)
    #[clap(long, env)]
    rpc_url: String,

    /// Mint address of the distributed token
    #[clap(long, env)]
    mint: Pubkey,

    /// Program ID of the deployed distributor program
    #[clap(long, env)]
    program_id: Pubkey,

    /// Airdrop version identifier
    #[clap(long, env)]
    airdrop_version: u64,

    /// Enables the proof endpoint allowing users to check/claim allocations
    #[clap(long, env)]
    enable_proof_endpoint: bool,
}

#[tokio::main]
#[instrument]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    tracing_subscriber::fmt().init();

    // SECURITY FIX: Never log raw args or rpc_url directly because managed providers 
    // embed sensitive API keys in the URL query string.
    info!(
        "starting airdrop api service: bind_addr={}, version={}, airdrop_version={}, mint={}",
        args.bind_addr, args.airdrop_version, args.airdrop_version, args.mint
    );

    let rpc_client = RpcClient::new(args.rpc_url.clone());
    info!("initialized non-blocking rpc client successfully");

    let (merkle_distributor, _bump) =
        get_merkle_distributor_pda(&args.program_id, &args.mint, args.airdrop_version);

    let distributor = read_distributor(&rpc_client, &merkle_distributor).await?;

    info!("successfully loaded distributor account: {:?}", merkle_distributor);

    let state = Arc::new(RouterState {
        tree: AirdropMerkleTree::new_from_file(&args.merkle_tree_path)?.convert_to_hashmap(),
        program_id: args.program_id,
        distributor_pubkey: merkle_distributor,
        rpc_client,
    });

    let app = router::get_routes(state, args.enable_proof_endpoint);

    info!("starting server listener at {}", args.bind_addr);
    axum::Server::bind(&args.bind_addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}