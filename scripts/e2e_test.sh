#!/bin/bash

set -e

# Generate sample files
cargo run --bin setup --manifest-path client/Cargo.toml

# Build and run the server detached
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
