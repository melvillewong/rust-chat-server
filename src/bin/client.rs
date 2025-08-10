use std::{
    env,
    io::{Read, Write, stdin, stdout},
    net::TcpStream,
    sync::mpsc,
    thread,
};

fn main() -> std::io::Result<()> {
    let mut args = env::args();

    args.next();
    let command = args.next().expect("Command not found");
    if command != "connect" {
        return Ok(());
    }
    let ip_addr_port = args.next().expect("IpAddr is expected");

    // Connect to Server
    let mut stream = TcpStream::connect(ip_addr_port)?;
    println!("Joined Server (\"quit\" to leave)");

    // Clone stream and move into one thread to perform reading broadcasted message
    let mut read_stream = stream.try_clone()?;

    let (sender, receiver) = mpsc::channel();

    // Spawning thread for reading
    thread::spawn(move || {
        let mut buffer = [0; 512];
        loop {
            match read_stream.read(&mut buffer) {
                Ok(0) => {
                    println!("Server closed connection");
                    let _ = sender.send(());
                    break;
                }
                Ok(bytes_read) => {
                    let input = String::from_utf8_lossy(&buffer[..bytes_read]);
                    println!("{input}");
                }
                Err(_) => {
                    let _ = sender.send(());
                    break;
                }
            }
        }
        read_stream
            .shutdown(std::net::Shutdown::Both)
            .expect("Unable to shutdown connection");
        println!("Leaved Server");
    });

    // Main thread for writing
    loop {
        if receiver.try_recv().is_ok() {
            break;
        }

        print!("> ");
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;

        if input.trim().eq_ignore_ascii_case("quit") {
            stream
                .shutdown(std::net::Shutdown::Both)
                .expect("Unable to shutdown connection");
            println!("Leaved Server");
            break;
        }

        stream.write_all(input.as_bytes())?;
    }

    Ok(())
}
