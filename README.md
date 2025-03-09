# Microservice Architecture in Rust
A microservice architecture in pure Rust is a distributed system where independent services communicate over the network, each handling a specific function. Rust’s performance, safety, and minimal runtime overhead make it ideal for building lightweight, 
Efficient, and secure microservices. 
By leveraging async programming, message passing, and API gateways, Rust-based microservices can achieve scalability, fault tolerance, and high concurrency while maintaining strong type safety and memory safety.

## Cureent Features
- ✅   HTTP parsing
- ✅   Each Service(Node) can connect to the Api Gateway
- ✅   Each Service periodically emit a heartbeat to the Gateway to stay alive
- ✅   The Api gateway keeps track of living nodes, every node can register multiple endpoints
- ✅   The Api gateway can route traffic to a specific service
- ❌   Internal communication between services
- ❌   Monitoring
- ❌   Rate limiting and load balancing
- ❌   SSL encryption layer
- ❌   Async requests handling(partially but branched out)

## Getting Started

### Prerequisites

Make sure you have **Rust** installed on your machine. You can install Rust by following the instructions on the official website: https://www.rust-lang.org/learn/get-started

### Installation

Clone the repository:

```bash
git clone https://github.com/x0rw/Rust-Multithreaded-Server.git
cd Rust-Multithreaded-Server/
make
```

Build the project:

```bash
cargo build --release
```

### Usage

Run the Api gateway:

```bash
make gateway
```
Run Nodes:

```bash
make node1
make node2
make node3
make node4
```
### Custom usage 
```bash
cargo run --bin sync_main
   Usage: {} node-name node_port node_inc_port gateway_port

cargo run --bin gateway #the gateway by default listen on port 1111

```
check the EchoController in base/src/controller.rs

By default, the gateway listens on `http://localhost:1111`.


## Contributing

If you'd like to contribute, feel free to fork the repository and submit a pull request with improvements, bug fixes, or new features.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
