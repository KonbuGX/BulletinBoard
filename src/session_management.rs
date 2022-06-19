use std::ops::DerefMut;

//redisにセッションをセット
pub fn set_session(redis_conn: &mut r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>, key: &str, value: &i32){
    r2d2_redis::redis::cmd("SET").arg(key).arg(value.to_string()).execute(redis_conn.deref_mut());
}

//redisにセッションをセット
pub fn get_session(redis_conn: &mut r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>, key: &str) -> i32{
    let mut acct_no: i32 = 0;
    if r2d2_redis::redis::Value::Nil != r2d2_redis::redis::cmd("GET").arg(key).query(redis_conn.deref_mut()).unwrap(){
        acct_no = r2d2_redis::redis::cmd("GET").arg(key).query(redis_conn.deref_mut()).unwrap();
    }
    return acct_no;
}

pub fn delete_session(redis_conn: &mut r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>, key: &str){
    r2d2_redis::redis::cmd("DEL").arg(key).execute(redis_conn.deref_mut());
}