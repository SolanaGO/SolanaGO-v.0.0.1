/*
 * SolanaGo - Decentralized Go AI on Solana Blockchain
 * Copyright (C) 2024 Your Organization. All rights reserved.
 */

use tokio::{
    sync::{mpsc, oneshot},
    time::{timeout, Duration},
};
use solana_sdk::{
    transaction::Transaction,
    signature::Signature,
};
use std::{
    sync::Arc,
    collections::HashMap,
};

/// Asynchronous RPC queue for handling blockchain transactions
pub struct AsyncRpcQueue {
    sender: mpsc::Sender<RpcRequest>,
    pending_requests: Arc<tokio::sync::Mutex<HashMap<Signature, oneshot::Sender<RpcResponse>>>>,
    size: Arc<std::sync::atomic::AtomicUsize>,
    is_shutdown: Arc<std::sync::atomic::AtomicBool>,
}

#[derive(Debug)]
pub struct RpcRequest {
    tx: Transaction,
    response_sender: oneshot::Sender<RpcResponse>,
    timeout_ms: u64,
}

#[derive(Debug)]
pub struct RpcResponse {
    pub status: RpcStatus,
    pub signature: Signature,
    pub result: Option<Vec<u8>>,
}

#[derive(Debug)]
pub enum RpcStatus {
    Success,
    Timeout,
    Error(String),
}

impl AsyncRpcQueue {
    pub fn new(queue_size: usize) -> Self {
        let (sender, receiver) = mpsc::channel(queue_size);
        let pending_requests = Arc::new(tokio::sync::Mutex::new(HashMap::new()));
        let size = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let is_shutdown = Arc::new(std::sync::atomic::AtomicBool::new(false));

        // Start background processor
        tokio::spawn(Self::process_requests(
            receiver,
            Arc::clone(&pending_requests),
            Arc::clone(&size),
            Arc::clone(&is_shutdown),
        ));

        Self {
            sender,
            pending_requests,
            size,
            is_shutdown,
        }
    }

    pub async fn submit(&self, tx: Transaction, timeout_ms: u64) -> Result<RpcResponse, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        
        let request = RpcRequest {
            tx,
            response_sender,
            timeout_ms,
        };

        self.sender.send(request).await
            .map_err(|_| RpcError::QueueFull)?;

        self.size.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        match timeout(Duration::from_millis(timeout_ms), response_receiver).await {
            Ok(Ok(response)) => {
                self.size.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                Ok(response)
            }
            Ok(Err(_)) => Err(RpcError::ChannelClosed),
            Err(_) => Err(RpcError::Timeout),
        }
    }

    async fn process_requests(
        mut receiver: mpsc::Receiver<RpcRequest>,
        pending_requests: Arc<tokio::sync::Mutex<HashMap<Signature, oneshot::Sender<RpcResponse>>>>,
        size: Arc<std::sync::atomic::AtomicUsize>,
        is_shutdown: Arc<std::sync::atomic::AtomicBool>,
    ) {
        let rpc_client = Arc::new(RpcClient::new());

        while let Some(request) = receiver.recv().await {
            if is_shutdown.load(std::sync::atomic::Ordering::SeqCst) {
                break;
            }

            let signature = request.tx.signatures[0];
            
            // Store response channel
            pending_requests.lock().await.insert(signature, request.response_sender);

            // Submit transaction
            let client = Arc::clone(&rpc_client);
            let pending = Arc::clone(&pending_requests);
            
            tokio::spawn(async move {
                let result = client.send_and_confirm_transaction(&request.tx).await;
                let status = match result {
                    Ok(()) => RpcStatus::Success,
                    Err(e) => RpcStatus::Error(e.to_string()),
                };

                // Send response
                if let Some(sender) = pending.lock().await.remove(&signature) {
                    let _ = sender.send(RpcResponse {
                        status,
                        signature,
                        result: None,
                    });
                }

                size.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
            });
        }
    }

    pub fn shutdown(&self) {
        self.is_shutdown.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn size(&self) -> usize {
        self.size.load(std::sync::atomic::Ordering::SeqCst)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RpcError {
    #[error("RPC queue is full")]
    QueueFull,
    #[error("Request timed out")]
    Timeout,
    #[error("Response channel closed")]
    ChannelClosed,
}
