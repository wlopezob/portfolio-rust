use anyhow::Result;
use tokio::process::Command;
use tokio::sync::RwLock;

/// Global authentication state to avoid checking on every request
static AUTH_VERIFIED: RwLock<bool> = RwLock::const_new(false);

#[async_trait::async_trait]
pub trait AuthProvider {
    async fn verify(&self) -> Result<()>;
    async fn is_verified(&self) -> bool;
}

#[derive(Clone)]
pub struct GcloudAuthProvider;

impl GcloudAuthProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GcloudAuthProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl AuthProvider for GcloudAuthProvider {
    async fn verify(&self) -> Result<()> {
        // Check if already verified (cached)
        if self.is_verified().await {
            return Ok(());
        }
        
        eprintln!("[AUTH] Verifying gcloud authentication...");
        
        let output = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            Command::new("gcloud")
                .args(&["auth", "application-default", "print-access-token"])
                .output()
        )
        .await
        .map_err(|_| anyhow::anyhow!("gcloud auth check timed out after 5 seconds"))??;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("[AUTH] Authentication failed: {}", stderr);
            anyhow::bail!("gcloud not authenticated. Run: gcloud auth application-default login");
        }

        eprintln!("[AUTH] Authentication verified successfully");
        
        // Mark as verified
        let mut verified = AUTH_VERIFIED.write().await;
        *verified = true;
        
        Ok(())
    }
    
    async fn is_verified(&self) -> bool {
        let verified = AUTH_VERIFIED.read().await;
        *verified
    }
}
