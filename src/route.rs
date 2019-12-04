use crate::api;
use actix_web::{web, Scope};

/// Routing definition; 路由定义
pub fn route() -> Scope {
    web::scope("/rust_web")
        .service(
            web::scope("/user") // user api
                .route("/list", web::get().to_async(api::user_api::list)) // query
                .route("/save", web::post().to_async(api::user_api::save)), // save
        )
        .service(
            // scope demo
            web::scope("/user2") // user api
                .route("/list", web::get().to_async(api::user_api::list)) // query
                .route("/save", web::post().to_async(api::user_api::save)), // save
        )
}
