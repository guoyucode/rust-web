#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;

use actix_files as fs;
use actix_web::{middleware, App, HttpServer, web};

mod model;
mod api;
mod util;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=debug,actix_web=debug");
    env_logger::init();

    HttpServer::new(move||{
        App::new()
            .data(util::DataBase::new("jygo.db")) // 数据库
            .wrap(middleware::Logger::default()) // 日志
            .wrap(middleware::DefaultHeaders::new().header("content-type", "appliction/json;charset=utf-8"))
            .service(web::resource("/user_list").route(web::get().to(api::user_api::list))) // 查询用户
            .service(web::resource("/user_save").route(web::post().to(api::user_api::save))) // 查询用户
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    .bind("0.0.0.0:8080").expect("端口可能被占用了!")
    .run()
}
