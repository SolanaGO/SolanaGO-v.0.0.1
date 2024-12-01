/*
 * SolanaGo - Decentralized Go AI on Solana Blockchain
 * Copyright (C) 2024 Your Organization. All rights reserved.
 */

use std::{
    time::{Duration, Instant},
    sync::Arc,
};
use tokio::sync::Mutex;

/// Token bucket rate limiter for RPC calls and AI model inference
pub struct RateLimiter {
    tokens: Arc<Mutex<f64>>,
    last_update: Arc<Mutex<Instant>>,
    capacity: f64,
    refill_rate: f64,
    refill_interval: Duration,
}

impl RateLimiter {
    pub fn new(capacity: u32, refill_rate_per_sec: u32) -> Self {
        Self {
            tokens: Arc::new(Mutex::new(capacity as f64)),
            last_update: Arc::new(Mutex::new(Instant::now())),
            capacity: capacity as f64,
            refill_rate: refill_rate_per_sec as f64,
            refill_interval: Duration::from_millis(100), // 10 updates per second
        }
    }

    pub async fn acquire_token(&self) -> bool {
        let mut tokens = self.tokens.lock().await;
        let mut last_update = self.last_update.lock().await;
        
        self.refill_tokens(&mut tokens, &mut last_update).await;

        if *tokens >= 1.0 {
            *tokens -= 1.0;
            true
        } else {
            false
        }
    }

    pub async fn try_acquire_tokens(&self, count: u32) -> bool {
        let mut tokens = self.tokens.lock().await;
        let mut last_update = self.last_update.lock().await;
        
        self.refill_tokens(&mut tokens, &mut last_update).await;

        if *tokens >= count as f64 {
            *tokens -= count as f64;
            true
        } else {
            false
        }
    }

    async fn refill_tokens(&self, tokens: &mut f64, last_update: &mut Instant) {
        let now = Instant::now();
        let elapsed = now.duration_since(*last_update);
        
        let new_tokens = (elapsed.as_secs_f64() * self.refill_rate).min(self.capacity - *tokens);
        *tokens = (*tokens + new_tokens).min(self.capacity);
        *last_update = now;
    }

    pub async fn tokens_available(&self) -> f64 {
        let mut tokens = self.tokens.lock().await;
        let mut last_update = self.last_update.lock().await;
        
        self.refill_tokens(&mut tokens, &mut last_update).await;
        *tokens
    }

    pub async fn wait_for_token(&self) {
        loop {
            if self.acquire_token().await {
                break;
            }
            tokio::time::sleep(self.refill_interval).await;
        }
    }

    pub async fn wait_for_tokens(&self, count: u32) {
        loop {
            if self.try_acquire_tokens(count).await {
                break;
            }
            tokio::time::sleep(self.refill_interval).await;
        }
    }
}

/// Adaptive rate limiter that adjusts based on network conditions
pub struct AdaptiveRateLimiter {
    base_limiter: RateLimiter,
    error_counter: Arc<Mutex<ErrorCounter>>,
    backoff_multiplier: f64,
}

struct ErrorCounter {
    errors: Vec<Instant>,
    window_size: Duration,
    threshold: usize,
}

impl AdaptiveRateLimiter {
    pub fn new(capacity: u32, refill_rate: u32) -> Self {
        Self {
            base_limiter: RateLimiter::new(capacity, refill_rate),
            error_counter: Arc::new(Mutex::new(ErrorCounter {
                errors: Vec::new(),
                window_size: Duration::from_secs(60),
                threshold: 5,
            })),
            backoff_multiplier: 0.5,
        }
    }

    pub async fn acquire_token(&self) -> bool {
        let error_count = self.recent_error_count().await;
        if error_count > 0 {
            // Apply exponential backoff
            let backoff = self.backoff_multiplier.powi(error_count as i32);
            tokio::time::sleep(Duration::from_secs_f64(backoff)).await;
        }

        self.base_limiter.acquire_token().await
    }

    pub async fn record_error(&self) {
        let mut counter = self.error_counter.lock().await;
        counter.errors.push(Instant::now());
        
        // Clean up old errors
        let cutoff = Instant::now() - counter.window_size;
        counter.errors.retain(|&time| time > cutoff);
    }

    async fn recent_error_count(&self) -> usize {
        let mut counter = self.error_counter.lock().await;
        let cutoff = Instant::now() - counter.window_size;
        counter.errors.retain(|&time| time > cutoff);
        counter.errors.len()
    }

    pub async fn is_healthy(&self) -> bool {
        self.recent_error_count().await < self.error_counter.lock().await.threshold
    }
}
