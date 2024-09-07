## PrustDB 🦀🔑

#### Peer to Peer Key-Value Database in Rust 

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![P2P](https://img.shields.io/badge/P2P-Network-blue?style=for-the-badge)
![Database](https://img.shields.io/badge/Database-KeyValue-green?style=for-the-badge)

A high-performance, peer-to-peer key-value database implemented in Rust. This project aims to provide a distributed, scalable, and efficient solution for storing and retrieving data in a decentralized network.

## 📚 Table of Contents

- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Architecture](#architecture)
- [Performance](#performance)
- [Security](#security)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## ✨ Features

- 🚀 High-performance key-value storage
- 🌐 Peer-to-peer network architecture
- 🔒 Secure data transmission with end-to-end encryption
- 🔄 Automatic data replication and load balancing
- 📊 Built-in metrics and monitoring
- 🧪 Comprehensive test suite
- 🔍 Efficient data querying and indexing
- 🔧 Configurable consistency levels
- 📈 Horizontal scalability

## 🚀 Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- OpenSSL development libraries (for cryptography support)

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/p2p_key_db.git
   cd p2p_key_db
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run tests to ensure everything is working correctly:
   ```
   cargo test
   ```

## 🔧 Usage

1. Start a node:
   ```
   cargo run --release -- --port 8000 --data-dir /path/to/data
   ```

2. Connect to an existing network:
   ```
   cargo run --release -- --port 8001 --peer 127.0.0.1:8000 --data-dir /path/to/data
   ```

3. Use the API to interact with the database:
   ```rust
   use prustdb_client::Client;

   async fn example() -> Result<(), Box<dyn std::error::Error>> {
       let client = Client::connect("127.0.0.1:8000").await?;
       
       // Set a value
       client.set("key", "value").await?;
       
       // Get a value
       let value = client.get("key").await?;
       println!("Value: {:?}", value);

       Ok(())
   }
   ```

For more detailed usage instructions, API documentation, and examples, please refer to our [documentation](https://docs.prustdb.io).

## 🏗️ Architecture

Our P2P Key-Value Database is built on a distributed hash table (DHT) architecture, similar to Kademlia. Here's a high-level overview:

1. **Node Identification**: Each node in the network is assigned a unique ID using a cryptographically secure random number generator.

2. **Key-Value Storage**: Data is stored as key-value pairs, distributed across the network based on the proximity of the key to node IDs.

3. **Routing**: Nodes maintain a routing table of known peers, organized into k-buckets based on the XOR distance between node IDs.

4. **Lookup Protocol**: To find a key, nodes perform iterative lookups, querying progressively closer nodes until the key is found or the closest nodes are reached.

5. **Data Replication**: To ensure data availability and fault tolerance, key-value pairs are replicated across multiple nodes using a configurable replication factor.

6. **Network Joining**: New nodes join the network by bootstrapping through a known peer, then performing lookups for their own ID to populate their routing table.

7. **Load Balancing**: The DHT naturally distributes data and lookup load across the network, with additional active balancing mechanisms for hotspots.

8. **Consistency**: We implement tunable consistency levels, from eventual consistency to strong consistency, allowing users to choose the appropriate trade-off between performance and consistency for their use case.

9. **Caching**: An intelligent caching layer improves read performance for frequently accessed data.

10. **Compression**: Data is compressed before storage and transmission to reduce storage requirements and network usage.

## 🚀 Performance

PrustDB is designed for high performance:

- Asynchronous I/O for efficient resource utilization
- Custom memory allocator for reduced memory fragmentation
- Bloom filters for rapid non-existence proofs
- Optimized data structures for fast lookups and insertions
- Benchmarking suite for continuous performance monitoring

## 🔒 Security

Security is a top priority for PrustDB:

- End-to-end encryption for all data transmissions
- Authentication and authorization for all nodes and clients
- Regular security audits and penetration testing
- Sandboxing of untrusted code execution
- Detailed security documentation and best practices

## 🤝 Contributing

We welcome contributions from the community! Please check out our [Contributing Guidelines](CONTRIBUTING.md) for details on how to get started, our code of conduct, and the process for submitting pull requests.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- The Rust community for their excellent tools and libraries
- The authors of the Kademlia paper for the foundational concepts
- All our contributors and users for their support and feedback

For the latest updates, follow us on [Twitter](https://twitter.com/dinescodes) and star our [GitHub repository](https://github.com/dinxsh/prustdb)
