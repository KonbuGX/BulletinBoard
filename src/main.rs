#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{get,post,web,App,HttpResponse,HttpServer,http::header};
use askama::{Template};
use dotenv::dotenv;
use std::env;
use diesel::r2d2::{Pool,ConnectionManager};
use diesel::sqlite::{SqliteConnection};
use actix_files::Files;

mod models;
mod service;
mod schema;
mod error;
use error::MyError;
use models::Thread;
use models::AddTreadParams;
use models::DeleteTreadParams;
use models::ThreadComment;
use models::GetThreadParams;
use models::AddCommentParams;
use service::select_all_thred;
use service::insert_thread;
use service::remove_thread;
use service::validation_thread;
use service::select_comment;
use service::insert_comment;
use service::remove_comment;
use service::validation_comment;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate{
    thread_list: Vec<Thread>,
    error_msg: Vec<String>,
}

#[derive(Template)]
#[template(path = "thread_comment.html")]
struct CommentTemplate{
    tid: i32,
    tname: String,
    comment_list: Vec<ThreadComment>,
    error_msg: Vec<String>,
}

//初期表示
#[get("/")]
async fn index(db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;
    let thread_list = select_all_thred(&conn);
    let error_msg = Vec::new();
    let html = IndexTemplate {thread_list,error_msg};
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
    .content_type("text/html")
    .body(response_body))
}

//登録ボタン押下時
#[post("/addThread")]
async fn add_thread(params: web::Form<AddTreadParams>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;

    //エラーがあればインサート処理をせずエラーメッセージを表示
    let error_msg = validation_thread(&params);
    if error_msg.len() > 0{
        let thread_list = select_all_thred(&conn);
        let html = IndexTemplate {thread_list,error_msg};
        let response_body = html.render()?;
        Ok(HttpResponse::Ok().content_type("text/html").body(response_body))
    }else {
        insert_thread(params.text.clone(),conn);
        Ok(HttpResponse::SeeOther().header(header::LOCATION,"/").finish())
    }
    
}

//削除ボタン押下時
#[post("/deleteThread")]
async fn delete_thread(params: web::Form<DeleteTreadParams>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;

    //スレッドとそれに付随するコメントを削除
    remove_thread(params.id.clone(),params.text.clone(),&conn);
    remove_comment(params.id.clone(),&conn);
    Ok(HttpResponse::SeeOther().header(header::LOCATION,"/").finish())
}

//タスクのコメント画面表示時
#[post("/threadComment")]
pub async fn thread_comment(params: web::Form<GetThreadParams>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;
    let tid = params.tid.clone();
    let tname = params.tname.clone();
    let comment_list = select_comment(&conn,tid);
    let error_msg = Vec::new();
    let html = CommentTemplate {tid,tname,comment_list,error_msg};
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
    .content_type("text/html")
    .body(response_body))
}

//コメントの登録ボタン押下時
#[post("/addComment")]
async fn add_thread_comment(params: web::Form<AddCommentParams>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;
    let tid = params.tid.clone();
    let cname = params.cname.clone();
    let cmt = params.cmt.clone();
    let tname = params.tname.clone();

    //エラーがない場合にインサート処理
    let error_msg = validation_comment(&params);
    if error_msg.len() <= 0 {
        insert_comment(tid,cname,cmt,&conn);
    }
    
    //タスクのコメント画面の表示処理
    let comment_list = select_comment(&conn,tid);
    let html = CommentTemplate {tid,tname,comment_list,error_msg};
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
    .content_type("text/html")
    .body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(),actix_web::Error> {
    dotenv().ok();
    let bind_address = env::var("ADDRESS").expect("ADDRESS is not set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = Pool::new(manager).expect("Failed pool");
    
    HttpServer::new(move || App::new().service(index).service(add_thread).service(delete_thread)
    .service(thread_comment).service(add_thread_comment)
    .service(Files::new("/public", "./public").show_files_listing()).data(pool.clone()))
    .bind(&bind_address)?
    .run()
    .await?;
    Ok(())
}