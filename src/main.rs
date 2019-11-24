#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;

use actix_files as fs;
use actix_web::{middleware, App, HttpServer, web, Scope};

mod model;
mod api;
mod util;

use util::{web_deps, database_deps};

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=debug,actix_web=debug");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .data(util::DataBase::new("jygo.db")) // 数据库
            .wrap(middleware::Logger::default()) // 日志
            .wrap(middleware::DefaultHeaders::new().header("content-type", "appliction/json;charset=utf-8"))
            .service(route())
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
        .bind("0.0.0.0:8080").expect("端口可能被占用了!")
        .run()
}

/// Routing definition; 路由定义
fn route() -> Scope {
    web::scope("/rust_web")
        .service(
            web::scope("/user") // user api
                .route("/list", web::get().to(api::user_api::list)) // query
                .route("/save", web::post().to(api::user_api::save)) // save
        )
        .service( // scope demo
                  web::scope("/user2") // user api
                      .route("/list", web::get().to(api::user_api::list)) // query
                      .route("/save", web::post().to(api::user_api::save)) // save
        )
}
