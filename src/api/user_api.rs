use actix_web::{HttpRequest, HttpResponse, web::Data};
use diesel::{Connection};
use qstring::QString;

use crate::DataBase;
use crate::model::user::user_select2;

pub fn list(db: Data<DataBase>, req: HttpRequest) -> HttpResponse {
    println!("/user_list");
    //获取参数
    let qs = QString::from(req.query_string());
    let limit: i64 = qs.get("limit").unwrap_or("100").parse().unwrap_or(100);
    let offset: i64 = qs.get("offset").unwrap_or("0").parse().unwrap_or(0);
    let conn = db.pool.get().unwrap();

    // 新的基于事务的代码
    /*let user_list = conn.transaction(|| {
        user_select2(&conn, 0, 100).map(|list| {
            println!("user_select2");
            Ok(list)
        })?
    }).unwrap();
    HttpResponse::Ok().content_type("text/html;charset=utf-8").body(format!(r#"{:?}"#, user_list))*/

    // 旧的代码,可以正常运行
    db.user_select(offset, limit).map(|list| {
        println!("db.query_users");
        HttpResponse::Ok().content_type("text/html;charset=utf-8").body(format!(r#"{:?}"#, list))
    }).unwrap_or_else(|err| {
        HttpResponse::BadGateway().content_type("text/html;charset=utf-8").body(format!("server error: {:?}", err))
    })
}


