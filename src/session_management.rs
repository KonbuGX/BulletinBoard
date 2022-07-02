use std::ops::DerefMut;
use std::collections::HashMap;
use once_cell::sync::OnceCell;

//redisにセッションをセット
pub fn set_session(redis_conn: &mut r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>,session: &OnceCell<String>, account_info: HashMap<&String, String>){
    let session_id = session.get().unwrap();
    for (key,value) in account_info {
        r2d2_redis::redis::cmd("HSET").arg(session_id).arg(key).arg(value).execute(redis_conn.deref_mut());
    }
}

//redisからセッションを取得
pub fn get_session(redis_conn: &mut r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>, session: &OnceCell<String>) -> HashMap<String, String>{
    let mut session_id: &String = &String::from("");
    if let Some(v) = session.get() {
        session_id = v;
    }

    let mut acct_info: HashMap<String, String> = HashMap::new();
    if (r2d2_redis::redis::Value::Nil != r2d2_redis::redis::cmd("HGETALL").arg(session_id).query(redis_conn.deref_mut()).unwrap()) |
       (session_id != &String::from("")){
        acct_info = r2d2_redis::redis::cmd("HGETALL").arg(session_id).query(redis_conn.deref_mut()).unwrap();
    }else {
        //ログインしていない状態を表す。
        let acct_no = 9999;
        let acct_name = String::from("名無し");
        acct_info.insert(String::from("acct_no"), acct_no.to_string());
        acct_info.insert(String::from("acct_name"), acct_name);
    }
    return acct_info;
}

//redisのセッションを削除
pub fn delete_session(redis_conn: &mut r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>, session: &OnceCell<String>, account_info_keys: Vec<&std::string::String>){
    let session_id = session.get().unwrap();
    for key in account_info_keys {
        r2d2_redis::redis::cmd("HDEL").arg(session_id).arg(key).execute(redis_conn.deref_mut());
    }
}