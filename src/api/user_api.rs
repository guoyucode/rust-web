use crate::web_deps::*;
use crate::model::user;

pub async fn list(db: Data<DataBase>, req: HttpRequest) -> HttpResponse {
    println!("/list");
    let qs = QString::from(req.query_string());
    let limit: i64 = qs.get("limit").unwrap_or("100").parse().unwrap_or(100);
    let offset: i64 = qs.get("offset").unwrap_or("0").parse().unwrap_or(0);
    let conn = db.get_conn();
    // Transaction-based code; 基于事务的代码
    let result= conn.transaction(|| {
        user::select(&conn, offset, limit)
    });
    super::response_match(result)
}

pub async fn save(db: Data<DataBase>, u: Json<user::User>) -> HttpResponse {
    println!("/save");
    let conn = db.get_conn();
    // Transaction-based code; 基于事务的代码
    let result= conn.transaction(|| {
        user::insert(&conn, &u)
    });
    super::response_match(result)
}


