use actix_web::{get,post,web,HttpResponse};
use askama::{Template};
use diesel::r2d2::{ConnectionManager};
use diesel::sqlite::{SqliteConnection};
use pwhash::bcrypt;
use std::collections::HashMap;

use crate::MyError;
use crate::models::{Thread,ThreadDeleteParams,ThreadSearchParams,AccountNameEditParams,PasswordEditParams};
use crate::service::get_login_status;
use crate::service::get_acct_name;
use crate::service::get_acct_no;
use crate::session_management::set_session;
use crate::session_management::get_session;
use crate::session_management::delete_session;

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
pub async fn edit_account(params: web::Form<AccountNameEditParams>,redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;
    let mut redis_conn = redis.get()?;

    //アカウントNo、アカウント名の取得
    let mut acct_info = get_session(&mut redis_conn, &SESSION_ID);
    let acct_no = get_acct_no(acct_info.get(ACCTNO.get().unwrap()));
    let mut acct_name = get_acct_name(acct_info.get(ACCTNAME.get().unwrap()));
    
    //アカウントネームのチャック処理
    let edit_acct_name = params.edit_acct_name.clone();
    let acct_name_edit_params = AccountNameEditParams::new(edit_acct_name);
    let error_msg = acct_name_edit_params.validation_account_name(&acct_name,&conn);
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
        acct_name_edit_params.update_account_info(&acct_no, &conn);

        //新しいアカウントネームをセットする。
        let account_info_key = vec![ACCTNAME.get().unwrap()];
        delete_session(&mut redis_conn,&SESSION_ID, account_info_key);

        let mut new_account_info: HashMap<&String,String> = HashMap::new();
        new_account_info.insert(ACCTNAME.get().unwrap(), acct_name_edit_params.edit_acct_name);
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
pub async fn edit_password(params: web::Form<PasswordEditParams>,redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;
    let mut redis_conn = redis.get()?;

    //アカウント名の取得
    let acct_info = get_session(&mut redis_conn, &SESSION_ID);
    let acct_no = get_acct_no(acct_info.get(ACCTNO.get().unwrap()));
    let acct_name = get_acct_name(acct_info.get(ACCTNAME.get().unwrap()));

    //パスワードのチェック処理
    let current_password = params.current_password.clone();
    let edit_password = params.edit_password.clone();
    let mut pwd_edit_params = PasswordEditParams::new(current_password,edit_password);
    let error_msg = pwd_edit_params.validation_password(&acct_name,&conn);
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
        pwd_edit_params.edit_password = bcrypt::hash(pwd_edit_params.edit_password).unwrap();
        pwd_edit_params.update_password(&acct_no, &conn);

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
    let thread_list = Thread::select_all_thred(&conn);

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
pub async fn delete_thread(params: web::Form<ThreadDeleteParams>,redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;

    //ログインの状態を取得
    let mut redis_conn = redis.get()?;
    let acct_info = get_session(&mut redis_conn, &SESSION_ID);
    let login_status = get_login_status(&acct_info);

    //アカウントNo、アカウント名の取得
    let acct_no = get_acct_no(acct_info.get(ACCTNO.get().unwrap()));
    let acct_name = get_acct_name(acct_info.get(ACCTNAME.get().unwrap()));

    //スレッドとそれに付随するコメントを削除
    let thd_id = params.thd_id.clone();
    let thd_name = params.thd_name.clone();
    let thd_delete_params = ThreadDeleteParams::new(thd_id,thd_name);
    let remove_status = thd_delete_params.remove_thread(&conn);
    if remove_status != String::from(""){
        //スレッドリストの取得
        let thread_list = Thread::select_all_thred(&conn);
        
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
        thd_delete_params.remove_comment(&conn);

        //スレッドリストの取得
        let thread_list = Thread::select_all_thred(&conn);

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
pub async fn search_thread_mypage(params: web::Form<ThreadSearchParams>,redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;

    //検索したスレッドリストの取得
    let thd_search_params = ThreadSearchParams::new(params.search_keyword.clone());
    let thread_list = thd_search_params.select_thred_name(&conn);

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

    let search_keyword = thd_search_params.search_keyword;
    let html = DeleteThreadTemplate {acct_no,acct_name,login_status,search_keyword,info_msg,thread_list,error_msg};
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
    .content_type("text/html")
    .body(response_body))
}