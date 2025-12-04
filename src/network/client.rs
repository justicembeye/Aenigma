use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use crate::network::protocol::NetworkMessage;

pub struct Client {
    // Channel pour envoyer des messages au thread réseau
    tx: mpsc::Sender<NetworkMessage>,
    // Channel pour recevoir des messages du thread réseau (pour l'UI)
    rx: mpsc::Receiver<NetworkMessage>,
}

impl Client {
    pub async fn connect(addr: String) -> Result<Self, Box<dyn std::error::Error>> {
        let socket = TcpStream::connect(addr).await?;
        let (mut reader, mut writer) = socket.into_split();

        let (ui_tx, rx) = mpsc::channel(100);
        let (tx, mut network_rx) = mpsc::channel(100);

        // Thread de lecture (Socket -> UI)
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            loop {
                match reader.read(&mut buffer).await {
                    Ok(0) => break,
                    Ok(n) => {
                        if let Ok(msg) = serde_json::from_slice::<NetworkMessage>(&buffer[..n]) {
                            let _ = ui_tx.send(msg).await;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        // Thread d'écriture (UI -> Socket)
        tokio::spawn(async move {
            while let Some(msg) = network_rx.recv().await {
                if let Ok(json) = serde_json::to_string(&msg) {
                    let _ = writer.write_all(json.as_bytes()).await;
                }
            }
        });

        Ok(Client { tx, rx })
    }

    pub async fn send(&self, msg: NetworkMessage) {
        let _ = self.tx.send(msg).await;
    }

    pub fn try_recv(&mut self) -> Option<NetworkMessage> {
        self.rx.try_recv().ok()
    }
}
