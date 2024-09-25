use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

struct AppStateWithCounter {
    // mutex is necessary to mutate safely across threads
    counter: Mutex<i32>,
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    // get counter's MutexGuard
    let mut counter = data.counter.lock().unwrap();

    // access counter inside MutexGuard
    *counter += 1;

    format!("Request number: {counter}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .route("/", web::get().to(index))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
