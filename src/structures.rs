use actix_web::{
    body::BoxBody, HttpRequest, Responder,
    http::{header::ContentType},
    HttpResponse,
};

use serde::Serialize;


//structura para devolver la respuesta de username
#[derive(Serialize)]
pub struct UsernameHtmlObj {
    username: &'static str,
}

// Responder
impl Responder for UsernameHtmlObj {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}