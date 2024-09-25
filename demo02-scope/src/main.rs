use actix_web::{web, App, HttpServer, Responder};

async fn hello() -> impl Responder {
    "Hello World!"
}

async fn index() -> impl Responder {
    "Index"
}

async fn count() -> impl Responder {
    "520"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                // scope相当于所有路由的命名空间
                // 同一个scope下所有的路由都有相同url前缀
                web::scope("/app")
                    .route("/hello", web::get().to(hello))
                    .route("/index", web::get().to(index)),
            )
            .service(web::scope("/user").route("/count", web::get().to(count)))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
