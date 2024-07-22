# Zama Assignment

This repository contains a Rust implementation of a client-server application that utilizes Merkle trees to ensure file integrity. The application allows a client to upload files to a server, delete local copies, and later download and verify the files' integrity.

## Overview

Imagine a client has a large set of potentially small files {F0, F1, …, Fn} and wants to upload them to a server and then delete its local copies. The client wants, however, to later download an arbitrary file from the server and be convinced that the file is correct and is not corrupted in any way (in transport, tampered with by the server, etc.).

This project implements:
- A client that computes a single Merkle tree root hash and keeps it on its disk after uploading the files to the server and deleting its local copies.
- A server that stores the files and provides the requested file and its Merkle proof.
- A Merkle tree to support the above functionalities, implemented from scratch with a library for the underlying hash functions.

The client can request the i-th file Fi and a Merkle proof Pi for it from the server. The client uses the proof and compares the resulting root hash with the one it persisted before deleting the files - if they match, the file is correct.

## Features

- **Client**: 
  - Uploads files to the server.
  - Deletes local copies after uploading.
  - Stores the Merkle tree root hash locally.
  - Requests files and their proofs from the server.
  - Verifies the integrity of the downloaded files using the stored Merkle tree root hash and the received proof.

- **Server**:
  - Receives and stores files.
  - Provides requested files and their Merkle proofs.

- **Merkle Tree**:
  - Implemented from scratch.
  - Supports proof generation and verification.

## Getting Started

### Prerequisites

- Rust (latest stable version recommended)
- Docker and Docker Compose (for setting up the server)

### Setting Up the Project

1. **Clone the repository**:
    ```sh
    git clone https://github.com/poppyseedDev/zama_assignment.git
    cd zama_assignment
    ```

### Directory Structure

```
zama_assignment/
├── Cargo.lock
├── Cargo.toml
├── client/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── Dockerfile.client
├── Dockerfile.server
├── merkle_tree/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── server/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── docker-compose.yml
```

### Building and Running the Application

1. **Navigate to your project directory** (where `docker-compose.yml` is located).
2. **Build and run the application** using Docker Compose:
    ```sh
    docker-compose up --build
    ```

## How It Works

1. **Client**:
   - Reads files from the `data/` directory.
   - Computes a Merkle tree root hash from the files and uploads the files to the server.
   - Deletes the local copies of the files and stores the Merkle tree root hash.
   - Can later request a file and its proof from the server to verify the file's integrity.

2. **Server**:
   - Stores the uploaded files.
   - Provides the requested file and its Merkle proof upon request.

3. **Merkle Tree**:
   - Constructs the Merkle tree from the file hashes.
   - Generates proofs for the files.
   - Verifies the proofs against the stored root hash.

## Shortcomings and Future Improvements

### Shortcomings
- The current implementation may not handle large files efficiently.
- Network communication lacks robust error handling and retries.
- The server does not persist the files and Merkle tree between restarts.

### Future Improvements
- Implement chunking for large files to handle them efficiently.
- Enhance error handling and implement retries for network communication.
- Add persistent storage for the server to retain files and Merkle tree data across restarts.
- Improve test coverage and add integration tests.

## Report

A detailed report explaining the approach, design decisions, and future improvements is included in the `REPORT.md` file in the repository.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### Build and Run

1. **Navigate to the root of your workspace**:

    ```sh
    cd /path/to/merkle_tree_app
    ```

2. **Build the entire workspace**:

    ```sh
    cargo build
    ```

3. **Run the server**:

    ```sh
    cargo run --manifest-path server/Cargo.toml
    ```

4. **Run the client setup script**:

    ```sh
    cargo run --bin setup_files --manifest-path client/Cargo.toml
    ```

5. **Run the main client**:

    ```sh
    cargo run --bin client --manifest-path client/Cargo.toml
    ```

