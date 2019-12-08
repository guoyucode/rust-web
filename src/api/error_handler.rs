use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::web::BytesMut;
use actix_web::{dev, http, HttpResponse, Responder, ResponseError, Result};
use std::ops::Deref;
/*pub fn render_500<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let res_mut = res.response_mut();
    let mut buf = BytesMut::new();
    let _ = write!(&mut buf, "{}", res_mut);
    let encoded = serialize::json::encode(&object).unwrap();
    res_mut.set_body(Body::from(buf));

    Ok(ErrorHandlerResponse::Response(res))
}*/

/*fn render_500<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("0001"),
    );
    let x = res.response_mut().deref();

    x.x.res.Ok(ErrorHandlerResponse::Response(res))
}*/

pub fn render_400<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(res))
}
