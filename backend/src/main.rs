mod entities;
use actix_web::{self, App, HttpResponse, HttpServer, http::StatusCode, web};
use entities::{
    lidluren::{self, Model},
    prelude::*,
};
use load_dotenv::load_dotenv;

use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};
use serde::Deserialize;
load_dotenv!();

type UtcDateTime = chrono::DateTime<chrono::Utc>;
type DataDatabaseConnection = web::Data<DatabaseConnection>;
#[actix_web::get("/shifts")]
async fn get_shifts(db: DataDatabaseConnection) -> web::Json<Vec<Model>> {
    web::Json(Lidluren::find().all(db.as_ref()).await.expect("get_shifts"))
}

#[derive(Deserialize)]
struct ShiftPayload {
    begintime: UtcDateTime,
    endtime: UtcDateTime,
    dayforceplanned: bool,
}
#[actix_web::put("/shifts")]
async fn create_shifts(
    db: DataDatabaseConnection,
    shiftpayload: web::Json<ShiftPayload>,
) -> HttpResponse {
    let newshift = lidluren::ActiveModel {
        dayforceplanned: ActiveValue::set(Some(shiftpayload.dayforceplanned as i8)),
        begintime: ActiveValue::set(shiftpayload.begintime.naive_utc()),
        endtime: ActiveValue::set(shiftpayload.endtime.naive_utc()),
        shiftid: ActiveValue::NotSet,
    };
    Lidluren::insert(newshift).exec(db.as_ref()).await.unwrap();

    HttpResponse::new(StatusCode::CREATED)
}
#[derive(Deserialize)]
struct ShiftDeletionQueryParams {
    shiftid: i32,
}
#[actix_web::delete("/shifts")]
async fn delete_shifts(
    db: DataDatabaseConnection,
    params: web::Query<ShiftDeletionQueryParams>,
) -> HttpResponse {
    let delresult = lidluren::ActiveModel {
        shiftid: ActiveValue::set(params.shiftid),
        dayforceplanned: ActiveValue::NotSet,
        begintime: ActiveValue::NotSet,
        endtime: ActiveValue::NotSet,
    }
    .delete(db.as_ref())
    .await
    .unwrap();
    if delresult.rows_affected == 0 {
        return HttpResponse::new(StatusCode::NOT_FOUND);
    }

    HttpResponse::new(StatusCode::CREATED)
}
#[derive(Deserialize)]
struct UpdatedShift {
    begintime: UtcDateTime,
    endtime: UtcDateTime,
    dayforceplanned: bool,
    shiftid: i32,
}
#[actix_web::post("/shifts")]
async fn update_shifts(
    newshift: web::Json<UpdatedShift>,
    db: DataDatabaseConnection,
) -> HttpResponse {
    lidluren::ActiveModel {
        shiftid: ActiveValue::set(newshift.shiftid),
        begintime: ActiveValue::set(newshift.begintime.naive_utc()),
        endtime: ActiveValue::set(newshift.endtime.naive_utc()),
        dayforceplanned: ActiveValue::set(Some(newshift.dayforceplanned as i8)),
    }
    .update(db.as_ref())
    .await
    .unwrap();

    HttpResponse::new(StatusCode::ACCEPTED)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let db: DatabaseConnection = sea_orm::Database::connect(env!("DB_STRING"))
        .await
        .expect("Database string");

    assert!(db.ping().await.is_ok());
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(get_shifts)
            .service(create_shifts)
            .service(update_shifts)
            .service(delete_shifts)
            .app_data(web::Data::new(db.clone()))
    })
    .bind((
        "0.0.0.0",
        env!("ACTIX_PORT").parse::<u16>().expect("ACTIX_PORT"),
    ))?
    .run()
    .await
}
