use actix_web::{web};
use crate::controller::{*};
use actix_files::Files;

//画面遷移を行う関数をルートとしてまとめる
pub fn routes(srv_cfg: &mut web::ServiceConfig){
    srv_cfg
    .service(index)
    .service(login)
    .service(login_account)
    .service(signout)
    .service(signup)
    .service(signup_account)
    .service(delete_account)
    .service(add_thread)
    .service(search_thread)
    .service(thread_comment)
    .service(add_thread_comment)
    .service(mypage)
    .service(account)
    .service(edit_account)
    .service(password)
    .service(edit_password)
    .service(delete_thread_list)
    .service(delete_thread)
    .service(search_thread_mypage)
    .service(Files::new("/public", "./public").show_files_listing());
}