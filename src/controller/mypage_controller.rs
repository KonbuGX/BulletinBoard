use actix_web::{get,post,web,HttpResponse};
use askama::{Template};
use diesel::r2d2::{ConnectionManager};
use diesel::sqlite::{SqliteConnection};
use pwhash::bcrypt;
use std::collections::HashMap;

use crate::MyError;
use crate::models::Thread;
use crate::models::EditAccountNameParams;
use crate::models::EditPasswordParams;
use crate::models::DeleteTreadParams;
use crate::models::AddTreadSearchParams;
use crate::service::select_all_thred;
use crate::service::select_thred_name;
use crate::service::get_login_status;
use crate::service::get_acct_name;
use crate::service::get_acct_no;
use crate::service::update_account_info;
use crate::service::update_password;
use crate::service::validation_account_name;
use crate::service::validation_password;
use crate::session_management::set_session;
use crate::session_management::get_session;
use crate::session_management::delete_session;

use crate::service::remove_thread;
use crate::service::remove_comment;

use crate::ACCTNO;
use crate::ACCTNAME;
use crate::SESSION_ID;

#[derive(Template)]
#[template(path = "mypage.html")]
struct MyPageTemplate{
    acct_no: String,
    acct_name: String,
    login_status: String,
}

#[derive(Template)]
#[template(path = "account.html")]
struct EditAccountTemplate{
    acct_no: String,
    acct_name: String,
    login_status: String,
    info_msg: String,
    error_msg: Vec<String>,
}

#[derive(Template)]
#[template(path = "password.html")]
struct EditPasswordTemplate{
    acct_no: String,
    acct_name: String,
    login_status: String,
    info_msg: String,
    error_msg: Vec<String>,
}

#[derive(Template)]
#[template(path = "thread.html")]
struct DeleteThreadTemplate{
    acct_no: String,
    acct_name: String,
    login_status: String,
    search_keyword: String,
    info_msg: String,
    thread_list: Vec<Thread>,
    error_msg: Vec<String>,
}

//ログイン中のアカウントのページ表示
#[get("/mypage")]
pub async fn mypage(redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>) -> Result<HttpResponse,MyError>{
    //ログインの状態を取得
    let mut redis_conn = redis.get()?;
    let acct_info = get_session(&mut redis_conn, &SESSION_ID);
    let login_status = get_login_status(&acct_info);

    //アカウントNo、アカウント名の取得
    let acct_no = get_acct_no(acct_info.get(ACCTNO.get().unwrap()));
    let acct_name = get_acct_name(acct_info.get(ACCTNAME.get().unwrap()));

    let html = MyPageTemplate {acct_no,acct_name,login_status};
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
    .content_type("text/html")
    .body(response_body))
}

//アカウントネーム変更画面のページ表示
#[get("/account")]
pub async fn account(redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>) -> Result<HttpResponse,MyError>{
    //ログインの状態を取得
    let mut redis_conn = redis.get()?;
    let acct_info = get_session(&mut redis_conn, &SESSION_ID);
    let login_status = get_login_status(&acct_info);
    
    //アカウント名の取得
    let acct_no = get_acct_no(acct_info.get(ACCTNO.get().unwrap()));
    let acct_name = get_acct_name(acct_info.get(ACCTNAME.get().unwrap()));

    let info_msg = String::new();
    let error_msg = Vec::new();
    let html = EditAccountTemplate {acct_no,acct_name,login_status,info_msg,error_msg};
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
    .content_type("text/html")
    .body(response_body))
}

//アカウントネーム変更画面で変更ボタン押下
#[post("/account")]
pub async fn edit_account(params: web::Form<EditAccountNameParams>,redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;
    let mut redis_conn = redis.get()?;

    //アカウントNo、アカウント名の取得
    let mut acct_info = get_session(&mut redis_conn, &SESSION_ID);
    let acct_no = get_acct_no(acct_info.get(ACCTNO.get().unwrap()));
    let mut acct_name = get_acct_name(acct_info.get(ACCTNAME.get().unwrap()));

    let error_msg = validation_account_name(&params,&acct_name,&conn);
    if error_msg.len() > 0{
        //ログインの状態を取得
        let login_status = get_login_status(&acct_info);
        
        let info_msg = String::new();
        let html = EditAccountTemplate {acct_no,acct_name,login_status,info_msg,error_msg};
        let response_body = html.render()?;
        Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
    }else{
        //アカウントネームを更新
        let edit_acct_name = params.edit_acct_name.clone();
        update_account_info(&acct_no, &edit_acct_name, &conn);

        //新しいアカウントネームをセットする。
        let account_info_key = vec![ACCTNAME.get().unwrap()];
        delete_session(&mut redis_conn,&SESSION_ID, account_info_key);

        let mut new_account_info: HashMap<&String,String> = HashMap::new();
        new_account_info.insert(ACCTNAME.get().unwrap(), edit_acct_name);
        set_session(&mut redis_conn,&SESSION_ID, new_account_info);

        //ログインの状態を取得
        acct_info = get_session(&mut redis_conn, &SESSION_ID);
        let login_status = get_login_status(&acct_info);

        acct_name = get_acct_name(acct_info.get(ACCTNAME.get().unwrap()));
        let info_msg = String::from("アカウント名を変更しました。");
        let html = EditAccountTemplate {acct_no,acct_name,login_status,info_msg,error_msg};
        let response_body = html.render()?;
        Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
    }
}

//パスワード変更画面の表示
#[get("/password")]
pub async fn password(redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>) -> Result<HttpResponse,MyError>{
    //ログインの状態を取得
    let mut redis_conn = redis.get()?;
    let acct_info = get_session(&mut redis_conn, &SESSION_ID);
    let login_status = get_login_status(&acct_info);
    
    //アカウント名の取得
    let acct_no = get_acct_no(acct_info.get(ACCTNO.get().unwrap()));
    let acct_name = get_acct_name(acct_info.get(ACCTNAME.get().unwrap()));

    let info_msg = String::new();
    let error_msg = Vec::new();
    let html = EditPasswordTemplate {acct_no,acct_name,login_status,info_msg,error_msg};
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
    .content_type("text/html")
    .body(response_body))
}

//パスワード変更画面の変更ボタン押下時
#[post("/password")]
pub async fn edit_password(params: web::Form<EditPasswordParams>,redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;
    let mut redis_conn = redis.get()?;

    //アカウント名の取得
    let acct_info = get_session(&mut redis_conn, &SESSION_ID);
    let acct_no = get_acct_no(acct_info.get(ACCTNO.get().unwrap()));
    let acct_name = get_acct_name(acct_info.get(ACCTNAME.get().unwrap()));

    let error_msg = validation_password(&params,&acct_name,&conn);
    if error_msg.len() > 0{
        //ログインの状態を取得
        let login_status = get_login_status(&acct_info);

        let info_msg = String::new();
        let html = EditPasswordTemplate {acct_no,acct_name,login_status,info_msg,error_msg};
        let response_body = html.render()?;
        Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
    }else{
        //パスワードを変更
        let edit_password = bcrypt::hash(params.edit_password.clone()).unwrap();
        update_password(&acct_no, edit_password, &conn);

        //ログインの状態を取得
        let login_status = get_login_status(&acct_info);

        let info_msg = String::from("パスワードを変更しました。");
        let html = EditPasswordTemplate {acct_no,acct_name,login_status,info_msg,error_msg};
        let response_body = html.render()?;
        Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
    }
}

//スレッド削除画面の表示
#[get("/deleteThread")]
pub async fn delete_thread_list(redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    //ログインの状態を取得
    let mut redis_conn = redis.get()?;
    let acct_info = get_session(&mut redis_conn, &SESSION_ID);
    let login_status = get_login_status(&acct_info);

    //スレッドリストの取得
    let conn = db.get()?;
    let thread_list = select_all_thred(&conn);

    //アカウントNo、アカウント名の取得
    let acct_no = get_acct_no(acct_info.get(ACCTNO.get().unwrap()));
    let acct_name = get_acct_name(acct_info.get(ACCTNAME.get().unwrap()));

    //検索エリアの検索文字,インフォメッセージ、エラーメッセージの初期化
    let search_keyword = String::from("");
    let info_msg = String::from("");
    let error_msg = Vec::new();

    let html = DeleteThreadTemplate {acct_no,acct_name,login_status,search_keyword,info_msg,thread_list,error_msg};
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
    .content_type("text/html")
    .body(response_body))
}

//削除ボタン押下時
#[post("/deleteThread")]
pub async fn delete_thread(params: web::Form<DeleteTreadParams>,redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;

    //ログインの状態を取得
    let mut redis_conn = redis.get()?;
    let acct_info = get_session(&mut redis_conn, &SESSION_ID);
    let login_status = get_login_status(&acct_info);

    //アカウントNo、アカウント名の取得
    let acct_no = get_acct_no(acct_info.get(ACCTNO.get().unwrap()));
    let acct_name = get_acct_name(acct_info.get(ACCTNAME.get().unwrap()));

    //スレッドとそれに付随するコメントを削除
    let remove_status = remove_thread(params.thd_id.clone(),params.thd_name.clone(),&conn);
    if remove_status != String::from(""){
        //スレッドリストの取得
        let thread_list = select_all_thred(&conn);
        
        //検索エリアの検索文字,インフォメッセージ、エラーメッセージの初期化
        let search_keyword = String::from("");
        let info_msg = String::from("");
        let mut error_msg = Vec::new();
        error_msg.push(remove_status);

        let html = DeleteThreadTemplate {acct_no,acct_name,login_status,search_keyword,info_msg,thread_list,error_msg};
        let response_body = html.render()?;
        Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
    }else{
        //スレッドコメントの削除
        remove_comment(params.thd_id.clone(),&conn);

        //スレッドリストの取得
        let thread_list = select_all_thred(&conn);

        //検索エリアの検索文字,インフォメッセージ、エラーメッセージの初期化
        let search_keyword = String::from("");
        let info_msg = String::from("スレッドを削除いたしました。");
        let error_msg = Vec::new();

        let html = DeleteThreadTemplate {acct_no,acct_name,login_status,search_keyword,info_msg,thread_list,error_msg};
        let response_body = html.render()?;
        Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
    }
}

//検索ボタン押下時
#[post("/searchThread_mypage")]
pub async fn search_thread_mypage(params: web::Form<AddTreadSearchParams>,redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;

    //検索したスレッドリストの取得
    let search_keyword = params.search_keyword.clone();
    let thread_list = select_thred_name(&search_keyword,&conn);

    //ログインの状態を取得
    let mut redis_conn = redis.get()?;
    let acct_info = get_session(&mut redis_conn, &SESSION_ID);
    let login_status = get_login_status(&acct_info);

    //アカウントNo、アカウント名の取得
    let acct_no = get_acct_no(acct_info.get(ACCTNO.get().unwrap()));
    let acct_name = get_acct_name(acct_info.get(ACCTNAME.get().unwrap()));

    //インフォメッセージ、エラーメッセージの初期化
    let info_msg = String::from("");
    let error_msg = Vec::new();

    let html = DeleteThreadTemplate {acct_no,acct_name,login_status,search_keyword,info_msg,thread_list,error_msg};
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
    .content_type("text/html")
    .body(response_body))
}