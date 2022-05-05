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

#[get("/status")]
async fn hello(_: web::Data<AppContext>) -> impl Responder {
    let msg = "success!";
    web::Json(Message::from(msg))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    dotenv::dotenv().ok();
    let bind_address: std::net::SocketAddr = env::var("BIND_ADDRESS")
        .expect("BIND_ADDRESS is not set in .env file")
        .parse()?;
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let cors_allowed_origin: String =
        env::var("CORS_ALLOWED_ORIGIN").expect("CORS_ALLOWED_ORIGIN is not in .env file");

    let conn = sea_orm::Database::connect(&db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let state = AppContext::new(conn);

    HttpServer::new(move || {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        App::new()
            .app_data(json_config)
            .app_data(web::Data::new(state.clone()))
            .wrap(
                Cors::default()
                    .allowed_origin(&cors_allowed_origin)
                    .allow_any_method()
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::CONTENT_TYPE,
                    ])
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(web::scope("/api").service(hello).configure(cake_config))
    })
    .bind(bind_address)?
    .run()
    .await?;

    Ok(())
}
