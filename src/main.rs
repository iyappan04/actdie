mod api;
mod models;
mod schema;

use actix_web:: {
    HttpServer, App, web, HttpResponse, Responder
};

use api::productapi::*;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .app_data(tera)
            .service(save_product)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
    
}