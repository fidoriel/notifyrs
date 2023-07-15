use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use rand::Rng;

async fn success_or_not(prob: f64) -> bool {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    x < prob
}

#[post("/random_echo")]
async fn random_echo(req_body: String) -> impl Responder {
    let prob = 0.99;
    let success = success_or_not(prob).await;
    println!("Success: {} Request body: {}", success, req_body);
    if success {
        HttpResponse::Ok().body(req_body)
    } else {
        HttpResponse::BadRequest().body("Bad request")
    }
}

#[post("/post")]
async fn simple_post(req_body: String) -> impl Responder {
    println!("Request body: {}", req_body);
    HttpResponse::Ok().body(req_body)
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(random_echo)
            .service(health)
            .service(simple_post)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
