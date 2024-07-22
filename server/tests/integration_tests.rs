use actix_web::{test, App};
use server::{upload, download, proof, create_app_state, configure_services};
use std::collections::HashMap;
use actix_web::web::Data;
use std::sync::{Arc, Mutex};
use merkle_tree::{hash};

#[actix_web::test]
async fn test_upload_and_proof() {
    let state = create_app_state();

    let mut app = test::init_service(App::new()
        .app_data(state.clone())
        .configure(configure_services)
    ).await;

    let payload = serde_json::json!({
        "file1.txt": "This is the content of file1.",
        "file2.txt": "File2 contains different content.",
    });

    let req = test::TestRequest::post()
        .uri("/upload")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());

    let req = test::TestRequest::get()
        .uri("/proof/file1.txt")
        .to_request();

    let resp: serde_json::Value = test::call_and_read_body_json(&mut app, req).await;
    assert!(resp.get("root").is_some());
    assert!(resp.get("proof").is_some());
}
