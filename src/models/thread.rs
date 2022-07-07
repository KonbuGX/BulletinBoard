use chrono::{NaiveDateTime};
use diesel::{Queryable,Insertable};
use crate::schema::thread;
use serde::Deserialize;

//スレッドの構造体
#[derive(Queryable, PartialEq, Debug)]
pub struct Thread{
    pub thread_id: i32,
    pub thread_name: String,
    pub lastupdate: Option<NaiveDateTime>,
}

//スレッドのインサート用構造体
#[derive(Debug,Insertable)]
#[table_name = "thread"]
pub struct NewThread{
    pub thread_id: i32,
    pub thread_name: String,
}

//検索キーワード取得用構造体
#[derive(Deserialize,Clone)]
pub struct AddTreadSearchParams{
    pub search_keyword: String,
}

//スレッドネーム取得用構造体
#[derive(Deserialize,Clone)]
pub struct AddTreadParams{
    pub thd_name: String,
}

//スレッドの削除パラメータ取得用構造体
#[derive(Deserialize,Clone)]
pub struct DeleteTreadParams{
    pub thd_id: i32,
    pub thd_name: String,
}