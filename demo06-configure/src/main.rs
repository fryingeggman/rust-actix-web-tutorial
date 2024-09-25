use actix_web::{web, App, HttpResponse, HttpServer};

fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test").route(web::get().to(|| async { HttpResponse::Ok().body("test") })),
    );
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app").route(web::get().to(|| async { HttpResponse::Ok().body("app") })),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("/") }),
            )
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
