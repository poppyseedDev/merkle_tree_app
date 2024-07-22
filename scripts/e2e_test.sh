#!/bin/bash

# Start Docker Compose
docker-compose up -d --build

# Wait for the services to be up
sleep 10

# Upload files
curl -X POST http://localhost:8000/upload -H "Content-Type: application/json" -d '{
    "file1.txt": "This is the content of file1.",
    "file2.txt": "File2 contains different content."
}'

# Get proof for a file
response=$(curl -s http://localhost:8000/proof/file1.txt)
echo "Proof response: $response"

# Extract root and proof from response
root=$(echo $response | jq -r '.root')
proof=$(echo $response | jq -r '.proof')

# Validate proof using the client logic
client_response=$(curl -s http://localhost:8000/download/file1.txt)
echo "Client download response: $client_response"

# Assuming the client has a function to validate proof
cargo run --bin client --manifest-path client/Cargo.toml -- validate "$client_response" "$root" "$proof"

# Clean up
docker-compose down
