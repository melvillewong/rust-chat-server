use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9000")?;
    println!("Server listening on 127.0.0.1:9000");

    let clients = Arc::new(Mutex::new(Vec::<TcpStream>::new()));

    for stream in listener.incoming() {
        let mut stream = stream?;
        println!("New client connected!");

        let clients = Arc::clone(&clients);
        let stream_clone = stream.try_clone()?;

        clients.lock().unwrap().push(stream_clone);

        thread::spawn(move || {
            let mut buffer = [0; 512];
            let thread_addr = stream.peer_addr().unwrap();
            println!("{thread_addr}");

            loop {
                match stream.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(bytes_read) => {
                        let input = String::from_utf8_lossy(&buffer[..bytes_read]);
                        let mut clients = clients.lock().unwrap();
                        for client in clients.iter_mut() {
                            dbg!("{client}");
                            if client.peer_addr().unwrap() != thread_addr {
                                client.write_all(input.as_bytes()).unwrap();
                            }
                        }
                    }
                    Err(_) => break,
                }
            }

            stream
                .shutdown(std::net::Shutdown::Both)
                .expect("Unable to shutdown");
            println!("Client ({thread_addr}) discounted");
        });
    }

    Ok(())
}
