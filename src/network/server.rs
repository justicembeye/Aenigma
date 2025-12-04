use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::broadcast;
use crate::network::protocol::NetworkMessage;


pub struct Server {
    port: u16,
    // On utilisera un channel broadcast pour envoyer les messages à tous les clients
    tx: broadcast::Sender<NetworkMessage>,
}

impl Server {
    pub fn new(port: u16) -> Self {
        let (tx, _rx) = broadcast::channel(100);
        Server { port, tx }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;
        println!("Serveur écoute sur le port {}", self.port);

        loop {
            let (mut socket, addr) = listener.accept().await?;
            println!("Nouvelle connexion: {}", addr);

            let tx = self.tx.clone();
            let mut rx = tx.subscribe();

            tokio::spawn(async move {
                let (mut reader, mut writer) = socket.split();
                let mut buffer = [0; 1024];

                loop {
                    tokio::select! {
                        result = reader.read(&mut buffer) => {
                            match result {
                                Ok(0) => break, // Connexion fermée
                                Ok(n) => {
                                    // Désérialisation basique (à améliorer pour gérer le framing)
                                    if let Ok(msg) = serde_json::from_slice::<NetworkMessage>(&buffer[..n]) {
                                        // On broadcast le message à tout le monde (pour l'instant, simple écho)
                                        let _ = tx.send(msg);
                                    }
                                }
                                Err(_) => break,
                            }
                        }
                        Ok(msg) = rx.recv() => {
                            // On envoie le message au client
                            if let Ok(json) = serde_json::to_string(&msg) {
                                let _ = writer.write_all(json.as_bytes()).await;
                            }
                        }
                    }
                }
            });
        }
    }
}
