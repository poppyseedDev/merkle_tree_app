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

struct AppState {
    files: Arc<Mutex<HashMap<String, FileData>>>,
    merkle_root: Arc<Mutex<Option<HashValue>>>,
}

#[post("/upload")]
async fn upload(file: web::Json<HashMap<String, String>>, state: web::Data<AppState>) -> impl Responder {
    let mut files = state.files.lock().unwrap();
    for (filename, content) in file.into_inner() {
        let file_hash = hash(&content);
        files.insert(filename.clone(), FileData { content, hash: file_hash });
    }

    // Recalculate Merkle root
    let sentence: String = files.values().map(|data| data.content.clone()).collect::<Vec<_>>().join(" ");
    let root = merkle_tree::calculate_merkle_root(&sentence);

    let mut merkle_root = state.merkle_root.lock().unwrap();
    *merkle_root = Some(root);

    HttpResponse::Ok().body("Files uploaded and Merkle root calculated.")
}

#[get("/download/{filename}")]
async fn download(file_name: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let files = state.files.lock().unwrap();
    if let Some(file_data) = files.get(file_name.as_str()) {
        HttpResponse::Ok().json(&file_data.content)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[derive(Serialize)]
struct ProofResponse {
    root: HashValue,
    proof: MerkleProof,
}

#[get("/proof/{filename}")]
async fn proof(file_name: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let files = state.files.lock().unwrap();
    let merkle_root = state.merkle_root.lock().unwrap();
    
    if let Some(file_data) = files.get(file_name.as_str()) {
        if let Some(root) = *merkle_root {
            let sentence: String = files.values().map(|data| data.content.clone()).collect::<Vec<_>>().join(" ");
            let index = files.keys().position(|k| k == file_name.as_str()).unwrap();
            let proof = generate_proof(&sentence, index);
            
            let proof_response = ProofResponse {
                root: proof.0,
                proof: proof.1,
            };

            return HttpResponse::Ok().json(proof_response);
        }
    }
    HttpResponse::NotFound().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        files: Arc::new(Mutex::new(HashMap::new())),
        merkle_root: Arc::new(Mutex::new(None)),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(upload)
            .service(download)
            .service(proof)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
