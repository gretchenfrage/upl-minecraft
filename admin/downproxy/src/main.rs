
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate regex;
extern crate yup_oauth2;
extern crate tokio;
extern crate hyper;
extern crate hyper_tls;
extern crate reqwest;

pub mod client;
pub mod gcs;
pub mod server;

use std::{
    process::exit,
    env,
};
use gcs::{GcsAccess, GcsClient};
use tokio::prelude::*;

macro_rules! argv_branch {
    ($argv:expr, {
        $( $function:ident ),* $(,)?
    })=>{{
        fn f<V>(v: V) -> impl Iterator<Item=String>
        where
            V: IntoIterator<Item=String>
        {
            v.into_iter()
        }

        let mut argi = f($argv);
        match argi.next() {
            Some(subcmd) => {
                let tail: Vec<String> = argi.collect();
                match subcmd.as_str() {
                    $(
                    stringify!($function) => {
                        ($function)(tail).await;
                        true
                    },
                    )*
                    _ => false
                }
            },
            None => false,
        }
    }};
}

pub static TOKEN_PATH_VAR: &str = "TOKEN_PATH";
pub static BUCKET: &str = "mcupl-var";
pub static OBJECT: &str = "host-address";

async fn local_client_address(_args: Vec<String>) {
    if let Ok(addr) = client::address::local_client_address() {
        println!("{}", addr.to_string());
    } else {
        exit(1);
    }
}

async fn download_host_address(_args: Vec<String>) {
    let addr = server::download_host_address().await
        .unwrap_or_else(|()| exit(1));
    println!("{}", addr.to_string());
}

async fn test_token(args: Vec<String>) {
    let path = args.get(0)
        .unwrap_or_else(|| {
            error!("required argument TOKEN_PATH missing");
            exit(1);
        });

    let auth = GcsAccess::new_from_path(path).await
        .unwrap_or_else(|()| exit(1));
    let token = auth.token().await
        .unwrap_or_else(|()| exit(1));
    println!("{}", token);
}

async fn upload_host_address(_args: Vec<String>) {
    info!("getting local client address");
    let addr = client::address::local_client_address()
        .unwrap_or_else(|()| exit(1));
    info!("address = {}", addr);

    info!("looking for token path in env var {}", TOKEN_PATH_VAR);
    let auth = GcsAccess::new_from_env_path(TOKEN_PATH_VAR).await
        .unwrap_or_else(|()| exit(1));
    let gcs = GcsClient::new(auth);
    info!("constructing gcs client");

    info!("posting object");
    let body = addr.to_string().into_bytes();
    gcs.set(BUCKET, OBJECT, body).await
        .unwrap_or_else(|()| exit(1));
    info!("success!");
}

#[tokio::main]
async fn main() {
    env_logger::init();

    if !argv_branch!(env::args().skip(1), {
        local_client_address,
        test_token,
        upload_host_address,
        download_host_address
    }) {
        println!("EPIC fail");
        exit(1);
    }
}