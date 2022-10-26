use crate::models::{Thread,NewThread,ThreadCreateParams,ThreadSearchParams,ThreadDeleteParams};
use diesel::prelude::*;
use crate::schema::thread::dsl::*;
use std::vec::Vec;
use diesel::r2d2::{ConnectionManager};
use r2d2::PooledConnection;
use diesel::SqliteConnection;
use crate::error_msg::{ErrorMsg,GetErrorMsg};

impl Thread{
    //Threadのリストを全取得
    pub fn select_all_thred(conn: &PooledConnection<ConnectionManager<SqliteConnection>>) -> Vec<Thread>{
        let thread_list = thread.load::<Thread>(conn).expect("Error loading Thread");
        return thread_list;
    }
}

impl NewThread{
    //NewThreadインスタンスの作成
    fn new(thd_id: i32,thd_name: String) -> NewThread{
        NewThread{
            thread_id: thd_id,
            thread_name: thd_name,
        }
    }
}

impl ThreadSearchParams{
    //ThreadSearchParamsインスタンスの作成
    pub fn new(keyword: String) -> ThreadSearchParams{
        ThreadSearchParams{
            search_keyword: keyword,
        }
    }

    //Threadのリストをネームで取得
    pub fn select_thred_name(&self,conn: &PooledConnection<ConnectionManager<SqliteConnection>>) -> Vec<Thread>{
        let format = format!("%{}%", self.search_keyword);
        let thread_list = thread.filter(thread_name.like(format)).load::<Thread>(conn).expect("Error loading Thread");
        return thread_list;
    }
}

impl ThreadCreateParams{
    //ThreadCreateParamsインスタンスの作成
    pub fn new(thd_name: String) -> ThreadCreateParams{
        ThreadCreateParams{
            thd_name: thd_name,
        }
    }

    //Threadのレコードをインサート
    pub fn insert_thread(&self,conn: &PooledConnection<ConnectionManager<SqliteConnection>>){
        let thread_list = Thread::select_all_thred(&conn);

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
        
        let thd_name = self.thd_name.to_string();
        let new_thread = NewThread::new(thd_id, thd_name);
        diesel::insert_into(thread).values(new_thread).execute(conn).expect("Insert Error Thread");
    }

    //チェック処理
    pub fn validation_thread(&self) -> Vec<String>{
        let mut error_msg:Vec<String> = Vec::new();
    
        //必須項目チェック
        if self.thd_name == String::from(""){
            let error_msg_struct = ErrorMsg{};
            let error_key = String::from("EM_0001");
            error_msg.push(error_msg_struct.get_error_msg_by_place_holder(error_key,String::from("スレッドネーム")));
    
        }
    
        return error_msg;
    }
}

impl ThreadDeleteParams{
    //ThreadDeleteParamsインスタンスの作成
    pub fn new(thd_id: i32,thd_name: String) -> ThreadDeleteParams{
        ThreadDeleteParams{
            thd_id: thd_id,
            thd_name: thd_name,
        }
    }

    //Threadのレコードをデリート
    pub fn remove_thread(&self,conn: &PooledConnection<ConnectionManager<SqliteConnection>>) -> String{
        let thd_id = self.thd_id;
        let thd_name = &self.thd_name;
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
}