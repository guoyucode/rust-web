use actix_web::HttpResponse;
use std::fmt::Debug;
use std::error::Error;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Response<DATA>
    where
        DATA: Debug + Serialize,
{
    code: i64,
    msg: Option<String>,
    data: Option<DATA>,
}

#[derive(Debug, Serialize)]
struct ResponsePageData<DATA>
    where
        DATA: Debug + Serialize,
{
    index: i64,
    limit: i64,
    total: i64,
    records: DATA,
}

/// Return the correct value; 返回正确的值
pub fn response_ok<T>(v: T) -> HttpResponse
    where T: Debug + Serialize {
    let res = Response {
        msg: None,
        code: 200,
        data: Some(v),
    };
    HttpResponse::Ok().content_type("appliction/json;charset=utf-8").json2(&res)
}

/// Return the correct value; 返回正确的值
pub fn response_page<T>(records: T, index: i64, limit: i64, total: i64) -> HttpResponse
    where T: Debug + Serialize {
    let page_data = ResponsePageData {
        index,
        limit,
        total,
        records,
    };
    response_ok(page_data)
}

/// Return error; 返回错误
pub fn response_error<E>(err: E) -> HttpResponse
    where E: Error,
{
    let res = Response::<()> {
        msg: Some(err.to_string()),
        code: 500,
        data: None,
    };
    HttpResponse::Ok().content_type("appliction/json;charset=utf-8").json2(&res)
}

/// Return value judgment; 返回值判断
pub fn response_match<V, E>(result: Result<V, E>) -> HttpResponse
    where V: Debug + Serialize,
          E: Error,
{
    match result {
        Ok(v) => response_ok(v),
        Err(e) => response_error(e),
    }
}

pub fn response_page_match<V, E>(result: Result<V, E>, index: i64, limit: i64, count: i64) -> HttpResponse
    where V: Debug + Serialize,
          E: Error,
{
    match result {
        Err(e) => response_error(e),
        Ok(v) => response_page(v, index, limit, count),
    }
}