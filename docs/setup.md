# Project Setup 🛠️

Let's get your Rust project up and running! Follow this step-by-step guide to set up the project structure, and you'll be coding in no time. 💻

## Prerequisites

Before we dive in, ensure you have the following tools installed on your machine:

- **Rust**: Install the latest stable version of Rust. You can follow the instructions [here](https://www.rust-lang.org/tools/install).
- **Docker & Docker Compose**: These tools will help you containerize your application. Install them by following the instructions [here](https://docs.docker.com/get-docker/).

### Step-by-Step Implementation

#### 1. Set Up the Rust Project

Start by creating a new Rust project:

```sh
cargo new merkle_tree_app
cd merkle_tree_app
```

This command creates a new directory `merkle_tree_app` with a basic Rust project structure.

#### 2. Create Separate Crates for Client and Server

We'll be working with three main parts: the Merkle Tree library, the client, and the server. Let's create them:

```sh
cargo new --lib merkle_tree
cargo new client
cargo new server
```

- `merkle_tree`: This crate will contain the Merkle Tree logic.
- `client`: This crate will handle the client-side operations, such as file uploads and integrity verification.
- `server`: This crate will manage file storage and proof generation on the server side.

#### 3. Update the Workspace Configuration

To manage these crates together, we'll configure a workspace. Open your `Cargo.toml` in the root directory and add the following:

```toml
[workspace]
members = [
    "merkle_tree",
    "client",
    "server",
]
```

This setup makes it easier to build, test, and manage dependencies across all parts of the project.

#### 4. Set Up Dependencies

Next, we'll need to add dependencies for HTTP requests, JSON handling, and async operations. Update the `Cargo.toml` files in the `client` and `server` crates with the following dependencies:

For `client/Cargo.toml`:
```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

For `server/Cargo.toml`:
```toml
[dependencies]
actix-web = "4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

This will set up the necessary tools to handle HTTP requests, JSON data, and asynchronous programming in Rust.

### Next Steps

With your project structure in place, you're ready to start implementing the core functionality! Head over to the next section to learn how to build a Merkle Tree from scratch. 🌳
