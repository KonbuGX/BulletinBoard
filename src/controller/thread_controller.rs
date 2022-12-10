use actix_web::{get,post,web,HttpResponse,http::header};
use askama::{Template};
use diesel::r2d2::{ConnectionManager};
use diesel::sqlite::{SqliteConnection};

use crate::MyError;
use crate::models::{Thread,ThreadSearchParams,ThreadCreateParams,ThreadComment,CommentCreateParams,ThreadGetParams};
use crate::service::get_login_status;
use crate::service::get_acct_name;
use crate::service::remove_account;
use crate::session_management::get_session;
use crate::session_management::delete_session;

use crate::ACCTNO;
use crate::ACCTNAME;
use crate::SESSION_ID;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate{
    acct_name: String,
    login_status: String,
    search_keyword: String,
    thread_list: Vec<Thread>,
    error_msg: Vec<String>,
}

#[derive(Template)]
#[template(path = "thread_comment.html")]
struct CommentTemplate{
    thd_id: i32,
    thd_name: String,
    acct_name: String,
    cmt_name: String,
    comment_list: Vec<ThreadComment>,
    error_msg: Vec<String>,
    login_status: String,
}

//初期表示
#[get("/")]
pub async fn index(redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;
    let thread_list = Thread::select_all_thred(&conn);
    let error_msg = Vec::new();

    //ログインの状態を取得
    let mut redis_conn = redis.get()?;
    let acct_info = get_session(&mut redis_conn, &SESSION_ID);
    let login_status = get_login_status(&acct_info);

    //アカウント名の取得
    let acct_name = get_acct_name(acct_info.get(ACCTNAME.get().unwrap()));

    //検索エリアのキーワードの初期化
    let search_keyword = String::from("");

    let html = IndexTemplate {acct_name,login_status,search_keyword,thread_list,error_msg};
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
    .content_type("text/html")
    .body(response_body))
}

//サインアウトボタン押下時
#[post("/signout")]
pub async fn signout(redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>) -> Result<HttpResponse,MyError>{
    let mut redis_conn = redis.get()?;
    let account_info = vec![ACCTNO.get().unwrap(),ACCTNAME.get().unwrap()];
    delete_session(&mut redis_conn,&SESSION_ID, account_info);
    //「""」をセットしてセッションIDがない状態を表す
    let _ = &SESSION_ID.set(String::from(""));

    Ok(HttpResponse::SeeOther().append_header((header::LOCATION,"/")).finish())
}

//アカウント削除のダイアログのボタン押下時
#[post("/deleteAccount")]
pub async fn delete_account(redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let mut redis_conn = redis.get()?;
    let acct_info = get_session(&mut redis_conn, &SESSION_ID);
    let mut acct_no = 0;
    if let Some(v) = acct_info.get(ACCTNO.get().unwrap()) {
        acct_no = v.parse::<i32>().unwrap();
    }

    let conn = db.get()?;
    let account_info_keys = vec![ACCTNO.get().unwrap(),ACCTNAME.get().unwrap()];
    remove_account(acct_no, conn);
    delete_session(&mut redis_conn,&SESSION_ID, account_info_keys);

    //「""」をセットしてセッションIDがない状態を表す
    let _ = &SESSION_ID.set(String::from(""));
    Ok(HttpResponse::SeeOther().append_header((header::LOCATION,"/")).finish())
}

//登録ボタン押下時
#[post("/addThread")]
pub async fn add_thread(params: web::Form<ThreadCreateParams>,redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;

    //エラーがあればインサート処理をせずエラーメッセージを表示
    let thd_create_params = ThreadCreateParams::new(params.thd_name.clone());
    let error_msg = thd_create_params.validation_thread();
    if error_msg.len() > 0{
        let thread_list = Thread::select_all_thred(&conn);
        let mut redis_conn = redis.get()?;
        let acct_info = get_session(&mut redis_conn, &SESSION_ID);
        let login_status = get_login_status(&acct_info);

        //アカウント名の取得
        let acct_name = get_acct_name(acct_info.get(ACCTNAME.get().unwrap()));

        //検索エリアのキーワードの初期化
        let search_keyword = String::from("");

        let html = IndexTemplate {acct_name,login_status,search_keyword,thread_list,error_msg};
        let response_body = html.render()?;
        Ok(HttpResponse::Ok().content_type("text/html").body(response_body))
    }else{
        //スレッドのインサート処理
        thd_create_params.insert_thread(&conn);

        //ホーム画面の表示
        Ok(HttpResponse::SeeOther().append_header((header::LOCATION,"/")).finish())
    }
    
}

//検索ボタン押下時
#[post("/searchThread")]
pub async fn search_thread(params: web::Form<ThreadSearchParams>,redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;
    let search_keyword = params.search_keyword.clone();

    let thread_search_params = ThreadSearchParams::new(params.search_keyword.clone());
    let thread_list = thread_search_params.select_thred_name(&conn);
    let error_msg = Vec::new();

    let mut redis_conn = redis.get()?;
    let acct_info = get_session(&mut redis_conn, &SESSION_ID);
    let login_status = get_login_status(&acct_info);

    //アカウント名の取得
    let acct_name = get_acct_name(acct_info.get(ACCTNAME.get().unwrap()));

    let html = IndexTemplate {acct_name,login_status,search_keyword,thread_list,error_msg};
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
    .content_type("text/html")
    .body(response_body))
}

//タスクのコメント画面表示時
#[post("/threadComment")]
pub async fn thread_comment(params: web::Form<ThreadGetParams>,redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;
    let thd_id = params.thd_id.clone();
    let thd_name = params.thd_name.clone();
    let comment_list = ThreadComment::select_comment(&conn,thd_id);
    let error_msg = Vec::new();

    //ログインの状態を取得
    let mut redis_conn = redis.get()?;
    let acct_info = get_session(&mut redis_conn, &SESSION_ID);
    let login_status = get_login_status(&acct_info);

    //アカウント名の取得
    let acct_name = get_acct_name(acct_info.get(ACCTNAME.get().unwrap()));
    let cmt_name = acct_name.clone();

    let html = CommentTemplate {thd_id,thd_name,acct_name,cmt_name,comment_list,error_msg,login_status};
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
    .content_type("text/html")
    .body(response_body))
}

//コメントの登録ボタン押下時
#[post("/addComment")]
pub async fn add_thread_comment(params: web::Form<CommentCreateParams>,redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;
    let thd_id = params.thd_id.clone();
    let mut cmt_name = params.cmt_name.clone();
    let cmt = params.cmt.clone();
    

    //エラーがない場合にインサート処理
    let thd_name = params.thd_name.clone();
    let cmt_create_params = CommentCreateParams::new(thd_id,cmt_name,cmt,thd_name);
    let error_msg = cmt_create_params.validation_comment();
    if error_msg.len() <= 0 {
        cmt_create_params.insert_comment(&conn);
    }
    
    //ログインの状態を取得
    let mut redis_conn = redis.get()?;
    let acct_info = get_session(&mut redis_conn, &SESSION_ID);
    let login_status = get_login_status(&acct_info);

    //アカウント名の取得
    let acct_name = get_acct_name(acct_info.get(ACCTNAME.get().unwrap()));
    cmt_name = acct_name.clone();

    //タスクのコメント画面の表示処理
    let thd_name = cmt_create_params.thd_name;
    let comment_list = ThreadComment::select_comment(&conn,thd_id);
    let html = CommentTemplate {thd_id,thd_name,acct_name,cmt_name,comment_list,error_msg,login_status};
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
    .content_type("text/html")
    .body(response_body))
}