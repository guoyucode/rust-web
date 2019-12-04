use crate::database_deps::*;
use crate::model;
use crate::model::user;
use crate::web_deps::*;
use diesel::result::Error;
use std::ops::Deref;

///     let qs = QString::from(req.query_string());
///    let limit: i64 = qs.get("limit").unwrap_or("100").parse().unwrap_or(100);
///    let offset: i64 = qs.get("offset").unwrap_or("0").parse().unwrap_or(0);
///    let keyword = qs.get("keyword").unwrap_or("");
/// 参数非必填
pub fn list(db: Data<DataBase>, params: Query<model::CommonFormParam>) -> Response {
    println!("/list");

    let index = params.index.to_owned().unwrap_or(1);
    let limit = params.limit.to_owned().unwrap_or(100);
    let keyword = params.keyword.to_owned();
    let offset = params.offset().to_owned();

    let conn = db.get_conn();

    let query = || {
        let mut q = user::t_user.into_boxed(); // 构建查询表达式
        if keyword != "" {
            q = q.filter(user::name.eq(keyword.clone()));
        }
        q = q.order(user::id.asc());
        q
    };

    let count_result = query().count().get_result::<i64>(&conn);
    let total = count_result.unwrap_or(-1);

    let result = query()
        .offset(offset)
        .limit(limit)
        .load::<user::User>(&conn);

    super::response_page_match(result, index, limit, total)
}

pub fn save(db: Data<DataBase>, u: Json<user::User>) -> Response {
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
