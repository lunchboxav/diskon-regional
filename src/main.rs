use actix_web::{get, web, App, HttpServer, HttpResponse, Responder, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Ump {
  propinsi: String,
  ump: i32
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/ump/{propinsi}")]
/// TODO: lookup ke umr.json, sehingga angkanya nggak hardcode.
async fn index(web::Path(propinsi): web::Path<String>) -> Result<HttpResponse> {
  Ok(HttpResponse::Ok().json(Ump {
    propinsi: propinsi.to_string(),
    ump: 1000000
  }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
        .service(hello)
        .service(index)
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
