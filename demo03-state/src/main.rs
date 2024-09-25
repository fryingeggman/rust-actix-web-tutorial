use actix_web::{get, web, App, HttpServer, Responder};

struct AppState {
    app_name: String,
    count: i32,
}

#[get("/hello")]
async fn hello(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}")
}

async fn name(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("My name is {app_name}")
}

async fn count(data: web::Data<AppState>) -> impl Responder {
    let count = data.count;
    format!("count: {count}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        app_name: String::from("Actix Web"),
        count: 0,
    });

    HttpServer::new(move || {
        App::new()
            // state能在同一scope中共享路由和资源
            .app_data(app_state.clone())
            .service(hello)
            .app_data(web::Data::new(AppState {
                app_name: String::from("App State"),
                count: 520,
            }))
            .service(
                web::scope("/app")
                    .route("/name", web::get().to(name))
                    .route("/count", web::get().to(count)),
            )
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
