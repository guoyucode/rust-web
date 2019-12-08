#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;

use actix_files as fs;
use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::{http, middleware, App, HttpServer};

mod api;
mod model;
mod route;
mod util;

use actix_web::dev::Service;
use futures::future::Future;
use std::borrow::BorrowMut;
use util::{database_deps, web_deps};

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=debug,actix_web=debug");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .data(util::DataBase::new("jygo.db")) // 数据库
            .wrap_fn(|req, srv| {
                println!("Hi from start. You requested: {}", req.path());
                let token = req.headers().get("token");
                let token = match token {
                    Some(v) => {
                        if v.is_empty() {
                            ""
                        } else {
                            v.to_str().unwrap_or("")
                        }
                    }
                    None => "",
                };
                println!("token: {}", token);
                srv.call(req).map(|res| {
                    println!("Hi from response");
                    /* res.map_body(|h, b| b);
                    if token == "" {
                        let error1 = actix_web::error::ErrorBadGateway("not token");
                        return res.error_response(error1);
                    }*/
                    return res;
                })
            })
            .wrap(ErrorHandlers::new().handler(
                http::StatusCode::from_u16(400).unwrap(),
                api::error_handler::render_400,
            ))
            .wrap(ErrorHandlers::new().handler(
                http::StatusCode::from_u16(400).unwrap(),
                api::error_handler::render_400,
            ))
            .wrap(middleware::Logger::default()) // 日志
            .wrap(
                middleware::DefaultHeaders::new()
                    .header("content-type", "appliction/json;charset=utf-8"),
            )
            .service(route::route())
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    .workers(3)
    .bind("127.0.0.1:8080")
    .expect("端口可能被占用了!")
    .run()
}
