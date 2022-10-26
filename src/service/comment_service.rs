use crate::models::{ThreadComment,NewThreadComment,CommentCreateParams,ThreadDeleteParams};
use diesel::prelude::*;
use crate::schema::thread_comment::dsl::*;
use std::vec::Vec;
use diesel::r2d2::{ConnectionManager};
use r2d2::PooledConnection;
use diesel::SqliteConnection;
use crate::error_msg::{ErrorMsg,GetErrorMsg};

impl ThreadComment{
    //ThreadCommentのリストを取得
    pub fn select_all_comment(conn: &PooledConnection<ConnectionManager<SqliteConnection>>) -> Vec<ThreadComment>{
        let comment_list = thread_comment.load::<ThreadComment>(conn).expect("Error loading Thread");
        return comment_list;
    }

    //ThreadCommentのリストをスレッドのid指定で取得
    pub fn select_comment(conn: &PooledConnection<ConnectionManager<SqliteConnection>>,thd_id: i32) -> Vec<ThreadComment>{
        let comment_list = thread_comment.filter(thread_id.eq(thd_id)).load::<ThreadComment>(conn).expect("Error loading Thread");
        return comment_list;
    }
}

impl NewThreadComment{
    //NewThreadCommentインスタンスの作成
    fn new(thd_id: i32,cmt_no: i32,cmt_name: String,cmt: String,) 
    -> NewThreadComment{
        NewThreadComment{
            thread_id: thd_id,
            comment_no: cmt_no,
            comment_name: cmt_name,
            comment: cmt,
        }
    }
}

impl CommentCreateParams{
    //CommentCreateParamsインスタンスの作成
    pub fn new(thd_id: i32,cmt_name: String,cmt: String,thd_name: String) 
    -> CommentCreateParams{
        CommentCreateParams{
            thd_id: thd_id,
            cmt_name: cmt_name,
            cmt: cmt,
            thd_name: thd_name,
        }
    }

    //Commentのレコードをインサート
    pub fn insert_comment(&self,conn: &PooledConnection<ConnectionManager<SqliteConnection>>){
        let thread_comment_list = ThreadComment::select_all_comment(&conn);

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
        
        let thd_id = self.thd_id;
        let cmt_name = self.cmt_name.to_string();
        let cmt = self.cmt.to_string();
        let new_thread_comment = NewThreadComment::new(thd_id,cmt_no,cmt_name,cmt);
        diesel::insert_into(thread_comment).values(new_thread_comment).execute(conn).expect("Insert Error Thread");
    }

    //チェック処理
    pub fn validation_comment(&self) -> Vec<String>{
        let mut error_msg:Vec<String> = Vec::new();
        let error_msg_struct = ErrorMsg{};
        let mut error_key: String;

        //必須項目チェック
        if self.cmt_name == String::from(""){
            error_key = String::from("EM_0001");
            error_msg.push(error_msg_struct.get_error_msg_by_place_holder(error_key,String::from("ネーム")));
        }
        if self.cmt == String::from(""){
            error_key = String::from("EM_0001");
            error_msg.push(error_msg_struct.get_error_msg_by_place_holder(error_key,String::from("コメント")));
        }

        return error_msg;
    }
}

impl ThreadDeleteParams{
    //コメントのレコードをデリート
    pub fn remove_comment(&self,conn: &PooledConnection<ConnectionManager<SqliteConnection>>){
        diesel::delete(thread_comment.filter(thread_id.eq(self.thd_id))).execute(conn).expect("Delete Error Comment");
    }
}

