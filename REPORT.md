# Zama Assignment Report

## Introduction

This report describes the approach, design decisions, and future improvements for a Rust-based client-server application that utilizes Merkle trees to ensure file integrity. The application allows a client to upload files to a server, delete local copies, and later download and verify the files' integrity.

## Approach

### Client-Server Interaction

The client-server interaction is implemented using HTTP requests. The client uploads files, requests proofs, and verifies the integrity of downloaded files using HTTP endpoints exposed by the server. This approach was chosen for its simplicity and ease of debugging.

### Merkle Tree Implementation

The Merkle tree implementation was enhanced by using old code from a different repository and fixing some issues with additional tests. This ensured the robustness and reliability of the Merkle tree functionality. The code also leverages the use of compact multiproofs, which can be further utilized for future improvements. Detailed descriptions of the Merkle tree components are provided in the implementation details section.

### Docker and Docker Compose

Docker and Docker Compose are used to containerize the client and server applications, ensuring a consistent environment for development and testing. The server and client are defined as services in `docker-compose.yml`, with the server starting first and the client depending on the server.

## Implementation Details

### Client

The client performs the following operations:
1. **Upload Files**: Reads files from the `data/` directory, computes a Merkle tree root hash, uploads the files to the server, and deletes the local copies.
2. **Store Merkle Root**: Stores the computed Merkle tree root hash locally.
3. **Download and Verify Files**: Requests files and their Merkle proofs from the server, saves the downloaded files, and verifies their integrity using the stored Merkle root hash and the received proof.

### Server

The server provides the following functionalities:
1. **Store Files**: Receives and stores files uploaded by the client.
2. **Provide Proofs**: Generates and provides Merkle proofs for requested files.
3. **Hello Endpoint**: A simple endpoint to check if the server is running.

### Merkle Tree

The Merkle tree implementation in this project includes several key components and functionalities:

- **Hash Computation**: A helper function `hash` that uses Rust's built-in hashing to compute the hash of any data type. This function is used to generate the hash values for the data blocks that form the leaves of the Merkle tree.
- **Tree Construction**: The function `calculate_merkle_root` constructs the Merkle tree from a list of file hashes (or words in a sentence) and recursively computes the Merkle root. It ensures the base layer is a power of two by padding with empty strings if necessary.
- **Proof Generation**: The function `generate_proof` creates a Merkle proof for a specific data block. It returns the Merkle root and a list of sibling nodes that can be used to verify the integrity of the data block.
- **Proof Verification**: The function `validate_proof` checks whether a given data block is part of the Merkle tree by using the provided Merkle proof and comparing the computed root with the stored Merkle root.
- **Compact Multiproof**: The project supports the generation and validation of compact multiproofs through `generate_compact_multiproof` and `validate_compact_multiproof`. Compact multiproofs allow the verification of multiple data blocks efficiently by providing only the necessary hashes.

## What Went Well

- **Simple and Clear Architecture**: The client-server architecture using HTTP is straightforward and easy to understand.
- **Merkle Tree Implementation**: Reusing old code for Merkle tree implementation allowed me to refresh my memories into its workings and ensured that the solution met the specific requirements.
- **Containerization**: Using Docker and Docker Compose simplified the setup and ensured consistency across different environments.
- **Improved Testing**: Adding more tests helped identify and fix issues in the Merkle tree implementation, enhancing its reliability.

## Challenges and Solutions

### Synchronization Issues

One of the main challenges was ensuring that the server was fully operational before the client started. Initially, using `docker-compose run client` led to synchronization issues as it did not honor the `depends_on` directive. The solution was to use `docker-compose up client`, which correctly handled service dependencies and ensured the client only started after the server was ready.

Another issue was with accessing `localhost:8000` versus accessing `server:8000`. The initial configuration attempted to access the server using `localhost:8000`, which is not accessible within the Docker network. Changing the URLs to `http://server:8000` ensured that the client could correctly reach the server within the Docker Compose network.

Additionally, another issue encountered was with the data retrieval format. The server returned data with escaped newline characters (`\\n`) and quotes (`"`) that required additional parsing on the client side to properly format the data.

### Ordering Hashes in the Merkle Tree

Furthermore, there was an issue with the ordering of the hashes. Currently, the hashes are ordered by filenames prior to hashing, and the correct index is found manually. A better approach would be to handle this ordering directly within the Merkle tree implementation to ensure consistency and accuracy.

## Shortcomings and Future Improvements

### Shortcomings

- **Handling Large Files**: The current implementation may not handle large files efficiently due to memory limitations.
- **Persistent Storage**: The server does not persist files and Merkle tree data between restarts, leading to data loss if the server is restarted.
- **Limited Error Handling**: The network communication lacks robust error handling and retries.

### Future Improvements

- **Chunking Large Files**: Implement chunking for large files to handle them more efficiently and avoid memory issues.
- **Persistent Storage**: Add persistent storage for the server to retain files and Merkle tree data across restarts, ensuring data durability.
- **Enhanced Error Handling**: Improve error handling and implement retries for network communication to handle transient errors more gracefully.
- **Integration Tests**: Add more comprehensive integration tests to cover different scenarios and ensure the robustness of the application.
- **Utilize Compact Multiproof**: Further leverage compact multiproofs for efficient proof management and verification.

## Conclusion

Thank you for reading through this report!
