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

## Generating a Drop:

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