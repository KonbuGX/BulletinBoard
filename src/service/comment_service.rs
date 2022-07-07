use actix_web::{web};
use crate::models::{ThreadComment,NewThreadComment,AddCommentParams};
use diesel::prelude::*;
use crate::schema::thread_comment::dsl::*;
use std::vec::Vec;
use diesel::r2d2::{ConnectionManager};
use r2d2::PooledConnection;
use diesel::SqliteConnection;

//ThreadCommentのリストをid指定で取得
pub fn select_all_comment(conn: &PooledConnection<ConnectionManager<SqliteConnection>>) -> Vec<ThreadComment>{
    let comment_list = thread_comment.load::<ThreadComment>(conn).expect("Error loading Thread");
    return comment_list;
}

//ThreadCommentのリストをスレッドのid指定で取得
pub fn select_comment(conn: &PooledConnection<ConnectionManager<SqliteConnection>>,thd_id: i32) -> Vec<ThreadComment>{
    let comment_list = thread_comment.filter(thread_id.eq(thd_id)).load::<ThreadComment>(conn).expect("Error loading Thread");
    return comment_list;
}

//Threadのレコードをインサート
pub fn insert_comment(thd_id: i32,cmt_name: String,cmt: String,conn: &PooledConnection<ConnectionManager<SqliteConnection>>){
    let thread_comment_list = select_all_comment(&conn);

    //commentNoの付番処理
    let mut cmt_no:i32 = 0;
    if thread_comment_list.len() > 0 {
        for temp in &thread_comment_list {
            if temp.comment_no > cmt_no {
                cmt_no = temp.comment_no;
            }  
        }
        cmt_no += 1;
    }else {
        cmt_no = 1;
    }
    
    let new_thread_comment = NewThreadComment{thread_id:thd_id,comment_no:cmt_no,comment_name:cmt_name,comment:cmt};
    diesel::insert_into(thread_comment).values(new_thread_comment).execute(conn).expect("Insert Error Thread");
}

//コメントのレコードをデリート
pub fn remove_comment(thd_id: i32,conn: &PooledConnection<ConnectionManager<SqliteConnection>>){
    diesel::delete(thread_comment.filter(thread_id.eq(thd_id))).execute(conn).expect("Delete Error Comment");
}

//チェック処理
pub fn validation_comment(params: &web::Form<AddCommentParams>) -> Vec<String>{
    let mut error_msg:Vec<String> = Vec::new();

    //必須項目チェック
    if params.cmt_name.clone() == String::from(""){
        error_msg.push(String::from("ネームが未入力です。"));
    }
    if params.cmt.clone() == String::from(""){
        error_msg.push(String::from("コメントが未入力です。"));
    }

    return error_msg;
}