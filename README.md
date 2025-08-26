# 📨 Rust TCP Chat Server

A TCP-based chat server and client written in Rust.  
Supports multiple clients, message broadcasting, and demonstrates networking with Rust.

---

## ✨ Features
- Multi-client chat over TCP  
- Message broadcasting  
- Server & client implemented in Rust  
- Graceful client disconnects  

---

## 🛠 Tech Stack
- **Language**: Rust (Edition 2024)  
- **Standard Library**: `std::net` for TCP networking
- **External Crate**: `chrono = "0.4.41"` for timestamp

---

## 🚀 Installation & Setup

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (with `cargo` and `rustc`)

### Build
```bash
cargo build --release
```

---

## 💬 Usage

### Start the server (default port: `9000`):
```bash
cargo run --bin server
```

### Connect a client:
```bash
cargo run --bin client connect <server_port>
```

### Or use `netcat` to test:
```bash
nc localhost 7878
```

### Example session:
```bash
[Client1] Hello from Alice!
[Client2] Hi Alice, this is Bob.
```

---

## 📂 Project Structure
```bash
.
├── src
│   ├── server.rs    # TCP server logic
│   └── client.rs    # TCP client logic
│
├── Cargo.toml       # Dependencies & metadata
└── Cargo.lock
```

---

## 🔮 Future Improvements
- User authentication & nicknames
- Private messages
- Command system (`/quit`, `/nick`, etc.)
- WebSocket support
- GUI client
- (Optional) Async runtime: [Tokio](https://tokio.rs)  
