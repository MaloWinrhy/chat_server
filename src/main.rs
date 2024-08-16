use tokio::net::{TcpListener, TcpStream}; 
use tokio::sync::broadcast;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader}; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Initialisation du listener TCP sur l'adresse et le port spécifiés
	let listener = TcpListener::bind("127.0.0.1:8080").await?;

    // Création d'un canal de diffusion pour permettre la communication entre les clients
    let (tx, _rx) = broadcast::channel::<String>(10);

    loop {
        // Acceptation des connexions entrantes
        let (mut socket, _) = listener.accept().await?;

        // Clonage du canal pour permettre la communication avec ce client spécifique
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {

           
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();


            loop {
                tokio::select! {
              
                    result = reader.read_line(&mut line) => {
                        match result {
                            Ok(0) => break,
                            Ok(_) => {
                                if tx.send(line.clone()).is_err() {
                                    break; 
                                }
                                line.clear(); 
                            },
                            Err(_) => break, 
                        }
                    }

                    result = rx.recv() => {
                        match result {
                            Ok(msg) => {
                                if writer.write_all(msg.as_bytes()).await.is_err() {
                                    break; 
                                }
                            },
                            Err(broadcast::error::RecvError::Lagged(_)) => {
                                continue; 
                            },
                            Err(_) => break,
                        }
                    }
                }
            }
        });
    }
}
