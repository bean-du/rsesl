use anyhow::Result;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

pub struct Server {
    listener: TcpListener,
    addr: String,
}

impl Server {
    pub async fn new(addr: String) -> Result<Self> {
        let listener = TcpListener::bind(addr.clone()).await?;
        let s = Server { listener, addr };
        Ok(s)
    }

    pub async fn accept(&mut self) -> Result<(TcpStream, SocketAddr)> {
        let (stream, addr) = self.listener.accept().await?;
        Ok((stream, addr))
    }
}
