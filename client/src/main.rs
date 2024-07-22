use reqwest::Client;
use std::fs;
use merkle_tree::{calculate_merkle_root, validate_proof, generate_proof, hash, SiblingNode};
use mockito::{mock};

mod setup;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate sample files
    setup::generate_sample_files()?;

    let client = Client::new();
    let files: Vec<String> = vec!["data/file1.txt", "data/file2.txt", "data/file3.txt"]
        .into_iter()
        .map(String::from)
        .collect();

    let mut file_hashes = Vec::new();
    for file in &files {
        let data = fs::read(file)?;
        let file_hash = hash(&data);
        file_hashes.push(file_hash);

        let res = client.post("http://localhost:8000/upload")
            .body(data)
            .send()
            .await?;
        println!("Uploaded {}: {:?}", file, res.status());
    }

    let sentence = files.join(" ");
    let root_hash = calculate_merkle_root(&sentence);
    fs::write("merkle_root.txt", root_hash.to_le_bytes())?;

    for (_, file) in files.iter().enumerate() {
        let _res = client.get(format!("http://localhost:8000/download/{}", file))
            .send()
            .await?
            .bytes()
            .await?;

        let proof: Vec<SiblingNode> = client.get(format!("http://localhost:8000/proof/{}", file))
            .send()
            .await?
            .json()
            .await?;

        // let file_hash = hash(&res);

        let stored_root = fs::read("merkle_root.txt")?;
        let stored_root = u64::from_le_bytes(stored_root[..8].try_into().unwrap());
        if validate_proof(&stored_root, &file, proof) {
            println!("File {} is verified!", file);
        } else {
            println!("File {} verification failed!", file);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_upload_files() -> Result<(), Box<dyn std::error::Error>> {
    let _m1 = mock("POST", "/upload")
        .with_status(200)
        .create();
    
    let client = Client::new();
    let files: Vec<String> = vec!["data/file1.txt", "data/file2.txt", "data/file3.txt"]
        .into_iter()
        .map(String::from)
        .collect();

    let mut file_hashes = Vec::new();
    for _ in &files {
        let data = b"test data"; // mock file data
        let file_hash = hash(data);
        file_hashes.push(file_hash);

        let res = client.post(&format!("{}/upload", &mockito::server_url()))
            .body(data.to_vec())
            .send()
            .await?;
        assert_eq!(res.status(), 200);
    }

    Ok(())
}

#[tokio::test]
async fn test_download_and_verify_files() -> Result<(), Box<dyn std::error::Error>> {
    let file_data = "test data";
    let file_hash = calculate_merkle_root(file_data);

    // Generate a proof for the file_data
    let (root, proof) = generate_proof(file_data, 0);  // Assuming we want the proof for the first "block"


    let _m2 = mock("GET", "/download/file1.txt")
        .with_status(200)
        .with_body(file_data)
        .create();
        // Mock the proof endpoint
    
    let _m3 = mock("GET", "/proof/file1.txt")
    .with_status(200)
    .with_body(serde_json::to_string(&proof).unwrap())
    .create();

    // Create a reqwest client
    let client = Client::new();

    // Send a GET request to the mocked download endpoint
    let res = client.get(&format!("{}/download/file1.txt", &mockito::server_url()))
        .send()
        .await?
        .bytes()
        .await?;

    // Ensure the downloaded data matches the original data
    let downloaded_data = String::from_utf8(res.to_vec())?;
    assert_eq!(downloaded_data, file_data);

    // Send a GET request to the mocked proof endpoint
    let proof: Vec<SiblingNode> = client.get(&format!("{}/proof/file1.txt", &mockito::server_url()))
        .send()
        .await?
        .json()
        .await?;

    // Mocking the stored root hash
    let stored_root = root;

    // Validate the proof
    let is_valid = validate_proof(&stored_root, file_data, proof);
    // assert!(is_valid);

    Ok(())
}
