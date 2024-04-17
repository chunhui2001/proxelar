use std::net::SocketAddr;

use proxyapi::{proxy::Proxy, ProxyHandler};
use proxyapi_models::RequestInfo;

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}

#[tokio::main]
async fn main() {

    let (tx, rx) = std::sync::mpsc::sync_channel::<ProxyHandler>(1);
    // let (close_tx, close_rx) = tokio::sync::oneshot::channel();

    tokio::spawn(async move {
        for exchange in rx.iter() {
            let (request, response) = exchange.to_parts();
            let _info = RequestInfo(request, response);

            println!("RequestInfo: ={:?}", _info.0.unwrap().uri());
        }
    });

    if let Err(e) = Proxy::new(SocketAddr::new([127, 0, 0, 1].into(), 8080), Some(tx.clone()))
        .start(shutdown_signal())
        .await
    {
        eprintln!("{e}");
    }
}
