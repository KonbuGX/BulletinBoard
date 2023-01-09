use crate::models::{Account,NewAccount,AccountAddParams,AccountNameEditParams,PasswordEditParams};
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
use crate::ACCTNO;
use regex::Regex;
use crate::REGEX_ALPHANUMERIC;

impl Account{
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
}

impl NewAccount{
    //NewAccountインスタンスの作成
    pub fn new(acct_no: i32,acct_name: String,pwd: String) 
    -> NewAccount{
        NewAccount{
            account_no: acct_no,
            account_name: acct_name,
            password: pwd,
        }
    }

    //Accountのレコードをインサート
    pub fn insert_account(&self,conn: &PooledConnection<ConnectionManager<SqliteConnection>>){
        diesel::insert_into(account).values(self).execute(conn).expect("Insert Error Account");
    }
}

impl AccountAddParams{
    //AccountAddParamsインスタンスの作成
    pub fn new(acct_name: String,pwd: String,) 
    -> AccountAddParams{
        AccountAddParams{
            acct_name: acct_name,
            pwd: pwd,
        }
    }

    //チェック処理
    pub fn validation_account(&self,status: String,
        conn: &PooledConnection<ConnectionManager<SqliteConnection>>) -> Vec<String>{
        let mut error_msg:Vec<String> = Vec::new();
        let error_msg_struct = ErrorMsg{};
        let mut error_key: String;

        //必須項目チェック
        let acct_name = self.acct_name.to_string();
        let pwd = self.pwd.to_string();
        if acct_name == String::from(""){
            error_key = String::from("EM_0001");
            error_msg.push(error_msg_struct.get_error_msg_by_place_holder(error_key,String::from("アカウント名")));
        }
        if pwd == String::from(""){
            error_key = String::from("EM_0001");
            error_msg.push(error_msg_struct.get_error_msg_by_place_holder(error_key,String::from("パスワード")));
        }

        //新規登録のみのチェック処理
        if status == ScreenStatus::SIGNUP.to_string() && acct_name != String::from(""){
            
            //重複チェック
            let temp_list = Account::select_account_byname(&acct_name,conn);
            if temp_list.len() > 0{
                error_key = String::from("EM_ACCT_0001");
                error_msg.push(error_msg_struct.get_error_msg(error_key));
            }

            //パスワードの文字数チェック
            if pwd.chars().count() < 8{
                error_key = String::from("EM_ACCT_0002");
                error_msg.push(error_msg_struct.get_error_msg(error_key));
            }

            //パスワードのフォーマットチェック
            let regex_format = Regex::new(REGEX_ALPHANUMERIC).unwrap();
            if !regex_format.is_match(&pwd) {
                error_key = String::from("EM_ACCT_0007");
                error_msg.push(error_msg_struct.get_error_msg(error_key));
            }

            return error_msg;
        }

        //ログインのみのチェック処理
        if status == ScreenStatus::LOGIN.to_string() && acct_name != String::from(""){
            //アカウントが存在するかどうかチャック
            let temp_list = Account::select_account_byname(&acct_name,conn);
            if temp_list.len() > 0{
                //アカウントがある場合にパスワードのチェック
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
}

impl AccountNameEditParams{
    //AccountNameEditParamsインスタンスの作成
    pub fn new(edit_acct_name: String) 
    -> AccountNameEditParams{
        AccountNameEditParams{
            edit_acct_name: edit_acct_name,
        }
    }

    //Accountのaccount_nameをアップデート
    pub fn update_account_info(&self,acct_no: &String,conn: &PooledConnection<ConnectionManager<SqliteConnection>>){
        diesel::update(account.filter(account_no.eq(acct_no.parse::<i32>().unwrap()))).set(account_name.eq(&self.edit_acct_name)).execute(conn).expect("Update Error Account");
    }

    //アカウント名変更時のチャック
    pub fn validation_account_name(&self,acct_name: &String,
        conn: &PooledConnection<ConnectionManager<SqliteConnection>>) -> Vec<String>{
        let mut error_msg:Vec<String> = Vec::new();
        let error_msg_struct = ErrorMsg{};
        let error_key: String;
        let edit_acct_name = self.edit_acct_name.to_string();
        //必須項目チェック
        if edit_acct_name == String::from(""){
            error_key = String::from("EM_0001");
            error_msg.push(error_msg_struct.get_error_msg_by_place_holder(error_key,String::from("アカウント名")));
            return error_msg;
        }

        //重複チェック
        if &edit_acct_name != acct_name {
            let temp_list = Account::select_account_byname(&edit_acct_name,conn);
            if temp_list.len() > 0 {
                error_key = String::from("EM_ACCT_0001");
                error_msg.push(error_msg_struct.get_error_msg(error_key));
            }
        }

        return error_msg;
    }

    
}

impl PasswordEditParams{
    //NewAccountインスタンスの作成
    pub fn new(current_password: String,edit_password: String) 
    -> PasswordEditParams{
        PasswordEditParams{
            current_password: current_password,
            edit_password: edit_password,
        }
    }

    //Accountのpasswordをアップデート
    pub fn update_password(&self,acct_no: &String,conn: &PooledConnection<ConnectionManager<SqliteConnection>>){
        diesel::update(account.filter(account_no.eq(acct_no.parse::<i32>().unwrap()))).set(password.eq(&self.edit_password)).execute(conn).expect("Update Error Account");
    }

    //パスワード変更時のチャック
    pub fn validation_password(&self,acct_name: &String,
        conn: &PooledConnection<ConnectionManager<SqliteConnection>>) -> Vec<String>{
        let mut error_msg:Vec<String> = Vec::new();
        let error_msg_struct = ErrorMsg{};
        let mut error_key: String;
    
        //必須項目チェック
        let current_password = self.current_password.clone();
        let edit_password = self.edit_password.clone();
        if current_password == String::from("") || edit_password == String::from("") {
            error_key = String::from("EM_0001");
            error_msg.push(error_msg_struct.get_error_msg_by_place_holder(error_key,String::from("パスワード")));
            return error_msg;
        }

        //現在のパスワードのチャック
        let temp_list = Account::select_account_byname(&acct_name,conn);
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

        //変更後パスワードのフォーマットチェック
        let regex_format = Regex::new(REGEX_ALPHANUMERIC).unwrap();
        if !regex_format.is_match(&edit_password) {
            error_key = String::from("EM_ACCT_0007");
            error_msg.push(error_msg_struct.get_error_msg(error_key));
        }

        return error_msg;
    }
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
    if let Some(v) = acct_info.get(ACCTNO.get().unwrap()) {
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