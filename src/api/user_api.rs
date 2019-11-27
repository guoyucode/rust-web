use crate::web_deps::*;
use crate::database_deps::*;
use crate::model::user;
use std::ops::Deref;
use diesel::result::Error;

pub async fn list(db: Data<DataBase>, req: Request) -> Response {
    println!("/list");
    let qs = QString::from(req.query_string());
    let limit: i64 = qs.get("limit").unwrap_or("100").parse().unwrap_or(100);
    let offset: i64 = qs.get("offset").unwrap_or("0").parse().unwrap_or(0);
    let keyword = qs.get("keyword").unwrap_or("");
    let conn = db.get_conn();

    let mut query = user::t_user.into_boxed(); // 构建查询表达式
    if  keyword != "" { query = query.filter(user::name.eq(keyword));}
    if  keyword != "" { query = query.order(user::id.asc());}

    let result = query
        .offset(offset)
        .limit(limit)
        .load::<user::User>(&conn);
    super::response_match(result)
}

pub async fn save(db: Data<DataBase>, u: Json<user::User>) -> Response {
    println!("/save");
    let conn = db.get_conn();
    // Transaction-based code; 基于事务的代码
    let result = conn.transaction(|| {
        insert_into(user::t_user)
            .values(u.deref())
            .execute(&conn)
            .and_then(|update_row| {
                if update_row > 0 {
                    Ok(u.clone())
                } else {
                    Err(Error::RollbackTransaction)
                }
            })
    });
    super::response_match(result)
}


