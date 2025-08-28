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
- **External Crate**:
  - `chrono = "0.4.41"` for timestamp
  - `tokio = { version = "1.47.1", features = ["full"] }` for async clients handling

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
│   ├── bin
│   │   ├── server.rs    # TCP server logic
│   │   └── client.rs    # TCP client logic
│   └── lib.rs
├── Cargo.toml    # Dependencies & metadata
└── Cargo.lock
```

---

## Design Decision

### Why use `into_split()` with `tokio::net::TcpStream`?

Challenge:

- Sharing the client connection safely across multiple tasks (for reading input and broadcasting message)
- Instead of cloning the entire `std::net::TcpStream` or wrapping it in `Arc<Mutex<TcpStream>>`, this projects uses `into_split()`.

Benefit:

- No wasted clone of the whole `TcpStream`: Only the `OwnedWriteHalf` is stored for broadcasting.
- No `Arc<Mutex<TcpStream>>` needed: Eliminates extra locks around reads/writes.
- No deadlocks: The read and write halves are independent by design.
- Cleaner broadcast: Lock the shared client list briefly, collect `OwnedWriteHalf`s, and send concurrently.

---

## 🔮 Future Improvements

- Command system (`/quit`, `/nick`, etc.)
- WebSocket support
- (Optional) Async runtime: [Tokio](https://tokio.rs)
