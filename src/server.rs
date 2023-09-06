use crate::connection::Connection;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

pub struct Server {
    listener: TcpListener,
    addr: String,
    conns: HashMap<String, Arc<Mutex<Connection>>>,
}

// impl Server {
impl Server {
    pub async fn new(addr: String) -> Result<Self> {
        let listener = TcpListener::bind(addr.clone()).await?;
        let conns: HashMap<String, Arc<Mutex<Connection>>> = HashMap::new();
        let s = Server {
            listener,
            addr,
            conns,
        };
        Ok(s)
    }

    pub async fn run(&mut self) {
        loop {
            let (stream, addr) = self.listener.accept().await.unwrap();
            println!("Accepted connection from {}", addr);
            let conn = Arc::new(Mutex::new(Connection::new(stream).await));

            // let conn_clone = conn.clone();
            // tokio::spawn(async move {
            //     loop {
            //         let msg = conn_clone.lock().await.get_message().await;
            //         // handle message
            //         // self.handle_message(msg).await;
            //         match msg {
            //             Ok(m) => {
            //                 println!("Received message: {:?}", m);
            //             }
            //             Err(e) => {
            //                 println!("Error: {:?}", e);
            //                 continue;
            //             }
            //         }
            //     }
            // });

            self.conns.insert(addr.to_string(), conn);
        }
    }
}
