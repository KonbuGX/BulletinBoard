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
pub struct AddCommentParams{
    pub tid: i32,
    pub cname: String,
    pub cmt: String,
    pub tname: String,
}

//スレッドのid,スレッド名取得用構造体
#[derive(Deserialize,Clone)]
pub struct GetThreadParams{
    pub tid: i32,
    pub tname: String,
}