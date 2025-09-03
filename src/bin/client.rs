use std::{
    env,
    io::{Write, stdin, stdout},
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut args = env::args();

    // Extract args
    args.next();
    let command = args.next().expect("Command not found");
    if command != "connect" {
        return Ok(());
    }
    let ip_addr_port = args.next().expect("IpAddr is expected");

    // Connect to Server
    let socket = TcpStream::connect(ip_addr_port).await?;
    let (mut reader, mut writer) = socket.into_split();

    // Set username and write to server
    let mut username = String::new();
    println!("Enter your username: ");
    stdin().read_line(&mut username).expect("Invalid username");
    writer.write_all(username.as_bytes()).await?;

    println!("Joined Server (\"quit\" to leave)");

    // Channels for inter-thread communication (for 2 threads)
    let (server_tx, mut main_rx) = mpsc::channel::<String>(256);
    let (input_tx, mut input_rx) = mpsc::channel::<String>(256);

    // Thread: for reading from server
    tokio::spawn(async move {
        let mut buffer = [0; 512];
        loop {
            match reader.read(&mut buffer).await {
                Ok(0) => {
                    let _ = server_tx.send("Server closed connections".into()).await;
                    break;
                }
                Ok(bytes_read) => {
                    let input = String::from_utf8_lossy(&buffer[..bytes_read]);
                    print!("{input}");
                    let _ = stdout().flush();
                }
                Err(_) => {
                    let _ = server_tx.send("Connection error".into()).await;
                    break;
                }
            }
        }
    });

    // Thread: for reading user input
    tokio::spawn(async move {
        loop {
            let mut input = String::new();
            if stdin().read_line(&mut input).is_err() {
                break;
            }

            if input.trim().eq_ignore_ascii_case("quit") {
                let _ = input_tx.send("quit".into()).await;
                break;
            }

            let _ = input_tx.send(input).await;
        }
    });

    // Main thread: continuously tracking for exiting msg from channels
    loop {
        if let Some(server_msg) = main_rx.recv().await {
            println!("{server_msg}");
            break;
        }

        if let Some(input_msg) = input_rx.recv().await {
            if input_msg.trim().eq_ignore_ascii_case("quit") {
                println!("Leaved Server");
                break;
            }
            writer.write_all(input_msg.as_bytes()).await?;
        }
    }

    Ok(())
}
