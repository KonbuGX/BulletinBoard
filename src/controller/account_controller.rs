use actix_web::{get,post,web,HttpResponse,http::header};
use askama::{Template};
use diesel::r2d2::{ConnectionManager};
use diesel::sqlite::{SqliteConnection};
use std::collections::HashMap;
use uuid::Uuid;
use pwhash::bcrypt;

use crate::MyError;
use crate::models::{Account,NewAccount,AccountAddParams};
use crate::session_management::set_session;
use crate::screen_status::*;
use crate::ACCTNO;
use crate::ACCTNAME;
use crate::SESSION_ID;

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

//ログイン画面表示
#[get("/login")]
pub async fn login() -> Result<HttpResponse,MyError>{
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
pub async fn login_account(params: web::Form<AccountAddParams>,redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;
    let mut acct_name = params.acct_name.clone();
    let mut pwd = params.pwd.clone();
    let acct_add_params = AccountAddParams::new(acct_name,pwd);

    let status = ScreenStatus::LOGIN.to_string();
    let error_msg = acct_add_params.validation_account(status,&conn);
    if error_msg.len() > 0{
        //エラーメッセージをログイン画面に表示
        acct_name = String::from("");
        pwd = String::from("");
        let html = LoginTemplate {acct_name,pwd,error_msg};
        let response_body = html.render()?;
        Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
    }else{
        //アカウントナンバーをセッションに追加。
        let temp_list = Account::select_account_byname(&acct_add_params.acct_name, &conn);
        let acct_no = temp_list[0].account_no;
        let mut redis_conn = redis.get()?;
        &SESSION_ID.set(Uuid::new_v4().to_string());

        let mut account_info: HashMap<&String,String> = HashMap::new();
        account_info.insert(ACCTNO.get().unwrap(), acct_no.to_string());
        account_info.insert(ACCTNAME.get().unwrap(), acct_add_params.acct_name);
        set_session(&mut redis_conn,&SESSION_ID, account_info);

        //ホーム画面の表示
        Ok(HttpResponse::SeeOther().append_header((header::LOCATION,"/")).finish())
    }
}

//新規登録画面表示
#[get("/signup")]
pub async fn signup() -> Result<HttpResponse,MyError>{
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
pub async fn signup_account(params: web::Form<AccountAddParams>,redis: web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>,db: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse,MyError>{
    let conn = db.get()?;
    let mut acct_name = params.acct_name.clone();
    let mut pwd = params.pwd.clone();
    let acct_add_params = AccountAddParams::new(acct_name,pwd);

    let status = ScreenStatus::SIGNUP.to_string();
    let error_msg = acct_add_params.validation_account(status,&conn);
    if error_msg.len() > 0{
        //エラーメッセージを新規登録画面に表示
        acct_name = String::from("");
        pwd = String::from("");
        let html = SignUpTemplate {acct_name,pwd,error_msg};
        let response_body = html.render()?;
        Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
    }else{
        //accountNoの付番処理
        let account_list = Account::select_all_account(&conn);
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
        pwd = bcrypt::hash(acct_add_params.pwd).unwrap();
        let new_account = NewAccount::new(acct_no,acct_add_params.acct_name,pwd);
        new_account.insert_account(&conn);

        //アカウント情報をセッションに追加
        &SESSION_ID.set(Uuid::new_v4().to_string());
        let mut redis_conn = redis.get()?;
        let mut account_info: HashMap<&String,String> = HashMap::new();
        account_info.insert(ACCTNO.get().unwrap(), new_account.account_no.to_string());
        account_info.insert(ACCTNAME.get().unwrap(), new_account.account_name);
        set_session(&mut redis_conn, &SESSION_ID, account_info);

        //ホーム画面の表示
        Ok(HttpResponse::SeeOther().append_header((header::LOCATION,"/")).finish())
    }
}