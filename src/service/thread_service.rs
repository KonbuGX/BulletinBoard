use actix_web::{web};
use crate::models::{Thread,NewThread,AddTreadParams};
use diesel::prelude::*;
use crate::schema::thread::dsl::*;
use std::vec::Vec;
use diesel::r2d2::{ConnectionManager};
use r2d2::PooledConnection;
use diesel::SqliteConnection;
use crate::error_msg::{ErrorMsg,GetErrorMsg};

//Threadのリストを全取得
pub fn select_all_thred(conn: &PooledConnection<ConnectionManager<SqliteConnection>>) -> Vec<Thread>{
    let thread_list = thread.load::<Thread>(conn).expect("Error loading Thread");
    return thread_list;
}

//Threadのリストをネームで取得
pub fn select_thred_name(search_keyword: &String,conn: &PooledConnection<ConnectionManager<SqliteConnection>>) -> Vec<Thread>{
    let format = format!("%{}%", search_keyword);
    let thread_list = thread.filter(thread_name.like(format)).load::<Thread>(conn).expect("Error loading Thread");
    return thread_list;
}

//Threadのレコードをインサート
pub fn insert_thread(thd_name: String,conn: &PooledConnection<ConnectionManager<SqliteConnection>>){
    let thread_list = select_all_thred(&conn);

    //threadIdの付番処理
    let mut thd_id:i32 = 0;
    if thread_list.len() > 0 {
        for temp_thread in &thread_list {
            if temp_thread.thread_id > thd_id {
                thd_id = temp_thread.thread_id;
            }  
        }
        thd_id += 1;
    }else {
        thd_id = 1;
    }
    
    let new_thread = NewThread{thread_id:thd_id,thread_name:thd_name};
    diesel::insert_into(thread).values(new_thread).execute(conn).expect("Insert Error Thread");
}

//Threadのレコードをデリート
pub fn remove_thread(thd_id: i32,thd_name: String,conn: &PooledConnection<ConnectionManager<SqliteConnection>>) -> String{
    let thread_list = thread.filter(thread_id.eq(thd_id)).filter(thread_name.eq(thd_name)).load::<Thread>(conn).expect("Error loading Thread");

    //削除対象のThredのレコードが更新されたり既に削除された場合に再度スレッドの確認を促す。
    if thread_list.len() > 0 {
        diesel::delete(thread.filter(thread_id.eq(thd_id))).execute(conn).expect("Delete Error Thread");
        return String::from("");
    } else {
        let error_msg_struct = ErrorMsg{};
        let error_key = String::from("EM_THD_0001");
        return error_msg_struct.get_error_msg(error_key);
    }
}

//チェック処理
pub fn validation_thread(params: &web::Form<AddTreadParams>) -> Vec<String>{
    let mut error_msg:Vec<String> = Vec::new();

    //必須項目チェック
    if params.thd_name.clone() == String::from(""){
        let error_msg_struct = ErrorMsg{};
        let error_key = String::from("EM_0001");
        error_msg.push(error_msg_struct.get_error_msg_by_place_holder(error_key,String::from("スレッドネーム")));

    }

    return error_msg;
}