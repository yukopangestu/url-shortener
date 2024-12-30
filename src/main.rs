use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Url {
    original: String,
}

#[derive(Serialize, Deserialize)]
pub struct ShortId {
    pub short_id: String,
}

pub struct AppState {
    pub url_map: Mutex<HashMap<String, String>>
}

async fn shorten_url(data: web::Data<AppState>, url: web::Json<Url>) -> impl Responder {
    let mut url_map = data.url_map.lock().unwrap();
    let short_id = Uuid::new_v4().to_string();
    url_map.insert(short_id.clone(), url.original.clone());
    HttpResponse::Ok().json(short_id)
}

async fn redirect_to_original(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let url_map = data.url_map.lock().unwrap();
    let short_id = path.into_inner();
    if let Some(original_url) = url_map.get(&short_id) {
        HttpResponse::Found()
            .append_header(("Location", original_url.clone()))
            .finish()
    } else {
        HttpResponse::NotFound().body("URL not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState{
        url_map: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/shorten", web::post().to(shorten_url))
            .route("/{short_id}", web::get().to(redirect_to_original))
    }).bind("127.0.0.1:8080")?.run().await
}

// fn main() {
//     println!("Hello, world!");
// }