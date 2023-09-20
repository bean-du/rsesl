use crate::message::Message;
use crate::message::MsgError;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use tokio::{
    io::{AsyncWriteExt, BufReader, BufWriter},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::Mutex,
};
use tracing::{debug, error, info, trace};

#[derive(Debug)]
pub struct Session {
    in_tx: broadcast::Sender<Arc<Mutex<Message>>>,
    out_tx: mpsc::Sender<String>,

    close_tx: broadcast::Sender<bool>,
    close_rx: broadcast::Receiver<bool>,
    pub is_closed: Arc<Mutex<bool>>,
}

impl Session {
    pub async fn new(
        stream: TcpStream,
        // in_tx is used to send Message to the outside
        in_tx: broadcast::Sender<Arc<Mutex<Message>>>,
        // shutdown is used to receive a signal to shutdown
        shutdown: broadcast::Receiver<bool>,
    ) -> Self {
        let (r, w) = stream.into_split();
        let writer = BufWriter::new(w);
        let reader = BufReader::new(r);

        let tx1 = in_tx.clone();

        let r_signal = shutdown.resubscribe();
        let w_signal = shutdown.resubscribe();

        let (out_tx, out_rx) = mpsc::channel::<String>(1000);
        let (close_tx, close_rx) = broadcast::channel::<bool>(1);
        let is_closed = Arc::new(Mutex::new(false));

        tokio::spawn(read(
            is_closed.clone(),
            close_tx.clone(),
            close_rx.resubscribe(),
            tx1,
            reader,
            r_signal,
        ));

        tokio::spawn(write(
            is_closed.clone(),
            close_tx.clone(),
            close_rx.resubscribe(),
            out_rx,
            writer,
            w_signal,
        ));

        Session {
            in_tx,
            out_tx,
            close_rx,
            close_tx,
            is_closed,
        }
    }

    pub async fn sender(self) -> mpsc::Sender<String> {
        self.out_tx.clone()
    }

    pub async fn is_closed(&self) -> bool {
        let c = self.is_closed.lock().await;
        *c
    }
    pub async fn send(&mut self, data: String) -> Result<()> {
        let b = format!("{}\n\n", data.trim());

        self.out_tx.send(b).await?;

        Ok(())
    }
}

pub async fn read(
    closed: Arc<Mutex<bool>>,
    close_tx: broadcast::Sender<bool>,
    mut close_rx: broadcast::Receiver<bool>,
    tx: broadcast::Sender<Arc<Mutex<Message>>>,
    reader: BufReader<OwnedReadHalf>,
    mut exit: broadcast::Receiver<bool>,
) {
    let mut reader = reader;
    loop {
        let c = closed.lock().await;
        if *c == true {
            return;
        }

        tokio::select! {
            _ = exit.recv() => {
                break;
            }
            _ = close_rx.recv() => {
                let mut c = closed.lock().await;
                *c = true;
                return;
            }
            msg = Message::parse(&mut reader) => {
                let msg = match msg {
                    Ok(m) => m,
                    Err(e) => {
                        match e.downcast::<MsgError>() {
                            Ok(e) => {
                                error!("Failed to parse message: {:?}", e);
                                info!("close session read thread");
                                match close_tx.send(true) {
                                    Ok(_) => {
                                        let mut c = closed.lock().await;
                                        *c = true;
                                    }
                                    Err(e) => {
                                        error!("send signal error {}", e)
                                    }
                                }
                                return ;
                            }
                            Err(e) => {
                                error!("Failed to parse message: {:?}", e);
                                continue;
                            }
                        }

                    }

                };
                debug!("received msg: {:?}", msg);
                let msg = Arc::new(Mutex::new(msg));
                match tx.send(msg) {
                    Ok(_) => {
                        debug!("send msg to brain successfully");
                    }
                    Err(e) => {
                        error!("Failed to send message: {}", e);
                        continue;
                    }
                }

            }
        }
    }
}

pub async fn write(
    closed: Arc<Mutex<bool>>,
    close_tx: broadcast::Sender<bool>,
    mut close_rx: broadcast::Receiver<bool>,
    mut rx: mpsc::Receiver<String>,
    writer: BufWriter<OwnedWriteHalf>,
    mut exit: broadcast::Receiver<bool>,
) {
    let mut writer = writer;
    loop {
        tokio::select! {
            Some(msg) = rx.recv() => {
                let b = format!("{}\n\n", msg.trim());
                match writer.write_all(b.as_bytes()).await {
                    Ok(_) => {}
                    Err(e) => {
                        error!("write msg to freeswitch error: {}", e);
                        continue;
                    }
                }
                match writer.flush().await {
                    Ok(_) => {}
                    Err(e) => {
                        error!("flush msg to freeswitch error: {}", e);
                        match close_tx.send(true) {
                            Ok(_) => {
                                let mut c = closed.lock().await;
                                *c = true;
                            }
                            Err(e) => {
                                error!("send signal error {}", e)
                            }
                        }
                        info!("close session write thread");
                        continue;
                    }
                }
            }
             _ = close_rx.recv() => {
                let mut c = closed.lock().await;
                *c = true;
                return;
            }
            _ = exit.recv() => {
                break;
            }
        }
    }
}
