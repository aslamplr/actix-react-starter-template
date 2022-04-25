use actix_web::{error, get, post, web, HttpResponse, Responder};

use crate::{
    context::AppContext,
    services::cake::{CakeService, CreateCakeModel},
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_cakes)
        .service(get_cake)
        .service(create_cake);
}

#[get("/cake")]
async fn get_all_cakes(ctx: web::Data<AppContext>) -> impl Responder {
    let cake_service = ctx.cake_service();
    let result = cake_service
        .get_all_cakes()
        .await
        .map_err(error::ErrorInternalServerError)?;
    Ok::<_, error::Error>(HttpResponse::Ok().json(result))
}

#[get("/cake/{id}")]
async fn get_cake(id: web::Path<i32>, ctx: web::Data<AppContext>) -> impl Responder {
    let cake_service = ctx.cake_service();
    let result = cake_service
        .get_cake_by_id(*id)
        .await
        .map_err(error::ErrorInternalServerError)?;
    Ok::<_, error::Error>(HttpResponse::Ok().json(result))
}

#[post("/cake")]
async fn create_cake(
    cake_input: web::Json<CreateCakeModel>,
    ctx: web::Data<AppContext>,
) -> impl Responder {
    let cake_service = ctx.cake_service();

    let result = cake_service
        .create_cake(cake_input.0)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok::<_, error::Error>(HttpResponse::Ok().json(result))
}
