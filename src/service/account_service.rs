use actix_web::{web};
use crate::models::{Account,NewAccount,AddAccountParams,EditAccountNameParams,EditPasswordParams};
use diesel::prelude::*;
use crate::schema::account::dsl::*;
use std::vec::Vec;
use diesel::r2d2::{ConnectionManager};
use r2d2::PooledConnection;
use diesel::SqliteConnection;
use pwhash::bcrypt;
use crate::screen_status::ScreenStatus;
use std::collections::HashMap;
use crate::error_msg::{ErrorMsg,GetErrorMsg};

//Accountのリストを全取得
pub fn select_all_account(conn: &PooledConnection<ConnectionManager<SqliteConnection>>) -> Vec<Account>{
    let thread_list = account.load::<Account>(conn).expect("Error loading Account");
    return thread_list;
}

//Accountのリストをネームで取得
pub fn select_account_byname(acct_name: &String,conn: &PooledConnection<ConnectionManager<SqliteConnection>>) -> Vec<Account>{
    let account_list = account.filter(account_name.eq(acct_name)).load::<Account>(conn).expect("Error loading Account");
    return account_list;
}

//Accountのレコードをインサート
pub fn insert_account(acct_no: i32,acct_name: &String,pwd: String,conn: &PooledConnection<ConnectionManager<SqliteConnection>>){
    let new_account = NewAccount{account_no:acct_no,account_name:acct_name,password:pwd};
    diesel::insert_into(account).values(new_account).execute(conn).expect("Insert Error Account");
}

//Accountのaccount_nameをアップデート
pub fn update_account_info(acct_no: &String,edit_acct_name: &String,conn: &PooledConnection<ConnectionManager<SqliteConnection>>){
    diesel::update(account.filter(account_no.eq(acct_no.parse::<i32>().unwrap()))).set(account_name.eq(edit_acct_name)).execute(conn).expect("Update Error Account");
}

//Accountのpasswordをアップデート
pub fn update_password(acct_no: &String,edit_password: String,conn: &PooledConnection<ConnectionManager<SqliteConnection>>){
    diesel::update(account.filter(account_no.eq(acct_no.parse::<i32>().unwrap()))).set(password.eq(edit_password)).execute(conn).expect("Update Error Account");
}

//アカウント名の取得
pub fn get_acct_name(acct_name: Option<&String>) -> String{
    match acct_name {
        Some(v) => return v.to_string(),
        None => return String::from("名無し")
    }
}

//アカウントナンバーの取得
pub fn get_acct_no(acct_no: Option<&String>) -> String{
    match acct_no {
        Some(v) => return v.to_string(),
        None => return String::from("0")
    }
}

//ログイン状態を確認
pub fn get_login_status(acct_info: &HashMap<String, String>) -> String {
    let acct_no_key = String::from("acct_no");
    if let Some(v) = acct_info.get(&acct_no_key) {
        let not_login_acct_no = 0.to_string();
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
    let error_msg_struct = ErrorMsg{};
    let mut error_key: String;

    //必須項目チェック
    if params.acct_name.clone() == String::from(""){
        error_key = String::from("EM_0001");
        error_msg.push(error_msg_struct.get_error_msg_by_place_holder(error_key,String::from("アカウント名")));
    }
    if params.pwd.clone() == String::from(""){
        error_key = String::from("EM_0001");
        error_msg.push(error_msg_struct.get_error_msg_by_place_holder(error_key,String::from("パスワード")));
    }

    //新規登録のみのチェック処理
    if status == ScreenStatus::SIGNUP.to_string() && params.acct_name.clone() != String::from(""){
        let acct_name = params.acct_name.clone();

        //重複チェック
        let temp_list = select_account_byname(&acct_name,conn);
        if temp_list.len() > 0{
            error_key = String::from("EM_ACCT_0001");
            error_msg.push(error_msg_struct.get_error_msg(error_key));
        }

        //パスワードの文字数チェック
        let pwd = params.pwd.clone();
        if pwd.chars().count() < 8{
            error_key = String::from("EM_ACCT_0002");
            error_msg.push(error_msg_struct.get_error_msg(error_key));
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
            let temp_pwd = &temp_list[0].password;
            if !bcrypt::verify(pwd,temp_pwd){
                error_key = String::from("EM_ACCT_0003");
                error_msg.push(error_msg_struct.get_error_msg(error_key));
            }
        }else{
            error_key = String::from("EM_ACCT_0004");
            error_msg.push(error_msg_struct.get_error_msg(error_key));
        }
    }

    return error_msg;
}

//アカウント名変更時のチャック
pub fn validation_account_name(params: &web::Form<EditAccountNameParams>,acct_name: &String,
    conn: &PooledConnection<ConnectionManager<SqliteConnection>>) -> Vec<String>{
    let mut error_msg:Vec<String> = Vec::new();
    let error_msg_struct = ErrorMsg{};
    let error_key: String;
    let edit_acct_name = params.edit_acct_name.clone();
    //必須項目チェック
    if edit_acct_name == String::from(""){
        error_key = String::from("EM_0001");
        error_msg.push(error_msg_struct.get_error_msg_by_place_holder(error_key,String::from("アカウント名")));
        return error_msg;
    }

    //重複チェック
    if &edit_acct_name != acct_name {
        let temp_list = select_account_byname(&edit_acct_name,conn);
        if temp_list.len() > 0 {
            error_key = String::from("EM_ACCT_0001");
            error_msg.push(error_msg_struct.get_error_msg(error_key));
        }
    }

    return error_msg;
}

//パスワード変更時のチャック
pub fn validation_password(params: &web::Form<EditPasswordParams>,acct_name: &String,
    conn: &PooledConnection<ConnectionManager<SqliteConnection>>) -> Vec<String>{
    let mut error_msg:Vec<String> = Vec::new();
    let error_msg_struct = ErrorMsg{};
    let mut error_key: String;
    
    //必須項目チェック
    let current_password = params.current_password.clone();
    let edit_password = params.edit_password.clone();
    if current_password == String::from("") || edit_password == String::from("") {
        error_key = String::from("EM_0001");
        error_msg.push(error_msg_struct.get_error_msg_by_place_holder(error_key,String::from("パスワード")));
        return error_msg;
    }

    //現在のパスワードのチャック
    let temp_list = select_account_byname(&acct_name,conn);
    let temp_pwd = &temp_list[0].password;
    if !bcrypt::verify(current_password,temp_pwd){
        error_key = String::from("EM_ACCT_0005");
        error_msg.push(error_msg_struct.get_error_msg(error_key));
    }

    //変更後パスワードの文字数チェック
    if edit_password.chars().count() < 8{
        error_key = String::from("EM_ACCT_0006");
        error_msg.push(error_msg_struct.get_error_msg(error_key));
    }

    return error_msg;
}