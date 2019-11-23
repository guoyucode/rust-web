use actix_web::{HttpRequest, HttpResponse, web::Data};
use diesel::{Connection};
use qstring::QString;

use crate::DataBase;
use crate::model::user::user_select;

pub async fn list(db: Data<DataBase>, req: HttpRequest) -> HttpResponse {
    println!("/user_list");
    //获取参数
    let qs = QString::from(req.query_string());
    let limit: i64 = qs.get("limit").unwrap_or("100").parse().unwrap_or(100);
    let offset: i64 = qs.get("offset").unwrap_or("0").parse().unwrap_or(0);
    let conn = db.pool.get().unwrap();
    // Transaction-based code; 基于事务的代码
    let result= conn.transaction(|| {
        user_select(&conn, offset, limit)
    });
    super::response_match(result)
}


