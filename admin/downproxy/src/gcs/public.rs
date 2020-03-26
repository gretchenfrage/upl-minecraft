
use reqwest;

pub async fn gcs_get_string(bucket: &str, object: &str) -> Result<String, ()> {
    let uri = format!(
        "https://storage.googleapis.com/storage/v1/b/{}/o/{}?alt=media",
        bucket,
        object);
    reqwest::get(&uri).await
        .map_err(|e| error!("problem requesting GCS object: {}", e))?
        .text().await
        .map_err(|e| error!("GCS response isn't valid text: {}", e))
}