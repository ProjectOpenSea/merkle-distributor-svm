# merkle-distributor

A program for distributing tokens efficiently via uploading a [Merkle root](https://en.wikipedia.org/wiki/Merkle_tree).

## Claiming Airdrop via CLI

To claim via CLI instead of using `https://jito.network/airdrop`, run the following commands.

1. Build the cli (must have rust + cargo installed):

```bash
cargo b -r
```

2. Run `claim` with the proper args. Be sure to replace `<YOUR KEYPAIR>` with the _full path_ of your keypair file. This will transfer tokens from the account `8Xm3tkQH581s3MoRHWUNYA5jKbgPATW4tJAAxgwDC6T6` to a the associated token account owned by your keypair, creating it if it doesn't exist.

```bash
./target/release/cli --rpc-url https://api.mainnet-beta.solana.com --keypair-path <YOUR KEYPAIR> --airdrop-version 0 --mint jtojtomepa8beP8AuQc6eXt5FriJwfFMwQx2v2f9mCL --program-id mERKcfxMC5SqJn4Ld4BUris3WKZZ1ojjWJ3A3J5CKxv claim --merkle-tree-path merkle_tree.json
```

Note that for searchers and validators, not all tokens will be vested until December 7, 2024. You can check the vesting status at `https://jito.network/airdrop`.

## Deploying a New Distributor to Existing Program

Follow these steps to deploy a new USDC distributor to the existing program at `mERKcfxMC5SqJn4Ld4BUris3WKZZ1ojjWJ3A3J5CKxv`:

### Prerequisites
1. Install Solana CLI tools: https://solana.com/docs/intro/installation
2. Build the CLI tool: `cargo build --release`
3. Acquire USDC tokens from the market (total amount = sum of all amounts in your CSV)

### Deployment Steps

1. **Create your recipients CSV file** in this format:
   ```csv
   pubkey,amount_unlocked,amount_locked,category
   <recipient_pubkey_1>,<unlocked_amount>,<locked_amount>,<category>
   <recipient_pubkey_2>,<unlocked_amount>,<locked_amount>,<category>
   ```

   ```bash
   python generate_csv.py
   ```

2. **Generate the Merkle tree from your CSV:**
   ```bash
   ./target/release/cli \
    --mint EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v \
    --rpc-url https://api.mainnet-beta.solana.com \
    --keypair-path  ~/.config/solana/id.json \
    create-merkle-tree \
    --csv-path airdrop_100k.csv \
    --merkle-tree-path merkle_tree.json
   ```

   ```bash
   spl-token create-account EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
   ```

3. **Create the new distributor on-chain:**
   ```bash
   ./target/release/cli \
      --rpc-url https://api.mainnet-beta.solana.com \
      --keypair-path ~/.config/solana/id.json \
      --mint EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v \
      --program-id mERKcfxMC5SqJn4Ld4BUris3WKZZ1ojjWJ3A3J5CKxv \
      --airdrop-version 422 \ # a unique version (identifier)
      new-distributor \
      --clawback-receiver-token-account CZZZeKxrQRPjwAA7REZWqsADChFY7P41K2wNmRWT779U \
      --start-vesting-ts 1756239698 \
      --end-vesting-ts 1756239699 \
      --clawback-start-ts 1756329899 \
      --merkle-tree-path merkle_tree.json
   ```
   
   **Parameters:**
   - `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v` = USDC mint address (mainnet)
   - Timestamps are Unix timestamps (seconds since epoch)  
   - `clawback-start-ts` must be at least 1 day after `end-vesting-ts`
   - The CLI will output the distributor vault address where you need to send tokens

4. **Transfer USDC tokens to the distributor vault:**
   
   Get the vault address from the newDistributor program output, bottom of page:
   https://solscan.io/tx/2iHVJ8yFAhohEYzXWUedZjji99Uhrhf5qKXzYKxaqPu7cNYg4E7LKNUWoc6GiXKWgjyBfMzJUpLYBTKjMwzwhre6

   ```bash
   spl-token transfer EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v \
     <TOTAL_AMOUNT> \
     <DISTRIBUTOR_VAULT_ADDRESS> \
     --fund-recipient
   ```
   
   **Note:** `<DISTRIBUTOR_VAULT_ADDRESS>` is printed by the previous command and `<TOTAL_AMOUNT>` is the sum of all `amount_unlocked` + `amount_locked` from your CSV.

5. **Test claiming tokens (optional):**
   ```bash
    ./target/release/cli \
    --rpc-url https://api.mainnet-beta.solana.com \
    --keypair-path ~/.config/solana/id.json \
    --mint EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v \
    --program-id mERKcfxMC5SqJn4Ld4BUris3WKZZ1ojjWJ3A3J5CKxv \
    --airdrop-version 422 \
    claim \
    --merkle-tree-path merkle_tree.json
   ```

### Important Security Notes
- The `new_distributor` instruction is susceptible to frontrunning
- Always verify the created distributor parameters match your expectations after the transaction succeeds
- Double-check the vault address before transferring large amounts of USDC

## Original Development Setup:

0. https://solana.com/docs/intro/installation
0. `avm install 0.30.1 && avm use 0.30.1`
1. `cargo build --release`
2. `solana-keygen new --outfile ~/.config/solana/id.json`
3. `solana airdrop 2 --url https://api.devnet.solana.com`
4. `cat ~/.config/solana/id.json`
5. Import private key to Phantom 
6. Swap for dusdc on Raydium https://raydium.io/swap/
7. `python generate_csv.py` 
8. `./target/release/cli --mint USDCoctVLVnvTXBEuP9s8hntucdJokbo17RwHuNXemT --rpc-url https://api.devnet.solana.com --keypair-path ~/.config/solana/id.json create-merkle-tree --csv-path airdrop_100k.csv --merkle-tree-path merkle_tree_100k.json`
9. ` RUSTUP_TOOLCHAIN=nightly-2025-04-01 anchor build`
10. `./target/release/cli --rpc-url https://api.devnet.solana.com --keypair-path ~/.config/solana/id.json --airdrop-version 0 --mint USDCoctVLVnvTXBEuP9s8hntucdJokbo17RwHuNXemT --program-id APEosY9z2WBL8ZXj9V7L2RPwpeKb3czc3N6qqMsubuok new-distributor --clawback-receiver-token-account HViKVWxAHVeFC3ptn83xLiHYrhMgNteUSK8GqHmDjRbv --start-vesting-ts 1755720310 --end-vesting-ts 1755720311 --merkle-tree-path merkle_tree_100k.json --clawback-start-ts 1755820312`
11. `anchor deploy --provider.cluster devnet`

12. `spl-token address --owner E2RL3eYMRAvvtLUNwrAE3bPuJoNTR2AvzqLSMJcbeNfB --token USDCoctVLVnvTXBEuP9s8hntucdJokbo17RwHuNXemT --verbose`
13. `spl-token transfer USDCoctVLVnvTXBEuP9s8hntucdJokbo17RwHuNXemT 10 E2RL3eYMRAvvtLUNwrAE3bPuJoNTR2AvzqLSMJcbeNfB --allow-non-system-account-recipient`