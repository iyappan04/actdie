use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, Result};

use std::env;
use diesel::prelude::*;

use crate::models::productmodel::*;

use diesel::{ Connection, prelude::* };

use diesel::pg::PgConnection;


use dotenv::dotenv;


// use actix_web::{ Data };


fn db_connect() -> PgConnection {


    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("Database Must Be Set");

    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", &db_url))

}


#[post("/product/create")]
pub async fn save_product(product: web::Json<CreateProduct>) -> Result<HttpResponse> {

    use crate::schema::products::dsl::*;

    let mut connection = db_connect();

    diesel::insert_into(products)
        .values(&product.into_inner())
        .execute(& mut connection)
        .expect("Error inserting new product");

    Ok(HttpResponse::Ok().json("Product Create Success."))

}

