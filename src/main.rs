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
use pwhash::bcrypt;

mod models;
mod service;
mod schema;
mod error;
mod session_management;
mod screen_status;
use error::MyError;
use models::Thread;
use models::AddTreadSearchParams;
use models::AddTreadParams;
use models::DeleteTreadParams;
use models::ThreadComment;
use models::GetThreadParams;
use models::AddCommentParams;
use models::AddAccountParams;
use service::select_all_thred;
use service::select_thred_name;
use service::insert_thread;
use service::remove_thread;
use service::validation_thread;
use service::return_login_status;
use service::select_comment;
use service::insert_comment;
use service::remove_comment;
use service::validation_comment;
use service::select_all_account;
use service::select_account_byname;
use service::insert_account;
use service::remove_account;
use service::validation_account;
use session_management::set_session;
use session_management::get_session;
use session_management::delete_session;

const ACCTNO: &str = "acct_no";

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate{
    login_status: String,
    thread_list: Vec<Thread>,
    error_msg: Vec<String>,
}

#[derive(Template)]
#[template(path = "thread_comment.html")]
struct CommentTemplate{
    thd_id: i32,
    thd_name: String,
    comment_list: Vec<ThreadComment>,
    error_msg: Vec<String>,
    login_status: String,
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate{
    acct_name: String,
    pwd: String,
    error_msg: Vec<String>,
}

#[derive(Template)]
#[template(path = "signup.html")]
struct SignUpTemplate{
    acct_name: String,
    pwd: String,
    error_msg: Vec<String>,
}

//初期表示
#[get("/")]
async fn index(redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;
    let thread_list = select_all_thred(&conn);
    let error_msg = Vec::new();
    let mut redis_conn = redis.get()?;
    let temp_status = get_session(&mut redis_conn, ACCTNO);
    let login_status = return_login_status(temp_status);
    
    let html = IndexTemplate {login_status,thread_list,error_msg};
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
    .content_type("text/html")
    .body(response_body))
}

//ログイン画面表示
#[get("/login")]
async fn login() -> Result<HttpResponse,MyError>{
    let acct_name = String::from("");
    let pwd = String::from("");
    let error_msg = Vec::new();
    let html = LoginTemplate {acct_name,pwd,error_msg};
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
    .content_type("text/html")
    .body(response_body))
}

//ログインボタン押下時
#[post("/login")]
async fn login_account(redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,params: web::Form<AddAccountParams>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;
    let status = screen_status::ScreenStatus::LOGIN.to_string();
    let error_msg = validation_account(&params,status,&conn);
    if error_msg.len() > 0{
        //エラーメッセージをログイン画面に表示
        let acct_name = String::from("");
        let pwd = String::from("");
        let html = LoginTemplate {acct_name,pwd,error_msg};
        let response_body = html.render()?;
        Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
    }else{
        //アカウントナンバーをセッションに追加。
        let acct_name = params.acct_name.clone();
        let temp_list = select_account_byname(acct_name, &conn);
        let acct_no = temp_list[0].account_no;
        let mut redis_conn = redis.get()?;
        set_session(&mut redis_conn, ACCTNO, &acct_no);

        //ホーム画面の表示
        Ok(HttpResponse::SeeOther().append_header((header::LOCATION,"/")).finish())
    }
}

//サインアウトボタン押下時
#[post("/signout")]
async fn signout(redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>) -> Result<HttpResponse,MyError>{
    let mut redis_conn = redis.get()?;
    delete_session(&mut redis_conn,ACCTNO);
    Ok(HttpResponse::SeeOther().append_header((header::LOCATION,"/")).finish())
}

//新規登録画面表示
#[get("/signup")]
async fn signup() -> Result<HttpResponse,MyError>{
    let acct_name = String::from("");
    let pwd = String::from("");
    let error_msg = Vec::new();
    let html = SignUpTemplate {acct_name,pwd,error_msg};
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
    .content_type("text/html")
    .body(response_body))
}

//新規登録ボタン押下時
#[post("/signup")]
async fn signup_account(redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,params: web::Form<AddAccountParams>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;
    let status = screen_status::ScreenStatus::SIGNUP.to_string();
    let error_msg = validation_account(&params,status,&conn);
    if error_msg.len() > 0{
        //エラーメッセージを新規登録画面に表示
        let acct_name = String::from("");
        let pwd = String::from("");
        let html = SignUpTemplate {acct_name,pwd,error_msg};
        let response_body = html.render()?;
        Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
    }else{
        //accountNoの付番処理
        let account_list = select_all_account(&conn);
        let mut acct_no:i32 = 0;
        if account_list.len() > 0 {
            for temp_account in &account_list {
                if temp_account.account_no > acct_no {
                    acct_no = temp_account.account_no;
                }  
            }
            acct_no += 1;
        }else{
            acct_no = 1;
        }

        //パスワードはハッシュ化してインサート。
        let acct_name = params.acct_name.clone();
        let pwd = bcrypt::hash(params.pwd.clone()).unwrap();
        insert_account(acct_no,acct_name,pwd,&conn);

        let mut redis_conn = redis.get()?;
        set_session(&mut redis_conn, ACCTNO, &acct_no);

        //ホーム画面の表示
        Ok(HttpResponse::SeeOther().append_header((header::LOCATION,"/")).finish())
    }
}

//アカウント削除のダイアログのボタン押下時
#[post("/deleteAccount")]
async fn delete_account(redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let mut redis_conn = redis.get()?;
    let acct_no = get_session(&mut redis_conn, ACCTNO);
    let conn = db.get()?;
    remove_account(acct_no, conn);
    delete_session(&mut redis_conn,ACCTNO);
    Ok(HttpResponse::SeeOther().append_header((header::LOCATION,"/")).finish())
}

//登録ボタン押下時
#[post("/addThread")]
async fn add_thread(redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,params: web::Form<AddTreadParams>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;
    let mut redis_conn = redis.get()?;

    //エラーがあればインサート処理をせずエラーメッセージを表示
    let error_msg = validation_thread(&params);
    if error_msg.len() > 0{
        let thread_list = select_all_thred(&conn);
        let temp_status = get_session(&mut redis_conn, ACCTNO);
        let login_status = return_login_status(temp_status);
        let html = IndexTemplate {login_status,thread_list,error_msg};
        let response_body = html.render()?;
        Ok(HttpResponse::Ok().content_type("text/html").body(response_body))
    }else{
        insert_thread(params.thd_name.clone(),&conn);

        //ホーム画面の表示
        Ok(HttpResponse::SeeOther().append_header((header::LOCATION,"/")).finish())
    }
    
}

//削除ボタン押下時
#[post("/deleteThread")]
async fn delete_thread(params: web::Form<DeleteTreadParams>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;

    //スレッドとそれに付随するコメントを削除
    remove_thread(params.thd_id.clone(),params.thd_name.clone(),&conn);
    remove_comment(params.thd_id.clone(),&conn);

    //ホーム画面の表示
    Ok(HttpResponse::SeeOther().append_header((header::LOCATION,"/")).finish())
}

//検索ボタン押下時
#[post("/searchThread")]
async fn search_thread(redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,params: web::Form<AddTreadSearchParams>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;
    let thd_name = params.thd_name.clone();
    let thread_list = select_thred_name(thd_name,&conn);
    let error_msg = Vec::new();

    let mut redis_conn = redis.get()?;
    let temp_status = get_session(&mut redis_conn, ACCTNO);
    let login_status = return_login_status(temp_status);
    let html = IndexTemplate {login_status,thread_list,error_msg};
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
    .content_type("text/html")
    .body(response_body))
}

//タスクのコメント画面表示時
#[post("/threadComment")]
pub async fn thread_comment(params: web::Form<GetThreadParams>,redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;
    let thd_id = params.thd_id.clone();
    let thd_name = params.thd_name.clone();
    let comment_list = select_comment(&conn,thd_id);
    let error_msg = Vec::new();

    //ログインの状態を取得
    let mut redis_conn = redis.get()?;
    let temp_status = get_session(&mut redis_conn, ACCTNO);
    let login_status = return_login_status(temp_status);

    let html = CommentTemplate {thd_id,thd_name,comment_list,error_msg,login_status};
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
    .content_type("text/html")
    .body(response_body))
}

//コメントの登録ボタン押下時
#[post("/addComment")]
async fn add_thread_comment(params: web::Form<AddCommentParams>,redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;
    let thd_id = params.thd_id.clone();
    let cname = params.cmt_name.clone();
    let cmt = params.cmt.clone();
    let thd_name = params.thd_name.clone();

    //エラーがない場合にインサート処理
    let error_msg = validation_comment(&params);
    if error_msg.len() <= 0 {
        insert_comment(thd_id,cname,cmt,&conn);
    }
    
    //ログインの状態を取得
    let mut redis_conn = redis.get()?;
    let temp_status = get_session(&mut redis_conn, ACCTNO);
    let login_status = return_login_status(temp_status);

    //タスクのコメント画面の表示処理
    let comment_list = select_comment(&conn,thd_id);
    let html = CommentTemplate {thd_id,thd_name,comment_list,error_msg,login_status};
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
    .content_type("text/html")
    .body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(),actix_web::Error> {
    dotenv().ok();
    //データベース、Redisのプール作成を行う
    let bind_address = env::var("ADDRESS").expect("ADDRESS is not set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let redis_addr = env::var("REDIS_ADDR").expect("REDIS_ADDR is not set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = Pool::new(manager).expect("Failed pool");
    let client = r2d2_redis::RedisConnectionManager::new(redis_addr).unwrap();
    let redis_pool = r2d2_redis::r2d2::Pool::builder().max_size(10).build(client).unwrap();

    HttpServer::new(move || {App::new().service(index).service(login).service(login_account).service(signout)
    .service(signup).service(signup_account).service(delete_account).service(add_thread).service(search_thread)
    .service(delete_thread).service(thread_comment).service(add_thread_comment)
    .service(Files::new("/public", "./public").show_files_listing()).app_data(web::Data::new(pool.clone())).app_data(web::Data::new(redis_pool.clone()))})
    .bind(&bind_address)?
    .run()
    .await?;
    Ok(())
}