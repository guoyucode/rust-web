#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;

use actix_files as fs;
use actix_web::{middleware, App, HttpServer};

mod model; /// 实体
mod api; /// 接口
mod util; /// 工具
mod route; /// 路由

use util::{web_deps, database_deps};

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=debug,actix_web=debug");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .data(util::DataBase::new("jygo.db")) // 数据库
            .wrap(middleware::Logger::default()) // 日志
            .wrap(middleware::DefaultHeaders::new().header("content-type", "appliction/json;charset=utf-8"))
            .service(route::route())
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
        .workers(3)
        .bind("0.0.0.0:8080").expect("端口可能被占用了!")
        .run()
}
