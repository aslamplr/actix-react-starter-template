mod context;
mod routes;
mod services;

use std::env;

use actix_cors::Cors;
use actix_web::{
    error, get, http::header, middleware, web, App, HttpResponse, HttpServer, Responder,
};
use env_logger::Env;

use entity::sea_orm;
use migration::{Migrator, MigratorTrait};

use context::AppContext;
use routes::cake::config as cake_config;

#[derive(Debug, serde::Serialize)]
pub struct Message<'a, T: ?Sized> {
    msg: &'a T,
}

impl<'a, T: ?Sized> Message<'a, T> {
    fn new(msg: &'a T) -> Self {
        Self { msg }
    }
}

impl<'a, T: AsRef<str> + ?Sized + 'a> From<&'a T> for Message<'a, str> {
    fn from(msg: &'a T) -> Self {
        let msg = msg.as_ref();
        Self::new(msg)
    }
}

#[get("/health")]
async fn hello(_: web::Data<AppContext>) -> impl Responder {
    let msg = "success!";
    web::Json(Message::from(msg))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    dotenv::dotenv().ok();
    let bind_addr = env::var("BIND_ADDR").expect("BIND_ADDRESS is not set in .env file");
    let bind_port = env::var("BIND_PORT").expect("BIND_PORT is not set in .env file");
    let bind_address: std::net::SocketAddr = format!("{}:{}", bind_addr, bind_port).parse()?;
    let database_migration_run = env::var("DATABASE_MIGRATION_RUN")
        .map(|value| value == "true")
        .unwrap_or(false);
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let cors_allowed_origins = env::var("CORS_ALLOWED_ORIGIN")
        .map(|value| {
            value
                .split(',')
                .map(|val| val.trim().to_string())
                .collect::<Vec<_>>()
        })
        .expect("CORS_ALLOWED_ORIGIN is not in .env file");

    let conn = sea_orm::Database::connect(&db_url).await.unwrap();
    if database_migration_run {
        Migrator::up(&conn, None).await.unwrap();
    }

    let state = AppContext::new(conn);

    HttpServer::new(move || {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });
        let cors_mw = {
            let mut cors = Cors::default()
                .allow_any_method()
                .allowed_headers(vec![
                    header::AUTHORIZATION,
                    header::ACCEPT,
                    header::CONTENT_TYPE,
                ])
                .supports_credentials()
                .max_age(3600);

            for origin in &cors_allowed_origins {
                cors = cors.allowed_origin(&origin);
            }
            cors
        };

        App::new()
            .app_data(json_config)
            .app_data(web::Data::new(state.clone()))
            .wrap(cors_mw)
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(hello)
            .configure(cake_config)
    })
    .bind(bind_address)?
    .run()
    .await?;

    Ok(())
}
