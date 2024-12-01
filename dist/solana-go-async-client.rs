/*
 * SolanaGo - Decentralized Go AI on Solana Blockchain
 * Copyright (C) 2024 Your Organization. All rights reserved.
 */

use solana_sdk::{
    account::Account,
    pubkey::Pubkey,
    transaction::Transaction,
};
use tokio::sync::{mpsc, Mutex};
use std::{
    collections::HashMap,
    sync::Arc,
    time::Duration,
};

/// AsyncModelClient manages distributed AI model inference across the Solana network
pub struct AsyncModelClient {
    // Network Configuration
    network_nodes: Vec<String>,
    config: NetworkConfig,
    
    // RPC Clients
    rpc_clients: Vec<RpcClient>,
    available_nodes: Arc<Mutex<Vec<usize>>>,
    
    // Transaction Processing
    tx_queue: mpsc::Sender<Transaction>,
    completion_queue: mpsc::Receiver<ModelResponse>,
    
    // Rate Limiting
    rate_limiters: Vec<RateLimiter>,
    node_mutexes: Vec<Mutex<()>>,
}

impl AsyncModelClient {
    pub async fn new(nodes: Vec<String>, config: NetworkConfig) -> Self {
        let mut clients = Vec::new();
        for node in &nodes {
            clients.push(RpcClient::new_with_timeout(
                node.clone(),
                Duration::from_millis(config.timeout_ms),
            ));
        }

        let (tx_sender, tx_receiver) = mpsc::channel(config.queue_size);
        let (completion_sender, completion_receiver) = mpsc::channel(config.queue_size);

        let client = Self {
            network_nodes: nodes,
            config,
            rpc_clients: clients,
            available_nodes: Arc::new(Mutex::new((0..nodes.len()).collect())),
            tx_queue: tx_sender,
            completion_queue: completion_receiver,
            rate_limiters: vec![RateLimiter::new(config.rate_limit); nodes.len()],
            node_mutexes: vec![Mutex::new(()); nodes.len()],
        };

        // Start processing thread
        tokio::spawn(client.process_transactions(tx_receiver, completion_sender));

        client
    }

    pub async fn init_model(&self, model_config: ModelConfig) -> Result<(), ClientError> {
        let init_ix = model_config.to_instruction();
        let recent_blockhash = self.get_recent_blockhash().await?;
        
        for (i, client) in self.rpc_clients.iter().enumerate() {
            let tx = Transaction::new_signed_with_payer(
                &[init_ix.clone()],
                Some(&self.config.payer),
                &[&self.config.payer],
                recent_blockhash,
            );

            match client.send_and_confirm_transaction(&tx).await {
                Ok(_) => log::info!("Initialized model on node {}", self.network_nodes[i]),
                Err(e) => {
                    log::error!("Failed to init node {}: {}", self.network_nodes[i], e);
                    return Err(ClientError::InitializationError);
                }
            }
        }

        Ok(())
    }

    pub async fn predict(&self, input: Vec<bool>) -> Result<ModelOutput, ClientError> {
        let node_id = self.get_available_node().await?;
        
        let predict_ix = self.build_predict_instruction(&input);
        let tx = Transaction::new_signed_with_payer(
            &[predict_ix],
            Some(&self.config.payer),
            &[&self.config.payer],
            self.get_recent_blockhash().await?,
        );

        self.tx_queue.send(tx).await?;

        match self.completion_queue.recv().await {
            Some(response) => {
                self.release_node(node_id).await;
                Ok(response.output)
            }
            None => {
                self.disable_node(node_id).await;
                Err(ClientError::PredictionFailed)
            }
        }
    }

    async fn process_transactions(
        &self,
        mut rx: mpsc::Receiver<Transaction>,
        completion_sender: mpsc::Sender<ModelResponse>,
    ) {
        while let Some(tx) = rx.recv().await {
            let node_id = self.get_available_node().await.unwrap();
            let client = &self.rpc_clients[node_id];

            match client.send_and_confirm_transaction(&tx).await {
                Ok(sig) => {
                    // Get account data and deserialize model output
                    if let Ok(output) = self.get_model_output(sig).await {
                        completion_sender.send(ModelResponse { output }).await.unwrap();
                    }
                }
                Err(e) => {
                    log::error!("Transaction failed on node {}: {}", self.network_nodes[node_id], e);
                    self.disable_node(node_id).await;
                }
            }
        }
    }

    // Node management methods
    async fn get_available_node(&self) -> Result<usize, ClientError> {
        let mut nodes = self.available_nodes.lock().await;
        nodes.pop().ok_or(ClientError::NoNodesAvailable)
    }

    async fn release_node(&self, node_id: usize) {
        let mut nodes = self.available_nodes.lock().await;
        nodes.push(node_id);
    }

    async fn disable_node(&self, node_id: usize) {
        log::warn!("Disabling node {} due to errors", self.network_nodes[node_id]);
        
        let limiter = &self.rate_limiters[node_id];
        let _lock = self.node_mutexes[node_id].lock().await;
        
        tokio::spawn(async move {
            limiter.wait_for_capacity().await;
            // Re-enable node after cooldown
            self.release_node(node_id).await;
        });
    }
}
