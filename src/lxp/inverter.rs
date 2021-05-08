use crate::prelude::*;

use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{
    tcp::{OwnedReadHalf, OwnedWriteHalf},
    TcpStream,
};
use tokio_util::codec::Decoder;

pub struct Inverter {
    config: Rc<Config>,
    from_coordinator: PacketSender,
    to_coordinator: PacketSender,
}

pub type PacketSender = tokio::sync::broadcast::Sender<Option<Packet>>;

impl Inverter {
    pub fn new(
        config: Rc<Config>,
        from_coordinator: PacketSender,
        to_coordinator: PacketSender,
    ) -> Self {
        Self {
            config,
            from_coordinator,
            to_coordinator,
        }
    }

    pub async fn start(&self) -> Result<()> {
        loop {
            match self.connect().await {
                Ok(_) => break,
                Err(e) => {
                    error!("connect: {}", e);
                    info!("attempting inverter reconnection in 5s");
                    self.to_coordinator.send(None)?; // kill any waiting readers
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            };
        }

        Ok(())
    }

    async fn connect(&self) -> Result<()> {
        let i = &self.config.inverter;

        info!("connecting to inverter at {}:{}", &i.host, i.port);

        let inverter_hp = (i.host.to_string(), i.port);
        let (reader, writer) = TcpStream::connect(inverter_hp).await?.into_split();

        info!("inverter connected!");

        futures::try_join!(self.sender(writer), self.receiver(reader))?;

        Ok(())
    }

    // inverter -> coordinator
    async fn receiver(&self, mut socket: OwnedReadHalf) -> Result<()> {
        let mut buf = BytesMut::new();
        let mut decoder = PacketDecoder::new();

        loop {
            // read_buf appends to buf rather than overwrite existing data
            let len = socket.read_buf(&mut buf).await?;

            if len == 0 {
                while let Some(packet) = decoder.decode_eof(&mut buf)? {
                    debug!("RX ({} bytes): {:?}", packet.bytes().len(), packet);
                    self.to_coordinator.send(Some(packet))?;
                }
                break;
            }

            while let Some(packet) = decoder.decode(&mut buf)? {
                debug!("RX ({} bytes): {:?}", packet.bytes().len(), packet);
                self.to_coordinator.send(Some(packet))?;
            }
        }

        Err(anyhow!("receiver exiting (inverter disconnect)"))
    }

    // coordinator -> inverter
    async fn sender(&self, mut socket: OwnedWriteHalf) -> Result<()> {
        let mut receiver = self.from_coordinator.subscribe();

        while let Some(packet) = receiver.recv().await? {
            debug!("TX ({} bytes): {:?}", packet.bytes().len(), packet);
            socket.write_all(&packet.bytes()).await?
        }

        Err(anyhow!(
            "sender exiting due to receiving None from coordinator"
        ))
    }
}
