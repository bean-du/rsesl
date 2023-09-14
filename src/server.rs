use crate::session::Session;
use anyhow::Result;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
use tracing::{debug, error, info, trace, warn};

pub struct Server {
    listener: TcpListener,
    addr: String,
    tx: Sender<Arc<Mutex<Session>>>,
    rx: Receiver<Arc<Mutex<Session>>>,
}

impl Server {
    pub async fn new(addr: String) -> Result<Self> {
        let listener = TcpListener::bind(addr.clone()).await?;
        let (tx, rx) = tokio::sync::mpsc::channel::<Arc<Mutex<Session>>>(32);
        let s = Server {
            listener,
            addr,
            tx,
            rx,
        };
        Ok(s)
    }

    pub async fn accept(&mut self) -> Result<Arc<Mutex<Session>>> {
        let (stream, addr) = self.listener.accept().await?;
        info!("Accepted connection from {}", addr);
        let conn = Arc::new(Mutex::new(Session::new(stream).await));
        Ok(conn)
    }
}
