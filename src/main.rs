use actix_web::{self, App, HttpServer};
use load_dotenv::load_dotenv;
load_dotenv!();

#[actix_web::get("/")]
async fn helloworld() -> &'static str {
    "Hello"
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(helloworld))
        .bind((
            "0.0.0.0",
            env!("ACTIX_PORT").parse::<u16>().expect("ACTIX_PORT"),
        ))?
        .run()
        .await
}
