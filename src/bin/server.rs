use std::{
    io::Read,
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{Arc, RwLock},
    thread,
};

use tcp_chat_server::{boardcast_leave_notification, boardcast_msg_and_store, fmt_username};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9000")?;
    println!("listening on 127.0.0.1:9000...");

    let clients = Arc::new(RwLock::new(Vec::<(SocketAddr, TcpStream)>::new()));

    for stream in listener.incoming() {
        let mut stream = stream?;
        let socket_addr = stream.peer_addr().unwrap();

        let clients = Arc::clone(&clients);
        let stream_clone = stream.try_clone()?;

        clients.write().unwrap().push((socket_addr, stream_clone));

        thread::spawn(move || {
            let mut buffer = [0; 512];

            // Read client's username
            let username = fmt_username(&clients, &mut stream);
            println!("New client [{}] connected!", username);

            loop {
                match stream.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(bytes_read) => {
                        let input = String::from_utf8_lossy(&buffer[..bytes_read]);
                        let mut client_guard = clients.write().unwrap();

                        boardcast_msg_and_store(
                            &mut client_guard,
                            username.clone(),
                            &input,
                            socket_addr,
                        );
                    }
                    Err(_) => break,
                }
            }

            boardcast_leave_notification(&clients, socket_addr, &username);
        });
    }

    Ok(())
}
