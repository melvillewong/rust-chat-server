use std::{
    collections::VecDeque,
    fs::OpenOptions,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex, RwLock},
    thread,
};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    client: String,
    msg: String,
    timestamp: DateTime<Local>,
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9000")?;
    println!("Server listening on 127.0.0.1:9000");

    let clients = Arc::new(RwLock::new(Vec::<(SocketAddr, TcpStream)>::new()));
    let msg_history = Arc::new(Mutex::new(VecDeque::<Message>::new()));

    for stream in listener.incoming() {
        let mut stream = stream?;
        let thread_addr = stream.peer_addr().unwrap();

        let clients = Arc::clone(&clients);
        let msg_hist_clone = Arc::clone(&msg_history);
        let stream_clone = stream.try_clone()?;

        clients.write().unwrap().push((thread_addr, stream_clone));

        thread::spawn(move || {
            let mut buffer = [0; 512];

            // Read client's username
            let mut name_buffer = [0; 128];
            let name_bytes = stream.read(&mut name_buffer).unwrap();
            let mut username = String::from_utf8_lossy(&name_buffer[..name_bytes])
                .trim_end()
                .to_string();
            if username.is_empty() {
                username = format!("user#{}", clients.read().unwrap().len());
            }

            println!("New client [{}] connected!", username);

            loop {
                match stream.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(bytes_read) => {
                        let input = String::from_utf8_lossy(&buffer[..bytes_read]);
                        let mut client_guard = clients.write().unwrap();

                        let now = chrono::Local::now();
                        let ts = now.format("[%Y-%m-%d %H:%M:%S]").to_string();

                        let fmt_msg = format!("{} [{}]: {}", ts, username, input);
                        for (addr, client) in client_guard.iter_mut() {
                            if *addr != thread_addr {
                                client.write_all(fmt_msg.as_bytes()).unwrap();
                            }
                        }

                        let stored_msg = Message {
                            client: username.clone(),
                            msg: fmt_msg,
                            timestamp: now,
                        };
                        msg_hist_clone.lock().unwrap().push_back(stored_msg.clone());
                        let mut file = OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open("history.txt")
                            .expect("cannot open file");
                        let json =
                            serde_json::to_string(&stored_msg).expect("Serialisation failed");
                        writeln!(file, "{}", json).expect("Write failed");
                    }
                    Err(_) => break,
                }
            }

            let mut clients = clients.write().unwrap();
            clients.retain(|(addr, _)| *addr != thread_addr);
            for (_, client) in clients.iter_mut() {
                client
                    .write_all(format!("[{}] leaved the server\n", username).as_bytes())
                    .unwrap();
            }

            let fmt_msg = chrono::Local::now()
                .format("[%Y-%m-%d %H:%M:%S]")
                .to_string();

            println!("{} [{}] disconnected", fmt_msg, username);
        });
    }

    Ok(())
}
