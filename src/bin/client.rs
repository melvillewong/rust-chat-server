use std::{
    env,
    io::{Read, Write, stdin, stdout},
    net::TcpStream,
    sync::mpsc,
    thread,
};

fn main() -> std::io::Result<()> {
    let mut args = env::args();

    // Extract args
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

    // Channels for inter-thread communication (for 2 threads)
    let (server_tx, main_rx) = mpsc::channel::<String>();
    let (input_tx, input_rx) = mpsc::channel::<String>();

    // Thread: for reading from server
    thread::spawn(move || {
        let mut buffer = [0; 512];
        loop {
            match read_stream.read(&mut buffer) {
                Ok(0) => {
                    let _ = server_tx.send("Server closed connections".into());
                    break;
                }
                Ok(bytes_read) => {
                    let input = String::from_utf8_lossy(&buffer[..bytes_read]);
                    println!("{input}");
                }
                Err(_) => {
                    let _ = server_tx.send("Connection error".into());
                    break;
                }
            }
        }
    });

    // Thread: for reading user input
    thread::spawn(move || {
        loop {
            print!("> ");
            let _ = stdout().flush();

            let mut input = String::new();
            if stdin().read_line(&mut input).is_err() {
                break;
            }

            if input.trim().eq_ignore_ascii_case("quit") {
                let _ = input_tx.send("quit".into());
                break;
            }

            let _ = input_tx.send(input);
        }
    });

    // Main thread: continuously tracking for exiting msg from channels
    loop {
        if let Ok(server_msg) = main_rx.try_recv() {
            println!("{server_msg}");
            break;
        }

        if let Ok(input_msg) = input_rx.try_recv() {
            if input_msg.trim().eq_ignore_ascii_case("quit") {
                println!("Leaved Server");
                break;
            }
            stream.write_all(input_msg.as_bytes())?;
        }

        thread::sleep(std::time::Duration::from_millis(50));
    }

    Ok(())
}
