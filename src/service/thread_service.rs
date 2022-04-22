use actix_web::{web};
use crate::models::{Thread,NewThread,AddTreadParams};
use diesel::prelude::*;
use crate::schema::thread::dsl::*;
use std::vec::Vec;
use diesel::r2d2::{ConnectionManager};
use r2d2::PooledConnection;
use diesel::SqliteConnection;

//Threadのリストを全取得
pub fn select_all_thred(conn: &PooledConnection<ConnectionManager<SqliteConnection>>) -> Vec<Thread>{
    let thread_list = thread.load::<Thread>(conn).expect("Error loading Thread");
    return thread_list;
}

//Threadのレコードをインサート
pub fn insert_thread(name: String,conn: PooledConnection<ConnectionManager<SqliteConnection>>){
    let thread_list = select_all_thred(&conn);

    //threadIdの付番処理
    let mut id:i32 = 0;
    if thread_list.len() > 0 {
        for temp_thread in &thread_list {
            if temp_thread.thread_id > id {
                id = temp_thread.thread_id;
            }  
        }
        id += 1;
    }else {
        id = 1;
    }
    
    let new_thread = NewThread{thread_id:id,thread_name:name};
    diesel::insert_into(thread).values(new_thread).execute(&conn).expect("Insert Error Thread");
}

//Threadのレコードをデリート
pub fn remove_thread(id: i32,name: String,conn: &PooledConnection<ConnectionManager<SqliteConnection>>) -> String{
    let thread_list = thread.filter(thread_id.eq(id)).filter(thread_name.eq(name)).load::<Thread>(conn).expect("Error loading Thread");

    //削除対象のThredのレコードが更新されたり既に削除された場合に再度スレッドの確認を促す。
    if thread_list.len() > 0 {
        diesel::delete(thread.filter(thread_id.eq(id))).execute(conn).expect("Delete Error Thread");
        return String::from("");
    } else {
        return String::from("スレッドが更新されております。ご確認下さい。");
    }
}

//チェック処理
pub fn validation_thread(params: &web::Form<AddTreadParams>) -> Vec<String>{
    let mut error_msg:Vec<String> = Vec::new();

    //必須項目チェック
    if params.text.clone() == String::from(""){
        error_msg.push(String::from("スレッド名が未入力です。"));
    }

    return error_msg;
}