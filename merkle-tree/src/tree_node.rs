use std::str::FromStr;

use serde::{Deserialize, Serialize};
use solana_program::{hash::hashv, pubkey::Pubkey};
use solana_sdk::hash::Hash;

use crate::csv_entry::{AirdropCategory, CsvEntry};

/// Default decimal precision for standard USDC mints (6 decimals).
pub const DEFAULT_USDC_DECIMALS: u32 = 6;

/// Represents the claim information for an account in the Merkle tree.
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct TreeNode {
    /// Pubkey of the claimant; will be responsible for signing the claim
    pub claimant: Pubkey,
    /// Claimant's proof of inclusion in the Merkle Tree
    pub proof: Option<Vec<[u8; 32]>>,
    /// Total amount unlocked under staker allocation
    pub total_unlocked_staker: u64,
    /// Total amount locked under staker allocation
    pub total_locked_staker: u64,
    /// Total amount unlocked under searcher allocation
    pub total_unlocked_searcher: u64,
    /// Total amount locked under searcher allocation
    pub total_locked_searcher: u64,
    /// Total amount unlocked under validator allocation
    pub total_unlocked_validator: u64,
    /// Total amount locked under validator allocation
    pub total_locked_validator: u64,
}

impl TreeNode {
    /// Hash the tree node data for inclusion in the Merkle tree root computation.
    pub fn hash(&self) -> Hash {
        hashv(&[
            &self.claimant.to_bytes(),
            &self.amount_unlocked().to_le_bytes(),
            &self.amount_locked().to_le_bytes(),
        ])
    }

    /// Return total amount of locked and unlocked tokens for this claimant.
    pub fn total_amount(&self) -> u64 {
        self.amount_unlocked()
            .checked_add(self.amount_locked())
            .expect("Total token amount calculation overflowed u64")
    }

    /// Get total amount of unlocked tokens for this claimant.
    pub fn amount_unlocked(&self) -> u64 {
        self.total_unlocked_searcher
            .checked_add(self.total_unlocked_validator)
            .expect("Unlocked amount addition overflowed u64")
            .checked_add(self.total_unlocked_staker)
            .expect("Unlocked amount addition overflowed u64")
    }

    /// Get total amount of locked tokens for this claimant.
    pub fn amount_locked(&self) -> u64 {
        self.total_locked_searcher
            .checked_add(self.total_locked_validator)
            .expect("Locked amount addition overflowed u64")
            .checked_add(self.total_locked_staker)
            .expect("Locked amount addition overflowed u64")
    }

    /// Construct a `TreeNode` from a `CsvEntry` using explicit token decimal precision.
    pub fn from_csv_entry(entry: CsvEntry, decimals: u32) -> Self {
        let mut node = Self {
            claimant: Pubkey::from_str(entry.pubkey.as_str()).expect("Invalid claimant pubkey in CSV"),
            proof: None,
            total_unlocked_staker: 0,
            total_locked_staker: 0,
            total_unlocked_searcher: 0,
            total_locked_searcher: 0,
            total_unlocked_validator: 0,
            total_locked_validator: 0,
        };

        // CSV entry uses UI amounts; convert to native base units using specified decimals.
        let amount_unlocked = ui_amount_to_token_amount(entry.amount_unlocked, decimals);
        let amount_locked = ui_amount_to_token_amount(entry.amount_locked, decimals);

        match entry.category {
            AirdropCategory::Staker => {
                node.total_unlocked_staker = amount_unlocked;
                node.total_locked_staker = amount_locked;
            }
            AirdropCategory::Validator => {
                node.total_unlocked_validator = amount_unlocked;
                node.total_locked_validator = amount_locked;
            }
            AirdropCategory::Searcher => {
                node.total_unlocked_searcher = amount_unlocked;
                node.total_locked_searcher = amount_locked;
            }
        }
        node
    }
}

/// Converts a UI token amount to a native token base unit amount using target decimals.
/// Employs checked arithmetic to prevent potential overflow attacks.
pub fn ui_amount_to_token_amount(amount: u64, decimals: u32) -> u64 {
    let multiplier = 10u64
        .checked_pow(decimals)
        .expect("Decimal exponentiation overflowed u64");
    amount
        .checked_mul(multiplier)
        .expect("UI amount conversion multiplied by decimals overflowed u64")
}

impl From<CsvEntry> for TreeNode {
    /// Default fallback conversion assuming USDC decimal precision (6 decimals).
    fn from(entry: CsvEntry) -> Self {
        Self::from_csv_entry(entry, DEFAULT_USDC_DECIMALS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_tree_node() {
        let tree_node = TreeNode {
            claimant: Pubkey::default(),
            proof: None,
            total_unlocked_staker: 0,
            total_locked_staker: 0,
            total_unlocked_searcher: 0,
            total_locked_searcher: 0,
            total_unlocked_validator: 0,
            total_locked_validator: 0,
        };
        let serialized = serde_json::to_string(&tree_node).unwrap();
        let deserialized: TreeNode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(tree_node, deserialized);
    }

    #[test]
    fn test_ui_amount_to_token_amount() {
        let ui_amount = 5;

        // Test USDC 6-decimal scaling (5 USDC -> 5_000_000 base units)
        let usdc_amount = ui_amount_to_token_amount(ui_amount, 6);
        assert_eq!(usdc_amount, 5_000_000);

        // Test 9-decimal scaling for legacy tokens (5 SOL -> 5_000_000_000 base units)
        let legacy_amount = ui_amount_to_token_amount(ui_amount, 9);
        assert_eq!(legacy_amount, 5_000_000_000);
    }
}