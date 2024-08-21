# Testing and Deployment 🧪🚀

Testing your code and deploying it is crucial to ensure everything works correctly.

## Testing

### Unit Tests

We’ve added unit tests to ensure the Merkle tree functions work correctly. Here’s how you can run them:

```bash
cargo test
```

### Integration Tests

Our `./scripts/e2e_test.sh` script performs an end-to-end test, running the server and client and verifying the file operations.

## Deployment

You can deploy the project using Docker. Here’s a quick guide:

1. **Build Docker Images**:
   ```bash
   docker-compose build
   ```

2. **Run the Containers**:
   ```bash
   docker-compose up -d
   ```

3. **Check the Logs**:
   ```bash
   docker-compose logs
   ```

Your project is now live! 🎉
