use reqwest::Client;
use std::fs;
use merkle_tree::{calculate_merkle_root, validate_proof, generate_proof, hash, SiblingNode};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct ProofResponse {
    root: u64,
    proof: Vec<SiblingNode>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let files: Vec<String> = vec!["./data/file1.txt", "./data/file2.txt", "./data/file3.txt"]
        .into_iter()
        .map(String::from)
        .collect();

    let mut upload_data = HashMap::new();
    for file in &files {
        let data = fs::read_to_string(file)?;
        let filename = file.rsplit('/').next().unwrap().to_string();
        upload_data.insert(filename, data);
    }

    let res = client.post("http://localhost:8000/upload")
        .json(&upload_data)
        .send()
        .await?
        .text()
        .await?;
        
    let res = res.trim_matches('"');  // Remove the additional quotes
    
    println!("Uploaded files: {:?}", res);

    let root_prefix = "Root: ";
    if let Some(pos) = res.find(root_prefix) {
        let root_str = &res[pos + root_prefix.len()..];
        if let Ok(root_hash) = root_str.parse::<u64>() {
            println!("Merkle root: {}", root_hash);
            fs::write("merkle_root.txt", root_hash.to_le_bytes())?;
        } else {
            eprintln!("Failed to parse Merkle root");
        }
    } else {
        eprintln!("Merkle root not found in response");
    }

    for file in &files {
        let filename = file.rsplit('/').next().unwrap();

        println!("http://localhost:8000/download/{}", filename);
        let res = client.get(format!("http://localhost:8000/download/{}", filename))
            .send()
            .await?
            .text()
            .await?;
        println!("Downloaded {}: {:?}", file, res);

        let proof_response: ProofResponse = client.get(format!("http://localhost:8000/proof/{}", filename))
            .send()
            .await?
            .json()
            .await?;

        let stored_root = fs::read("merkle_root.txt")?;
        let stored_root = u64::from_le_bytes(stored_root[..8].try_into().unwrap());
        println!("Stored root: {}", stored_root);
        println!("Res: {}", res);
        if validate_proof(&stored_root, &res, proof_response.proof) {
            println!("File {} is verified!", file);
        } else {
            println!("File {} verification failed!", file);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_upload_files() -> Result<(), Box<dyn std::error::Error>> {
    let _m1 = mockito::mock("POST", "/upload")
        .with_status(200)
        .create();
    
    let client = Client::new();
    let files: Vec<String> = vec!["data/file1.txt", "data/file2.txt", "data/file3.txt"]
        .into_iter()
        .map(String::from)
        .collect();

    let mut upload_data = HashMap::new();
    for file in &files {
        let data = "test data"; // mock file data
        upload_data.insert(file.clone(), data.to_string());
    }

    let res = client.post(&format!("{}/upload", &mockito::server_url()))
        .json(&upload_data)
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    Ok(())
}

#[tokio::test]
async fn test_download_and_verify_files() -> Result<(), Box<dyn std::error::Error>> {
    let file_data = "test data";
    let file_hash = calculate_merkle_root(file_data);

    // Generate a proof for the file_data
    let (root, proof) = generate_proof(file_data, 0);  // Assuming we want the proof for the first "block"


    let proof_response = ProofResponse {
        root,
        proof,
    };

    let _m2 = mockito::mock("GET", "/download/file1.txt")
        .with_status(200)
        .with_body(file_data)
        .create();

    let _m3 = mockito::mock("GET", "/proof/file1.txt")
        .with_status(200)
        .with_body(serde_json::to_string(&proof_response).unwrap())
        .create();

    let client = Client::new();

    let res = client.get(&format!("{}/download/file1.txt", &mockito::server_url()))
        .send()
        .await?
        .text()
        .await?;

    assert_eq!(res, file_data);

    let proof_response: ProofResponse = client.get(&format!("{}/proof/file1.txt", &mockito::server_url()))
        .send()
        .await?
        .json()
        .await?;

    let is_valid = validate_proof(&proof_response.root, "test", proof_response.proof);
    assert!(is_valid);

    Ok(())
}
