use actix_web::{web, App, HttpServer, Responder, post, get, HttpResponse};
use serde::{Serialize, Deserialize};
use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use merkle_tree::{hash, generate_proof, validate_proof, HashValue, SiblingNode, MerkleProof};

#[derive(Clone)]
struct FileData {
    content: String,
    hash: HashValue,
}

pub struct AppState {
    pub files: Arc<Mutex<HashMap<String, FileData>>>,
    pub merkle_root: Arc<Mutex<Option<HashValue>>>,
}

fn get_sorted_concatenated_hashes(files: &HashMap<String, FileData>) -> String {
    let mut sorted_filenames: Vec<&String> = files.keys().collect();
    sorted_filenames.sort();
    sorted_filenames.iter()
        .map(|&filename| files[filename].hash.clone().to_string())
        .collect::<Vec<_>>()
        .join(" ")
    // let mut hashes = files.values().map(|data| data.hash.clone().to_string()).collect::<Vec<_>>().join(" ")
}

#[post("/upload")]
async fn upload(file: web::Json<HashMap<String, String>>, state: web::Data<AppState>) -> impl Responder {
    let mut files = state.files.lock().unwrap();
    let mut hashes: Vec<String> = Vec::new();

    for (filename, content) in file.into_inner() {
        let file_hash = hash(&content);
        println!("Content: {}", content);
        files.insert(filename.clone(), FileData { content, hash: file_hash.clone() });
        hashes.push(file_hash.to_string());
    }

    // Recalculate Merkle root
    let concatenated_hashes = get_sorted_concatenated_hashes(&files);
    println!("concatenated_hashes: {}", concatenated_hashes);
    // TODO: it would be better to use calculate_merkle_root_rec(hashes) directly here
    let root = merkle_tree::calculate_merkle_root(&concatenated_hashes);

    let mut merkle_root = state.merkle_root.lock().unwrap();
    *merkle_root = Some(root);

    HttpResponse::Ok().json(format!("Root: {}", root))
}

#[get("/download/{filename}")]
async fn download(file_name: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let files = state.files.lock().unwrap();
    let filename = file_name.as_str().rsplit('/').next().unwrap_or("");
    if let Some(file_data) = files.get(filename) {
        HttpResponse::Ok().json(&file_data.content)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[derive(Deserialize, Serialize)]
struct ProofResponse {
    root: HashValue,
    proof: MerkleProof,
}

#[get("/proof/{filename}")]
async fn proof(file_name: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let files = state.files.lock().unwrap();
    let filename = file_name.as_str().rsplit('/').next().unwrap_or("");
    let merkle_root = state.merkle_root.lock().unwrap();
    
    if let Some(file_data) = files.get(filename) {
        if let Some(root) = &*merkle_root {
            let concatenated_hashes: String = get_sorted_concatenated_hashes(&files);
            println!("Concatenated hashes: {}", concatenated_hashes);

            // Create a sorted list of filenames to determine the index
            let mut sorted_filenames: Vec<&String> = files.keys().collect();
            sorted_filenames.sort();
            let index = sorted_filenames.iter().position(|&k| k == filename).unwrap();

            println!("Index: {}", index);
            let (generated_root, proof) = generate_proof(&concatenated_hashes, index);
            println!("Root: {:?}", generated_root);
            println!("Proof: {:?}", proof);
            
            let proof_response = ProofResponse {
                root: generated_root,
                proof: proof,
            };

            return HttpResponse::Ok().json(proof_response);
        }
    }
    HttpResponse::NotFound().finish()
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

pub fn create_app_state() -> web::Data<AppState> {
    web::Data::new(AppState {
        files: Arc::new(Mutex::new(HashMap::new())),
        merkle_root: Arc::new(Mutex::new(None)),
    })
}

pub fn configure_services(cfg: &mut web::ServiceConfig) {
    cfg.service(upload);
    cfg.service(download);
    cfg.service(proof);
    cfg.service(hello);
}
