use reqwest::Client;
use std::fs;
use merkle_tree::{calculate_merkle_root, validate_proof, generate_proof, hash, SiblingNode};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize)]
struct ProofResponse {
    root: u64,
    proof: Vec<SiblingNode>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <server_url>", args[0]);
        std::process::exit(1);
    }
    let server_url = &args[1];
    println!("Server URL: {}", server_url);

    let client = Client::new();
    let files: Vec<String> = vec!["./data/file1.txt", "./data/file2.txt", "./data/file3.txt"]
        .into_iter()
        .map(String::from)
        .collect();

    upload_files(&client, &files, server_url).await?;
    delete_files(&files)?;
    save_merkle_root(&client, server_url).await?;
    download_and_verify_files(&client, &files, server_url).await?;
    
    Ok(())
}

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

fn delete_files(files: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    for file in files {
        if let Err(e) = fs::remove_file(file) {
            eprintln!("Failed to delete {}: {}", file, e);
        } else {
            println!("Deleted {}", file);
        }
    }

    Ok(())
}

async fn save_merkle_root(client: &Client, server_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let res = client.get(format!("{}/merkle_root", server_url))
        .send()
        .await?
        .text()
        .await?;

    let res = res.trim_matches('"');  // Remove the additional quotes
    let root_prefix = "Root: ";
    if let Some(pos) = res.find(root_prefix) {
        let root_str = &res[pos + root_prefix.len()..];
        if let Ok(root_hash) = root_str.parse::<u64>() {
            println!("Merkle root: {}", root_hash);
            fs::write("./data/merkle_root.txt", root_hash.to_le_bytes())?;
        } else {
            eprintln!("Failed to parse Merkle root");
        }
    } else {
        eprintln!("Merkle root not found in response");
    }

    Ok(())
}

async fn download_and_verify_files(client: &Client, files: &[String], server_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    for file in files {
        let filename = file.rsplit('/').next().unwrap();

        let res = download_file(client, filename, server_url).await?;
        fs::write(file, &res)?;

        let proof_response = get_proof(client, filename, server_url).await?;

        let stored_root = fs::read("merkle_root.txt")?;
        let stored_root = u64::from_le_bytes(stored_root[..8].try_into().unwrap());

        println!("Stored root: {}", stored_root);
        println!("Generated root: {}", proof_response.root);
        println!("Res: {}", res);
        println!("Res: {}", &hash(&res).to_string());
        println!("Proof: {:?}", proof_response.proof);
        if validate_proof(&stored_root, &hash(&res).to_string(), proof_response.proof) {
            println!("File {} is verified!", filename);
        } else {
            println!("File {} verification failed!", filename);
        }
    }

    Ok(())
}

async fn download_file(client: &Client, filename: &str, server_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let res = client.get(format!("{}/download/{}", server_url, filename))
        .send()
        .await?
        .text()
        .await?;

    let res = res.trim_matches('"').replace("\\n", "\n");
    println!("Downloaded {}", filename);

    Ok(res)
}

async fn get_proof(client: &Client, filename: &str, server_url: &str) -> Result<ProofResponse, Box<dyn std::error::Error>> {
    let proof_response = client.get(format!("{}/proof/{}", server_url, filename))
        .send()
        .await?
        .json()
        .await?;

    Ok(proof_response)
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
