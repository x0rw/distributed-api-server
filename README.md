# This description is outdated
# This description is outdated
# This description is outdated
A simple, lightweight HTTP server written in **pure Rust**, designed for handling basic HTTP `GET` and `POST` requests. This server supports routing and preloading routes, making it a great choice for small web applications or as a foundation for building more complex services.

## Features
- ✔️  Supports HTTP `GET` and `POST` requests
- ✔️  Custom Error handling
- Each Service(Node) can connect to the Api Gateway
- Each Service periodically emit a heartbeat to the Gateway to stay alive
- The Api gateway keeps track of living nodes, every node can register multiple routes
- The Api gateway can route traffic to a spesific service

 multithreading is not implemented yet

## Getting Started

### Prerequisites

Make sure you have **Rust** installed on your machine. You can install Rust by following the instructions on the official website: https://www.rust-lang.org/learn/get-started

### Installation

Clone the repository:

```bash
git clone https://github.com/x0rw/Rust-Multithreaded-Server.git
cd Rust-Multithreaded-Server/server
```

Build the project:

```bash
cargo build --release
```

### Usage

Run the server:

```bash
cargo run
```

By default, the server listens on `http://localhost:1111`.


## Contributing

If you'd like to contribute, feel free to fork the repository and submit a pull request with improvements, bug fixes, or new features.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
