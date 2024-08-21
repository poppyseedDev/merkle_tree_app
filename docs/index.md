#### **2. Project Setup (`setup.md`)**

# Project Setup 🛠️

Let's get your project up and running! We'll guide you step-by-step.

## Prerequisites

Before we start, make sure you have:

- **Rust**: Install the latest stable version of Rust. Follow the instructions [here](https://www.rust-lang.org/tools/install) if you haven't installed it yet.
- **Docker & Docker Compose**: Install Docker and Docker Compose. Follow the instructions [here](https://docs.docker.com/get-docker/).

## Setting Up the Project

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/poppyseedDev/zama_assignment.git
   cd zama_assignment
   ```

2. **Install Dependencies**:
   ```bash
   cargo build --release
   ```

3. **Prepare Docker**:
   We'll use Docker to run our server and client. Make sure Docker is running on your machine.
   ```bash
   docker-compose up --build
   ```

4. **Run the Project**:
   Your project is now set up! Use the following command to run the project:
   ```bash
   ./scripts/e2e_test.sh
   ```
   This script will run the server and client, perform file operations, and verify integrity using Merkle trees.

Congratulations! 🎉 You've successfully set up the project.
