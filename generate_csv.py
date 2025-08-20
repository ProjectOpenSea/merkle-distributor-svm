#!/usr/bin/env python3

import csv
import random
from solders.keypair import Keypair

def generate_random_pubkey():
    """Generate a random Solana public key"""
    keypair = Keypair()
    return str(keypair.pubkey())

def main():
    filename = "airdrop_100k.csv"
    num_wallets = 100000
    
    # Your specific wallet should get 10 USDC
    target_wallet = "F2pKXoQ6BL2Ex1PbDaKBv2DrNBgSvRRx8mTQU9ssmAqb"
    target_amount = 10
    
    categories = ["Staker", "Validator", "Searcher"]
    
    with open(filename, 'w', newline='') as csvfile:
        writer = csv.writer(csvfile)
        
        # Write header
        writer.writerow(["pubkey", "amount_unlocked", "amount_locked", "category"])
        
        # Write your specific wallet first
        writer.writerow([target_wallet, target_amount, 0, "Staker"])
        
        # Generate remaining wallets
        for i in range(num_wallets - 1):
            pubkey = generate_random_pubkey()
            
            # Random amounts between 1-1000 USDC
            amount_unlocked = random.randint(1, 1000)
            amount_locked = random.randint(0, 500)
            category = random.choice(categories)
            
            writer.writerow([pubkey, amount_unlocked, amount_locked, category])
            
            if (i + 1) % 10000 == 0:
                print(f"Generated {i + 1} wallets...")
    
    print(f"Generated {filename} with {num_wallets} wallets")
    print(f"Target wallet {target_wallet} can claim {target_amount} USDC")

if __name__ == "__main__":
    main()