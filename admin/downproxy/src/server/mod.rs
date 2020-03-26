
use crate::{
    BUCKET,
    OBJECT,
    gcs::public::gcs_get_string,
};
use std::{
    net::IpAddr,
    str::FromStr,
};

pub async fn download_host_address() -> Result<IpAddr, ()> {
    let string = gcs_get_string(BUCKET, OBJECT).await?;

    IpAddr::from_str(&string)
        .map_err(|e| 
            error!("GCS response not parseable as IpAddr: {}", e))
}