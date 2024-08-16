use tokio::net::{TcpListener, TcpStream}; 
use tokio::sync::broadcast;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader}; 
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
mod test;
type Users = Arc<Mutex<HashMap<String, String>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

	let listener = TcpListener::bind("127.0.0.1:8080").await?;

    let (tx, _rx) = broadcast::channel::<String>(10);

    let users = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (mut socket, _) = listener.accept().await?;
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        let users = users.clone();

        let client_addr = socket.peer_addr().unwrap().to_string();

      

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            let client_id = client_addr.clone();

            let pseudo = loop {
                writer.write_all("Entrez votre pseudo(Commencez la ligne avec /nick): ".as_bytes()).await.unwrap();
                line.clear();
                if reader.read_line(&mut line).await.unwrap() > 0 {
                    if line.starts_with("/nick") {
                        let parts: Vec<&str> = line.trim().splitn(2, ' ').collect();
                        if parts.len() == 2 {
                            let new_nick = parts[1].trim().to_string();
                            users.lock().unwrap().insert(client_id.clone(), new_nick.clone());
                            writer.write_all(format!("Votre pseudo est maintenant {}\n", new_nick).as_bytes()).await.unwrap();
                            break new_nick;
                        }
                    }
                }
            };

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        match result {
                            Ok(0) => break, 
                            Ok(_) => {
                                if line.starts_with("/nick") {
                                    let parts: Vec<&str> = line.trim().splitn(2, ' ').collect();
                                    if parts.len() == 2 {
                                        let new_nick = parts[1].trim().to_string();
                                        users.lock().unwrap().insert(client_id.clone(), new_nick.clone());
                                        let confirmation = format!("Votre pseudo est maintenant {}\n", new_nick);
                                        writer.write_all(confirmation.as_bytes()).await.unwrap();
                                    }
                                } else {
                                    let user_list = users.lock().unwrap();
                                    let pseudo = user_list.get(&client_id).unwrap_or(&"Anonyme".to_string()).clone();
                                    let msg = format!("[{}]: {}", pseudo, line);
                                    if tx.send(msg).is_err() {
                                        break;
                                    }
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
                            Err(broadcast::error::RecvError::Lagged(_)) => continue,
                            Err(_) => break,
                        }
                    }
                }
            }

            users.lock().unwrap().remove(&client_id);
        });
    }
}
