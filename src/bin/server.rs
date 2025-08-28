use std::{net::SocketAddr, sync::Arc};

use rust_chat_server::{boardcast_leave_notification, boardcast_msg_and_store, fmt_username};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, tcp::OwnedWriteHalf},
    sync::RwLock,
};

pub type Clients = Arc<RwLock<Vec<(OwnedWriteHalf, SocketAddr)>>>;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9000").await?;
    println!("listening on 127.0.0.1:9000...");

    let clients: Clients = Arc::new(RwLock::new(Vec::new()));

    while let Ok((socket, addr)) = listener.accept().await {
        let clients = Arc::clone(&clients);

        let (mut reader, writer) = socket.into_split();

        {
            let mut guard = clients.write().await;
            guard.push((writer, addr));
        }

        tokio::spawn(async move {
            let mut buffer = [0; 512];

            // Read client's username
            let username = fmt_username(&clients, &mut reader).await;
            println!("New client [{}] connected!", username);

            loop {
                match reader.read(&mut buffer).await {
                    Ok(0) => break,
                    Ok(bytes_read) => {
                        let input = String::from_utf8_lossy(&buffer[..bytes_read]);

                        boardcast_msg_and_store(&clients, username.clone(), &input, addr).await;
                    }
                    Err(_) => break,
                }
            }

            boardcast_leave_notification(&clients, addr, &username).await;
        });
    }

    Ok(())
}
