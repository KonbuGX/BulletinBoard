use chrono::{NaiveDateTime};
use diesel::{Queryable,Insertable};
use crate::schema::account;
use serde::Deserialize;

//スレッドの構造体
#[derive(Queryable, PartialEq, Debug)]
pub struct Account{
    pub account_no: i32,
    pub account_name: String,
    pub password: String,
    pub lastupdate: Option<NaiveDateTime>,
}

//スレッドのインサート用構造体
#[derive(Debug,Insertable)]
#[table_name = "account"]
pub struct NewAccount<'a>{
    pub account_no: i32,
    pub account_name: &'a String,
    pub password: String,
}

//アカウントのパラメータ取得用構造体
#[derive(Deserialize,Clone)]
pub struct AddAccountParams{
    pub acct_name: String,
    pub pwd: String,
}

//編集後アカウントネーム取得用構造体
#[derive(Deserialize,Clone)]
pub struct EditAccountNameParams{
    pub edit_acct_name: String,
}

//パスワード取得用構造体
#[derive(Deserialize,Clone)]
pub struct EditPasswordParams{
    pub current_password: String,
    pub edit_password: String,
}