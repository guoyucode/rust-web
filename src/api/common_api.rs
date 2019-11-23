use actix_web::{HttpResponse};

/// Return the correct value; 返回正确的值
pub fn response_ok<T>(v: T) -> HttpResponse
    where T: std::fmt::Debug {
    HttpResponse::Ok().content_type("text/html;charset=utf-8").body(format!(r#"{:?}"#, v))
}

/// Return error; 返回错误
pub fn response_error<T>(err: T) -> HttpResponse
    where T: std::fmt::Debug {
    HttpResponse::BadGateway().content_type("text/html;charset=utf-8").body(format!(r#"{:?}"#, err))
}

/// Return value judgment; 返回值判断
pub fn response_match<V, E>(result: Result<V, E>) -> HttpResponse
    where V: std::fmt::Debug,
          E: std::fmt::Debug,
{
    match result {
        Ok(v) => response_ok(v),
        Err(e) => response_error(e),
    }
}