

#[cfg(test)]
mod tests {
    use tokio::io::AsyncReadExt;
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpListener;
    use tokio::net::TcpStream;
    use tokio::time::Duration;

    use super::*;

    #[tokio::test]
    async fn test_connection() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            if let Ok((mut socket, _)) = listener.accept().await {
                let _ = socket.write_all(b"Hello, world!").await;
            }
        });

        let mut socket = TcpStream::connect(addr).await.unwrap();
        let mut buffer = [0; 13];
        let _ = socket.read_exact(&mut buffer).await;

        assert_eq!(&buffer, b"Hello, world!");
    }

    #[tokio::test]
    async fn test_nickname_change() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            if let Ok((mut socket, _)) = listener.accept().await {
                let _ = socket.write_all(b"Entrez votre pseudo(Commencez la ligne avec /nick): ").await;
                let mut buffer = [0; 14];
                let _ = socket.read_exact(&mut buffer).await;
                let _ = socket.write_all(b"/nick John\n").await;
                let _ = socket.read_exact(&mut buffer).await;
                let _ = socket.write_all(b"Hello, world!\n").await;
                let _ = socket.read_exact(&mut buffer).await;
            }
        });

        let mut socket = TcpStream::connect(addr).await.unwrap();
        let mut buffer = [0; 13];
        let _ = socket.read_exact(&mut buffer).await;
        let _ = socket.write_all(b"/nick John\n").await;
        let _ = socket.read_exact(&mut buffer).await;
        let _ = socket.write_all(b"Hello, world!\n").await;
        let _ = socket.read_exact(&mut buffer).await;

        assert_eq!(&buffer, b"Hello, world!");

    }

    #[tokio::test]
    async fn test_broadcast_message() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            if let Ok((mut socket, _)) = listener.accept().await {
                let _ = socket.write_all(b"Entrez votre pseudo(Commencez la ligne avec /nick): ").await;
                let mut buffer = [0; 13];
                let _ = socket.read_exact(&mut buffer).await;
                let _ = socket.write_all(b"/nick John\n").await;
                let _ = socket.read_exact(&mut buffer).await;
                let _ = socket.write_all(b"Hello, world!\n").await;
                let _ = socket.read_exact(&mut buffer).await;

                assert_eq!(&buffer, b"Hello, world!");
            }
        });

        let mut socket = TcpStream::connect(addr).await.unwrap();
        let mut buffer = [0; 13];
        let _ = socket.read_exact(&mut buffer).await;
        let _ = socket.write_all(b"/nick John\n").await;
        let _ = socket.read_exact(&mut buffer).await;
        let _ = socket.write_all(b"Hello, world!\n").await;
        let _ = socket.read_exact(&mut buffer).await;

        tokio::time::sleep(Duration::from_secs(1)).await;

        let mut socket2 = TcpStream::connect(addr).await.unwrap();
        let mut buffer2 = [0; 100];
        let _ = socket2.read_exact(&mut buffer2).await;

        assert_eq!(&buffer, b"Hello, world!");

    }
}
