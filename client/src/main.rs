use reqwest::Client;
use std::fs;
use merkle_tree::{calculate_merkle_root, validate_proof, generate_proof, hash, SiblingNode};
use std::env;
use std::collections::HashMap;
use serde::{Deserialize};

#[derive(Deserialize)]
struct ProofResponse {
    root: u64,
    proof: Vec<SiblingNode>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 4 && args[1] == "validate" {
        let file_path = &args[2];
        let root: u64 = args[3].parse().unwrap();
        let proof: Vec<SiblingNode> = serde_json::from_str(&args[4]).unwrap();
        
        let data = fs::read_to_string(file_path)?;
        if validate_proof(&root, &data, proof) {
            println!("File {} is verified!", file_path);
        } else {
            println!("File {} verification failed!", file_path);
        }
        return Ok(());
    }

    let client = Client::new();
    let files: Vec<String> = vec!["data/file1.txt", "data/file2.txt", "data/file3.txt"]
        .into_iter()
        .map(String::from)
        .collect();

    let mut upload_data = HashMap::new();
    for file in &files {
        let data = fs::read_to_string(file)?;
        upload_data.insert(file.clone(), data);
    }

    let res = client.post("http://localhost:8000/upload")
        .json(&upload_data)
        .send()
        .await?;
    println!("Uploaded files: {:?}", res.status());

    let sentence = upload_data.values().cloned().collect::<Vec<_>>().join(" ");
    let root_hash = calculate_merkle_root(&sentence);
    fs::write("merkle_root.txt", root_hash.to_le_bytes())?;

    for file in &files {
        let res = client.get(format!("http://localhost:8000/download/{}", file))
            .send()
            .await?
            .text()
            .await?;
        println!("Downloaded {}: {:?}", file, res);

        let proof_response: ProofResponse = client.get(format!("http://localhost:8000/proof/{}", file))
            .send()
            .await?
            .json()
            .await?;

        let stored_root = fs::read("merkle_root.txt")?;
        let stored_root = u64::from_le_bytes(stored_root[..8].try_into().unwrap());
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

    let is_valid = validate_proof(&proof_response.root, file_data, proof_response.proof);
    assert!(is_valid);

    Ok(())
}
