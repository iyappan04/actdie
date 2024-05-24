use serde::{Deserialize, Serialize};
use crate::schema::products;
use diesel::prelude::*;

// use this struct to represent json to save a product

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "products"]
pub struct CreateProduct {
    pub name: String,
    pub description: String,
    pub amount: i32
}

// use this struct to mapping value
#[derive(Serialize)]
pub struct ProductResponse {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub amount: i32
}

// use this struct to respresent data on response
#[derive(Serialize)]
pub struct ProductsResponse {
    pub status: String,
    pub data: Vec<ProductResponse>
}
