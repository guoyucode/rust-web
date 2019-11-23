use actix_web::{HttpResponse};
use serde::export::fmt::Debug;

#[derive(Debug, Serialize)]
pub struct Response<DATA>
where
    DATA: Debug + serde::Serialize,
{
    msg: String,
    code: i64,
    data: DATA,
}

/// Return the correct value; 返回正确的值
pub fn response_ok<T>(v: T) -> HttpResponse
    where T: std::fmt::Debug + serde::Serialize {
    let res = Response {
        msg: "".to_string(),
        code: 200,
        data: v,
    };
    HttpResponse::Ok().content_type("appliction/json;charset=utf-8").json2(&res)
}

/// Return error; 返回错误
pub fn response_error<T>(err: T) -> HttpResponse
    where T: std::fmt::Debug + std::string::ToString,
{
    let res = Response {
        msg: err.to_string(),
        code: 500,
        data: (),
    };
    HttpResponse::Ok().content_type("appliction/json;charset=utf-8").json2(&res)
}

/// Return value judgment; 返回值判断
pub fn response_match<V, E>(result: Result<V, E>) -> HttpResponse
    where V: std::fmt::Debug + serde::Serialize,
          E: std::fmt::Debug + std::string::ToString,
{
    match result {
        Ok(v) => response_ok(v),
        Err(e) => response_error(e),
    }
}