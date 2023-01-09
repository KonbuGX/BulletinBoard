#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web,App,HttpServer};
use dotenv::dotenv;
use std::env;
use diesel::r2d2::{Pool,ConnectionManager};
use diesel::sqlite::{SqliteConnection};
use once_cell::sync::OnceCell;

mod models;
mod service;
mod schema;
mod error_msg;
mod error;
mod session_management;
mod screen_status;
mod routes;
mod controller;
use error::MyError;
use routes::routes;

pub static ACCTNO: OnceCell<String> = OnceCell::new();
pub static ACCTNAME: OnceCell<String> = OnceCell::new();
pub static SESSION_ID: OnceCell<String> = OnceCell::new();
pub const REGEX_ALPHANUMERIC: &str = "^[[:alnum:]]+$";


#[actix_rt::main]
async fn main() -> Result<(),actix_web::Error> {
    dotenv().ok();

    //定数の初期化
    ACCTNO.set(String::from("acct_no")).unwrap();
    ACCTNAME.set(String::from("acct_name")).unwrap();

    //データベース、Redisのプール作成を行う
    let bind_address = env::var("ADDRESS").expect("ADDRESS is not set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let redis_addr = env::var("REDIS_ADDR").expect("REDIS_ADDR is not set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = Pool::new(manager).expect("Failed pool");
    let client = r2d2_redis::RedisConnectionManager::new(redis_addr).unwrap();
    let redis_pool = r2d2_redis::r2d2::Pool::builder().max_size(10).build(client).unwrap();

    HttpServer::new(move ||
    {App::new()
        .configure(routes)
        .app_data(web::Data::new(pool.clone()))
        .app_data(web::Data::new(redis_pool.clone()))
    })
    .bind(&bind_address)?
    .run()
    .await?;
    Ok(())
}