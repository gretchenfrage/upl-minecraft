
pub mod auth;
pub mod public;
pub use self::auth::GcsAccess;

use hyper::{
    Request,
    client::{
        Client,
        HttpConnector,
    },
};
use hyper_tls::HttpsConnector;

pub struct GcsClient {
    access: GcsAccess,
    http: Client<HttpsConnector<HttpConnector>>,
}

impl GcsClient {
    pub fn new(access: GcsAccess) -> Self {
        let mut connector = HttpsConnector::new();
        connector.https_only(true);
        let http = Client::builder().build(connector);
        
        GcsClient {
            access,
            http,
        }
    }

    pub async fn set(
        &self, bucket: &str, object: &str, data: Vec<u8>
    ) -> Result<(), ()> {
        let uri = format!(
            "https://www.googleapis.com/upload/storage/v1/b/{}/o?uploadType=media&name={}",
            bucket, object);

        let token = self.access.token().await?;
        let auth_header = format!(
            "Bearer {}",
            token);

        let request = Request::post(uri)
            .header("Authorization", auth_header)
            .header("Content-Type", "text/plain")
            .header("Cache-Control", "no-cache,max-age=0")
            .body(data.into())
            .map_err(|e| 
                error!("failure to build HTTP request: {}", e))?;

        let response = self.http.request(request).await
            .map_err(|e| error!("failed to make HTTP request: {}", e))?;
        if !response.status().is_success() {
            error!("GCS request failed:\n{:#?}", response);
            Err({})
        } else {
            Ok({})
        }
    }
}
