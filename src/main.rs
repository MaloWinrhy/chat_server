use tokio::net::{TcpListener, TcpStream}; 
use tokio::sync::broadcast;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader}; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    //init TCP listener
	let listener = TcpListener::bind("127.0.0.1:8080").await?;

    //create channel
    let (tx, _rx) = broadcast::channel::<String>(10);
	Ok(())
}