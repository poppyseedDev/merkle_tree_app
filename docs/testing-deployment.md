# Testing and Deployment 🧪🚀

Ensuring your code is robust through testing and deploying it efficiently is key to building reliable applications. This section will guide you through testing your Merkle Tree application and deploying it using Docker.

## Testing

### Unit Tests 🧩

Unit tests are essential for verifying that individual components of your Merkle Tree implementation work as expected. We've included unit tests for the core Merkle Tree functions.

**Run Unit Tests:**

To execute the unit tests, simply run:

```bash
cargo test
```

This command will run all the unit tests in your project and output the results, helping you ensure that your code behaves as expected.

### Integration Tests 🔗

Integration tests check how different parts of your application work together. We've provided an end-to-end test script (`./scripts/e2e_test.sh`) that:

1. Starts the server.
2. Runs client operations like file upload, download, and verification.
3. Verifies that the operations perform correctly.

**Run the End-to-End Tests:**

You will be able to execute the script using:

```bash
./scripts/e2e_test.sh
```

This will simulate a full workflow, ensuring that your server and client interact correctly in a real-world scenario.


## Deployment

Deploying your application in a consistent and reproducible way is crucial. Docker makes this easy by containerizing your server and client, ensuring they run in the same environment regardless of where they're deployed.

### Docker Setup 🐳

We've provided a `docker-compose.yml` file to manage your server and client as Docker containers.

**Docker Compose Configuration:**

```yaml
version: '3'
services:
  server:
    build:
      context: .
      dockerfile: Dockerfile.server
    ports:
      - "8000:8000"
  client:
    build:
      context: .
      dockerfile: Dockerfile.client
    depends_on:
      - server
    volumes:
      - ./data:/app/data
```

### Dockerfiles 📦

**Server Dockerfile (`Dockerfile.server`):**

```Dockerfile
FROM rust:latest
WORKDIR /app
COPY . .

# Build the server binary
RUN cd /app/merkle_tree && cargo build --release
RUN cd /app/server && cargo build --release --bin server

EXPOSE 8000

CMD ["./target/release/server"]
```

**Client Dockerfile (`Dockerfile.client`):**

```Dockerfile
FROM rust:latest
WORKDIR /app
COPY . .

# Build and run the setup_files binary to create the data files
RUN cd /app/client && cargo build --release --bin setup
RUN ./target/release/setup

# Build the main client binary
RUN cd /app/merkle_tree && cargo build --release
RUN cd /app/client && cargo build --release --bin client

CMD ["./target/release/client", "http://server:8000"]
```

### End-to-End Test Script 🚀

The end-to-end test script (`./scripts/e2e_test.sh`) automates the process of testing your entire setup in a Docker environment.

**Script Details:**

```bash
#!/bin/bash

set -e

# Generate sample files
cargo run --bin setup --manifest-path client/Cargo.toml

# Build and run the server in detached mode
docker-compose up --build --force-recreate -d server

# Wait for the server to be fully up and running
echo "Waiting for server to start..."
until curl -s http://localhost:8000/hello; do
  echo "Waiting for server..."
  sleep 2
done

# Run client operations (handled by client Docker container)
docker-compose up --build --force-recreate client

# Stop and remove the Docker containers
docker-compose down
```

### Running Your Application Locally

If you prefer to run the application directly without Docker, follow these steps:

1. **Build the entire workspace:**

    ```bash
    cargo build --release
    ```

2. **Run the server:**

    ```bash
    cargo run --manifest-path server/Cargo.toml
    ```

3. **Run the client setup script:**

    ```bash
    cargo run --bin setup --manifest-path client/Cargo.toml
    ```

4. **Run the main client:**

    ```bash
    cargo run --bin client --manifest-path client/Cargo.toml http://localhost:8000
    ```

---

Your project is now ready for deployment and testing! 🎉 By following these steps, you ensure that your application is thoroughly tested and can be deployed consistently across different environments. Happy coding! 😊