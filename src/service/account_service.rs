use actix_web::{web};
use crate::models::{Account,NewAccount,AddAccountParams};
use diesel::prelude::*;
use crate::schema::account::dsl::*;
use std::vec::Vec;
use diesel::r2d2::{ConnectionManager};
use r2d2::PooledConnection;
use diesel::SqliteConnection;
use pwhash::bcrypt;
use crate::screen_status::ScreenStatus;
use std::collections::HashMap;

//Accountのリストを全取得
pub fn select_all_account(conn: &PooledConnection<ConnectionManager<SqliteConnection>>) -> Vec<Account>{
    let thread_list = account.load::<Account>(conn).expect("Error loading Account");
    return thread_list;
}

//Accountのリストをネームで取得
pub fn select_account_byname(acct_name: &String,conn: &PooledConnection<ConnectionManager<SqliteConnection>>) -> Vec<Account>{
    let format = format!("%{}%", acct_name);
    let account_list = account.filter(account_name.like(format)).load::<Account>(conn).expect("Error loading Account");
    return account_list;
}

//Accountのレコードをインサート
pub fn insert_account(acct_no: i32,acct_name: &String,pwd: String,conn: &PooledConnection<ConnectionManager<SqliteConnection>>){
    let new_account = NewAccount{account_no:acct_no,account_name:acct_name,password:pwd};
    diesel::insert_into(account).values(new_account).execute(conn).expect("Insert Error Account");
}

//ログイン状態を確認
pub fn return_login_status(acct_info: &HashMap<String, String>) -> String {
    let acct_no_key = String::from("acct_no");
    if let Some(v) = acct_info.get(&acct_no_key) {
        let not_login_acct_no = 9999.to_string();
        if v != &not_login_acct_no {
            return ScreenStatus::LOGIN.to_string();
        }else{
            return ScreenStatus::LOGOUT.to_string();
        }
    }else{
        return ScreenStatus::LOGOUT.to_string();
    }
}

//Accountのレコードをデリート
pub fn remove_account(acct_no: i32,conn: PooledConnection<ConnectionManager<SqliteConnection>>){
    diesel::delete(account.filter(account_no.eq(acct_no))).execute(&conn).expect("Delete Error Account");
}

//チェック処理
pub fn validation_account(params: &web::Form<AddAccountParams>,status: String,
    conn: &PooledConnection<ConnectionManager<SqliteConnection>>) -> Vec<String>{
    let mut error_msg:Vec<String> = Vec::new();

    //必須項目チェック
    if params.acct_name.clone() == String::from(""){
        error_msg.push(String::from("アカウント名が未入力です。"));
    }
    if params.pwd.clone() == String::from(""){
        error_msg.push(String::from("パスワードが未入力です。"));
    }

    //新規登録のみのチェック処理
    if status == ScreenStatus::SIGNUP.to_string() && params.acct_name.clone() != String::from(""){
        let acct_name = params.acct_name.clone();

        //重複チェック
        let temp_list = select_account_byname(&acct_name,conn);
        if temp_list.len() > 0{
            error_msg.push(String::from("アカウント名が重複しています。"));
        }

        //パスワードの文字数チェック
        let pwd = params.pwd.clone();
        if pwd.chars().count() < 8{
            error_msg.push(String::from("パスワードの文字数を8文字以上にしてください。"));
        }

        return error_msg;
    }

    //ログインのみのチェック処理
    if status == ScreenStatus::LOGIN.to_string() && params.acct_name.clone() != String::from(""){
        //アカウントが存在するかどうかチャック
        let acct_name = params.acct_name.clone();
        let temp_list = select_account_byname(&acct_name,conn);
        if temp_list.len() > 0{
            //アカウントがある場合にパスワードのチェック
            let pwd = params.pwd.clone();
            let acct_name = params.acct_name.clone();
            let temp_list = select_account_byname(&acct_name,conn);
            let temp_pwd = &temp_list[0].password;
            if !bcrypt::verify(pwd,temp_pwd){
                error_msg.push(String::from("パスワードが違います。"));
            }
        }else {
            error_msg.push(String::from("アカウントが存在していません。"));
        }
    }

    return error_msg;
}