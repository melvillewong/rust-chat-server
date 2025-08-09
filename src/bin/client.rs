use std::{env, io::Read, net::TcpStream};

fn main() -> std::io::Result<()> {
    let mut args = env::args();
    args.next();

    let command = args.next().expect("Command not found");
    if command != "connect" {
        return Ok(());
    }

    let ip_addr_port = args.next().expect("IpAddr is expected");

    let stream = TcpStream::connect(ip_addr_port)?;
    let mut stream = stream;

    let mut buffer = [0; 512];

    let bytes_read = stream.read(&mut buffer)?;
    let received = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("{}", received);

    // if let Ok(stream) = client
    Ok(())
}
