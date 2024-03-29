use chrono::{NaiveDateTime};
use diesel::{Insertable};
use crate::schema::thread_comment;
use serde::Deserialize;

//コメントの構造体
#[derive(Queryable, PartialEq, Debug)]
pub struct ThreadComment{
    pub thread_id: i32,
    pub comment_no: i32,
    pub comment_name: String,
    pub comment: String,
    pub lastupdate: Option<NaiveDateTime>,
}

//コメントのインサート用構造体
#[derive(Debug,Insertable)]
#[table_name = "thread_comment"]
pub struct NewThreadComment{
    pub thread_id: i32,
    pub comment_no: i32,
    pub comment_name: String,
    pub comment: String,
}

//コメントのパラメータ取得用構造体
#[derive(Deserialize,Clone)]
pub struct CommentCreateParams{
    pub thd_id: i32,
    pub cmt_name: String,
    pub cmt: String,
    pub thd_name: String,
}

//スレッドのid,スレッド名取得用構造体
#[derive(Deserialize,Clone)]
pub struct ThreadGetParams{
    pub thd_id: i32,
    pub thd_name: String,
}