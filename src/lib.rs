use std::{net::SocketAddr, sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::{
    fs::OpenOptions,
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::{Mutex, RwLock, RwLockReadGuard},
};

pub type Clients = Arc<RwLock<Vec<(OwnedWriteHalf, SocketAddr)>>>;
pub type ClientGuard<'a> = RwLockReadGuard<'a, Vec<(Arc<Mutex<TcpStream>>, SocketAddr)>>;

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    client: String,
    msg: String,
    timestamp: String,
}

pub async fn fmt_username(clients: &Clients, stream: &mut OwnedReadHalf) -> String {
    let mut name_buffer = [0; 128];
    let name_bytes = stream.read(&mut name_buffer).await.unwrap_or(0);
    let mut username = if name_bytes > 0 {
        String::from_utf8_lossy(&name_buffer[..name_bytes])
            .trim_end()
            .to_string()
    } else {
        String::new()
    };
    if username.is_empty() {
        username = format!("user#{}", clients.read().await.len());
    }
    username
}

pub async fn boardcast_msg_and_store(
    clients: &Clients,
    username: String,
    input: &str,
    socket_addr: SocketAddr,
) {
    let now = chrono::Local::now();
    let ts = now.format("[%Y-%m-%d %H:%M:%S]").to_string();
    let fmt_msg = format!("{} [{}]: {}", ts, username, input);

    let mut guard = clients.write().await;

    for (writer, addr) in guard.iter_mut() {
        if *addr != socket_addr {
            let _ = writer.write_all(fmt_msg.as_bytes()).await;
        }
    }

    store_msg(username, input.trim_end().to_string(), ts).await;
}

async fn store_msg(client: String, msg: String, timestamp: String) {
    let stored_msg = Message {
        client,
        msg,
        timestamp,
    };
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("history.txt")
        .await
        .expect("cannot open file");
    let json = serde_json::to_string(&stored_msg).expect("Serialisation failed");
    file.write_all(json.as_bytes()).await.expect("Write failed");
    file.write_all(b"\n").await.expect("Write failed");

    println!("Stored: {}", json);
}

pub async fn boardcast_leave_notification(
    clients: &Clients,
    socket_addr: SocketAddr,
    username: &str,
) {
    let mut guard = clients.write().await;
    guard.retain(|(_, addr)| *addr != socket_addr);

    for (writer, _) in guard.iter_mut() {
        let msg = format!("[{}] left the server\n", username);
        if let Err(e) = writer.write_all(msg.as_bytes()).await {
            eprintln!("Failed to send leave message: {}", e);
        }
    }

    let fmt_msg = chrono::Local::now()
        .format("[%Y-%m-%d %H:%M:%S]")
        .to_string();

    println!("{} [{}] disconnected", fmt_msg, username);
}
