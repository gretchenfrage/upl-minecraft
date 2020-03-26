
use std::{
    path::Path,
    env,
};
use yup_oauth2::{
    ServiceAccountAuthenticator,
    ServiceAccountKey,
    read_service_account_key,
    authenticator::{
        HyperClientBuilder,
        DefaultHyperClient,
        Authenticator,
    },
};

/// Credentials and helpers for access to google cloud storage.
///
/// Docs on obtaining credentials:
/// https://developers.google.com/identity/protocols/oauth2/service-account
pub struct GcsAccess {
    auth: Authenticator<<DefaultHyperClient as HyperClientBuilder>::Connector>,
}

impl GcsAccess {
    pub async fn new(key: ServiceAccountKey) -> Result<Self, ()>
    {
        let auth = ServiceAccountAuthenticator::builder(key)
            .build().await
            .map_err(|e| 
                error!("failure to load service account: {}", e))?;
        Ok(GcsAccess { auth })
    }

    pub async fn new_from_path<P>(path: P) -> Result<Self, ()>
    where 
        P: AsRef<Path>
    {
        let key = read_service_account_key(path)
            .await
            .map_err(|e| 
                error!("failed to parse service account key: {}", e))?;
        GcsAccess::new(key).await
    }

    pub async fn new_from_env_path(var: &str) -> Result<Self, ()>
    {
        let path = env::var(var)
            .map_err(|e| 
                error!("failed to get service account path: {}", e))?;
        Self::new_from_path(path).await
    }

    pub async fn token(&self) -> Result<String, ()>
    {
        let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
        self.auth.token(scopes).await
            .map(|token| token.as_str().to_owned())
            .map_err(|e| error!("failed to acquire GCS access token: {}", e))
    }
}