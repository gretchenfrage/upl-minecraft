
pub mod auth;
pub mod public;
pub use self::auth::GcsAccess;
pub use mime::{self, Mime};
pub use headers::CacheControl;

use std::iter::Extend;
use hyper::{
    Request,
    client::{
        Client,
        HttpConnector,
    },
};
use hyper_tls::HttpsConnector;
//use mime_multipart as multipart;
use serde::{Serialize, Deserialize};
use serde_json;
use headers::{self, Header, HeaderValue};

pub struct GcsClient {
    access: GcsAccess,
    http: Client<HttpsConnector<HttpConnector>>,
}

pub trait MimeData {
    fn media_type(&self) -> Mime;

    fn to_bytes(&self) -> Vec<u8>;

    fn into_bytes(self) -> Vec<u8>;
}

impl MimeData for String {
    fn media_type(&self) -> Mime {
        mime::TEXT_PLAIN
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_owned()
    }

    fn into_bytes(self) -> Vec<u8> {
        self.into_bytes()
    } 
}

impl MimeData for Vec<u8> {
    fn media_type(&self) -> Mime {
        mime::APPLICATION_OCTET_STREAM
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.clone()
    }

    fn into_bytes(self) -> Vec<u8> {
        self
    }
}

impl MimeData for (Vec<u8>, Mime) {
    fn media_type(&self) -> Mime {
        self.1.clone()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }

    fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

pub struct InsertOptions {
    cache_control: CacheControl,
}

impl Default for InsertOptions {
    fn default() -> Self {
        InsertOptions {
            cache_control: CacheControl::new().with_no_cache()
        }
    }
}

fn header_fmt<H: Header>(header: H) -> String {
    struct HeaderFormatter(String);

    impl Extend<HeaderValue> for HeaderFormatter {
        fn extend<I>(&mut self, iter: I)
        where
            I: IntoIterator<Item=HeaderValue>
        {
            let buffer = &mut self.0;
            for value in iter {
                let part = value.to_str().unwrap();
                if buffer.len() > 0 {
                    buffer.push(',');
                }
                buffer.push_str(part);    
            }
        }
    }

    let mut f = HeaderFormatter(String::new());
    header.encode(&mut f);
    f.0
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

    pub async fn insert<D>(
        &self, 
        bucket: &str, 
        object: &str,
        data: D,
        options: InsertOptions,
    ) -> Result<(), ()> 
    where
        D: MimeData,
    {
        let uri = format!(
            "https://storage.googleapis.com/upload/storage/v1/b/{}/o?uploadType=multipart&name={}",
            bucket,
            object);

        #[derive(Serialize, Deserialize, Debug)]
        #[serde(rename_all="camelCase")]
        struct MultipartMeta {
            cache_control: String,
        }

        let mp_meta = MultipartMeta {
            cache_control: header_fmt(options.cache_control),
        };
        debug!("mp_meta = {:#?}", mp_meta);
        let mp_meta = serde_json::to_string(&mp_meta)
            .map_err(|e| error!("unexpected failure to JSON-serialize: {}", e))?;
        debug!("mp_meta json = {}", mp_meta);

        let boundary = "my-multipart-boundary";

        macro_rules! format_crlf {
            ($((
                $($arg:tt)*
            );)*)=>{{
                let mut buffer = String::new();
                $(
                    let line = format!($($arg)*);
                    buffer.push_str(&line);
                    buffer.push_str("\r\n");
                )*
                buffer
            }};
        }

        let body = format_crlf! {
            ("--{}", boundary);
            ("Content-Type: application/json;charset=UTF-8");
            ("Content-Transfer-Encoding: identity");
            ("");
            ("{}", mp_meta);

            ("--{}", boundary);
            ("Content-Type: {}", data.media_type().essence_str());
            ("Content-Transfer-Encoding: identity");
            ("");
        };
        debug!("body before data = \n{}", body);
        let mut body = body.into_bytes();
        body.extend(data.into_bytes());
        body.extend(format!("\r\n\r\n--{}--", boundary).into_bytes());


        let token = self.access.token().await?;
        let auth_header = format!("Bearer {}", token);

        let request = Request::post(uri)
            .header("Authorization", auth_header)
            .header("Content-Type", format!("multipart/related; boundary={}", boundary))
            .body(body.into())
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
