//! Retry logic for gcloud commands

use anyhow::Result;
use std::time::Duration;

/// Retry policy configuration
pub struct RetryPolicy {
    max_attempts: u32,
    base_delay: Duration,
}

impl RetryPolicy {
    pub fn new(max_attempts: u32) -> Self {
        Self {
            max_attempts,
            base_delay: Duration::from_secs(1),
        }
    }
    
    /// Execute a function with retry logic using exponential backoff
    pub async fn execute<F, Fut, T>(&self, mut f: F) -> Result<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        let mut last_error = None;
        
        for attempt in 1..=self.max_attempts {
            eprintln!("[RETRY] Attempt {}/{}", attempt, self.max_attempts);
            
            match f().await {
                Ok(result) => {
                    eprintln!("[RETRY] Success on attempt {}", attempt);
                    return Ok(result);
                }
                Err(e) => {
                    eprintln!("[RETRY] Attempt {} failed: {}", attempt, e);
                    last_error = Some(e);
                    
                    if attempt < self.max_attempts {
                        // Exponential backoff: base_delay * 2^(attempt-1)
                        let wait_time = self.base_delay * 2u32.pow(attempt - 1);
                        eprintln!("[RETRY] Waiting {:?} before retry...", wait_time);
                        tokio::time::sleep(wait_time).await;
                    }
                }
            }
        }
        
        eprintln!("[RETRY] All {} attempts failed", self.max_attempts);
        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("All retries failed")))
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self::new(3)
    }
}
