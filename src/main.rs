use ctix_web::{self, App, HttpResponse, HttpServer, http::StatusCode};
use load_dotenv::load_dotenv;
use sea_orm::{DatabaseConnection, DbErr};
load_dotenv!();

#[actix_web::get("/shifts")]
async fn get_shifts() -> HttpResponse {
    HttpResponse::new(StatusCode::OK)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: DatabaseConnection = sea_orm::Database::connect(env!("DB_STRING"))
        .await
        .expect("Database string");

    assert!(db.ping().await.is_ok());

    HttpServer::new(|| App::new().service(get_shifts))
        .bind((
            "0.0.0.0",
            env!("ACTIX_PORT").parse::<u16>().expect("ACTIX_PORT"),
        ))?
        .run()
        .await
}
