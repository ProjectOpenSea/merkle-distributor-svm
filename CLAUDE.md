# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Build the entire workspace (includes all crates)
cargo build --release

# Build CLI tool specifically
cargo b -r

# Build Anchor program
anchor build

# Generate client libraries for Python tests
anchorpy client-gen target/idl/merkle_distributor.json ./test/client_py --program-id [PROGRAM_ID]
```

## Test Commands

```bash
# Python tests (requires Python 3.10 + dependencies from test/requirements.txt)
# Must be run from programs/ directory after anchor build
cd programs/ && anchor build && cd ../test && python test.py

# Anchor tests (if using yarn/npm)
yarn mocha
```

## Architecture Overview

This is a Solana program for efficient token distribution using Merkle trees, similar to Uniswap's Merkle Distributor. The system allows airdropping tokens to thousands of recipients while minimizing on-chain storage costs by storing only a single Merkle root.

### Core Components

- **Anchor Program** (`programs/merkle-distributor/`): The on-chain Solana program that handles token claims
  - Main instructions: `new_distributor`, `new_claim`, `claim_locked`, `clawback`, `set_admin`
  - State accounts: `MerkleDistributor`, `ClaimStatus`
  - **Security Note**: The `new_distributor` instruction is susceptible to frontrunning - always verify the created distributor parameters match expectations

- **Merkle Tree Library** (`merkle-tree/`): Rust library for generating and managing Merkle trees from CSV data
  - Handles CSV parsing and tree construction
  - Provides utilities for proof generation

- **CLI Tool** (`cli/`): Command-line interface for claiming tokens
  - Primary usage: `./target/release/cli --rpc-url <RPC> --keypair-path <KEYPAIR> --airdrop-version <VERSION> --mint <MINT> --program-id <PROGRAM_ID> claim --merkle-tree-path merkle_tree.json`

- **API Server** (`api/`): REST API for serving Merkle proofs and distributor information
  - Axum-based web server
  - Provides endpoints for proof retrieval when `enable_proof_endpoint` is set

- **Verification Library** (`verify/`): Utilities for verifying Merkle proofs

### Key Configuration Files

- `Anchor.toml`: Anchor workspace configuration with program addresses for different networks
- `Cargo.toml`: Workspace configuration with all crate dependencies
- Program ID: `mERKcfxMC5SqJn4Ld4BUris3WKZZ1ojjWJ3A3J5CKxv` (consistent across all networks)

### Vesting and Claims

The system supports both immediate claims and time-locked vesting. Claims are processed through Merkle proofs, and the program handles partial claims and clawback functionality for unclaimed tokens after a specified time period.