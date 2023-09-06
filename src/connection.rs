use crate::message::Message;
use anyhow::{Ok, Result};
use tokio::{
    io::{AsyncWriteExt, BufReader, BufWriter},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
};

#[derive(Debug)]
pub struct Connection {
    writer: BufWriter<OwnedWriteHalf>,
    reader: BufReader<OwnedReadHalf>,
}

impl Connection {
    pub async fn new(stream: TcpStream) -> Self {
        let (r, w) = stream.into_split();
        let writer = BufWriter::new(w);
        let reader = BufReader::new(r);

        Connection { writer, reader }
    }

    pub async fn send(&mut self, data: String) -> Result<()> {
        let b = data.trim().as_bytes();
        self.writer.write_all(b).await?;
        self.writer.flush().await?;

        self.writer.write_all("\r\n\r\n".as_bytes()).await?;
        self.writer.flush().await?;
        Ok(())
    }

    pub async fn get_message(&mut self) -> Result<Message> {
        let mut reader = &mut self.reader;
        Message::parse(&mut reader).await
    }
}
