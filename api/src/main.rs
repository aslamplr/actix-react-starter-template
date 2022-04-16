use actix_web::{get, middleware, web, App, HttpServer, Responder};
use env_logger::Env;

#[derive(Debug, serde::Serialize)]
struct Message<'a> {
    text: &'a str,
}

impl<'a> Message<'a> {
    fn new(text: &'a str) -> Self {
        Self { text }
    }
}

#[get("/status")]
async fn hello() -> impl Responder {
    web::Json(Message::new("success!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(web::scope("/api").service(hello))
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
