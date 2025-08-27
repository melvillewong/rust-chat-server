use std::{
    fs::OpenOptions,
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    sync::{Arc, RwLock, RwLockWriteGuard},
};

use serde::{Deserialize, Serialize};

pub type Clients = Arc<RwLock<Vec<(SocketAddr, TcpStream)>>>;

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    client: String,
    msg: String,
    timestamp: String,
}

pub fn fmt_username(clients: &Clients, stream: &mut TcpStream) -> String {
    let mut name_buffer = [0; 128];
    let name_bytes = stream.read(&mut name_buffer).unwrap();
    let mut username = String::from_utf8_lossy(&name_buffer[..name_bytes])
        .trim_end()
        .to_string();
    if username.is_empty() {
        username = format!("user#{}", clients.read().unwrap().len());
    }
    username
}

pub fn boardcast_msg_and_store(
    client_guard: &mut RwLockWriteGuard<Vec<(SocketAddr, TcpStream)>>,
    username: String,
    input: &str,
    socket_addr: SocketAddr,
) {
    let now = chrono::Local::now();
    let ts = now.format("[%Y-%m-%d %H:%M:%S]").to_string();

    let fmt_msg = format!("{} [{}]: {}", ts, username, input);
    for (addr, client) in client_guard.iter_mut() {
        if *addr != socket_addr {
            client.write_all(fmt_msg.as_bytes()).unwrap();
        }
    }

    store_msg(username, input.trim_end().to_string(), ts);
}

fn store_msg(client: String, msg: String, timestamp: String) {
    let stored_msg = Message {
        client,
        msg,
        timestamp,
    };
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("history.txt")
        .expect("cannot open file");
    let json = serde_json::to_string(&stored_msg).expect("Serialisation failed");
    writeln!(file, "{}", json).expect("Write failed");
}

pub fn boardcast_leave_notification(clients: &Clients, socket_addr: SocketAddr, username: &str) {
    let mut clients = clients.write().unwrap();
    clients.retain(|(addr, _)| *addr != socket_addr);
    for (_, client) in clients.iter_mut() {
        client
            .write_all(format!("[{}] leaved the server\n", username).as_bytes())
            .unwrap();
    }

    let fmt_msg = chrono::Local::now()
        .format("[%Y-%m-%d %H:%M:%S]")
        .to_string();

    println!("{} [{}] disconnected", fmt_msg, username);
}
