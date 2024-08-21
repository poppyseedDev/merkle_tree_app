# File Sharing Implementation 📁

In this section, we'll go through how to implement file sharing using the Rust client-server model.

## Client-Server Overview

The client is responsible for:

1. **Uploading Files**: Send files to the server.
2. **Storing the Merkle Root**: Save the Merkle tree's root hash locally.
3. **Requesting Files**: Download files from the server when needed.
4. **Verifying Integrity**: Ensure the downloaded files match the original using Merkle proofs.

The server is responsible for:

1. **Storing Files**: Keep the uploaded files safe.
2. **Providing Proofs**: Generate Merkle proofs for file verification.

## Implementing the Client

Let's break down the client code:

### Uploading Files

```rust
async fn upload_files(client: &Client, files: &[String], server_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut upload_data = HashMap::new();
    for file in files {
        let data = fs::read_to_string(file)?;
        let filename = file.rsplit('/').next().unwrap().to_string();
        upload_data.insert(filename, data);
    }

    let res = client.post(format!("{}/upload", server_url))
        .json(&upload_data)
        .send()
        .await?
        .text()
        .await?;

    println!("Uploaded files: {:?}", res.trim_matches('"'));
    Ok(())
}
```

### Verifying Files

```rust
async fn download_and_verify_files(client: &Client, files: &[String], server_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    for file in files {
        let filename = file.rsplit('/').next().unwrap();
        let res = download_file(client, filename, server_url).await?;
        fs::write(file, &res)?;

        let proof_response = get_proof(client, filename, server_url).await?;
        let stored_root = fs::read("merkle_root.txt")?;
        let stored_root = u64::from_le_bytes(stored_root[..8].try_into().unwrap());

        if validate_proof(&stored_root, &hash(&res).to_string(), proof_response.proof) {
            println!("File {} is verified!", filename);
        } else {
            println!("File {} verification failed!", filename);
        }
    }
    Ok(())
}
```

### Handling Data Format Issues

Sometimes the data from the server comes in a format that needs additional parsing, like escaped newlines (`\\n`) and quotes (`"`). The code above handles these with:

```rust
let res = res.trim_matches('"').replace("\\n", "\n");
```

## Implementing the Server

### Receiving Files

The server receives files from the client, stores them, and computes the Merkle root. Here's how:

```rust
#[post("/upload")]
async fn upload(file: web::Json<HashMap<String, String>>, state: web::Data<AppState>) -> impl Responder {
    let mut files = state.files.lock().unwrap();
    for (filename, content) in file.into_inner() {
        let file_hash = hash(&content);
        files.insert(filename.clone(), FileData { content, hash: file_hash.clone() });
    }

    let concatenated_hashes = get_sorted_concatenated_hashes(&files);
    let root = merkle_tree::calculate_merkle_root(&concatenated_hashes);

    let mut merkle_root = state.merkle_root.lock().unwrap();
    *merkle_root = Some(root);

    HttpResponse::Ok().json(format!("Root: {}", root))
}
```

### Generating Proofs

The server can generate and return Merkle proofs for requested files:

```rust
#[get("/proof/{filename}")]
async fn proof(file_name: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let files = state.files.lock().unwrap();
    if let Some(file_data) = files.get(filename) {
        let (generated_root, proof) = generate_proof(file_data.content, index);
        return HttpResponse::Ok().json(ProofResponse { root: generated_root, proof });


    }
    HttpResponse::NotFound().finish()
}
```

### Issues and Improvements

- **Data Format**: Handling issues like `\\n` and `"` that required additional parsing.
- **Hash Ordering**: Ordering by filenames manually before hashing. A better approach would be to handle this directly in the Merkle tree implementation.

