/*
 * SolanaGo - Decentralized Go AI on Solana Blockchain
 * Copyright (C) 2024 Your Organization. All rights reserved.
 */

use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    // Network Settings
    pub network: String,  // mainnet-beta, testnet, or devnet
    pub rpc_timeout_ms: u64,
    pub max_retries: u32,
    pub commitment_level: CommitmentLevel,

    // Program Settings
    pub program_id: Pubkey,
    pub model_account: Pubkey,
    pub state_account: Pubkey,
    
    // Transaction Settings
    pub compute_budget: u32,
    pub priority_fee: u64,
    pub max_tx_size: usize,

    // Rate Limiting
    pub rate_limit: RateLimit,
    pub cooldown_period: Duration,
    
    // Queue Settings
    pub queue_size: usize,
    pub batch_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub requests_per_second: u32,
    pub burst_size: u32,
    pub token_bucket_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    // AI Model Parameters
    pub model_type: ModelType,
    pub input_shape: Vec<usize>,
    pub output_shape: Vec<usize>,
    pub batch_size: usize,
    
    // Training Parameters
    pub learning_rate: f32,
    pub training_steps: u64,
    pub checkpoint_interval: u64,

    // Resource Limits
    pub max_compute_units: u32,
    pub max_memory_bytes: u64,
    
    // Validation
    pub validation_frequency: u32,
    pub accuracy_threshold: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelType {
    ValueNetwork,
    PolicyNetwork,
    Combined,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommitmentLevel {
    Processed,
    Confirmed,
    Finalized,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            network: "devnet".to_string(),
            rpc_timeout_ms: 30000,
            max_retries: 3,
            commitment_level: CommitmentLevel::Confirmed,
            program_id: Pubkey::default(),
            model_account: Pubkey::default(),
            state_account: Pubkey::default(),
            compute_budget: 200_000,
            priority_fee: 0,
            max_tx_size: 1232,
            rate_limit: RateLimit {
                requests_per_second: 10,
                burst_size: 20,
                token_bucket_size: 100,
            },
            cooldown_period: Duration::from_secs(60),
            queue_size: 1000,
            batch_size: 16,
        }
    }
}

impl NetworkConfig {
    pub fn mainnet() -> Self {
        Self {
            network: "mainnet-beta".to_string(),
            commitment_level: CommitmentLevel::Finalized,
            compute_budget: 1_400_000,
            priority_fee: 1000,
            ..Default::default()
        }
    }

    pub fn testnet() -> Self {
        Self {
            network: "testnet".to_string(),
            ..Default::default()
        }
    }

    pub fn with_program_id(mut self, program_id: Pubkey) -> Self {
        self.program_id = program_id;
        self
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.queue_size == 0 {
            return Err(ConfigError::InvalidQueueSize);
        }
        if self.batch_size == 0 || self.batch_size > 64 {
            return Err(ConfigError::InvalidBatchSize);
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Invalid queue size")]
    InvalidQueueSize,
    #[error("Invalid batch size (must be between 1 and 64)")]
    InvalidBatchSize,
    #[error("Invalid compute budget")]
    InvalidComputeBudget,
}
