use crate::{handler::HttpRequest, routes::RouteType};
pub enum StatusCode {
    Ok200,
    NotFound404,
}
pub enum Response {
    JSON(String, StatusCode),
    HTML(String, StatusCode),
    TEXT(String, StatusCode),
}
impl Response {
    fn process(self) -> String {
        match self {
            Response::JSON(data, status) => {
                let mut response = status.to_string();
                let response = format!("{response}\r\nContent-Length : {}\r\nContent-Type : application/json\r\n\r\n{data}",
                    data.len()
                );
                //println!("{response}");
                response
            }
            Response::TEXT(data, status) => data,
            Response::HTML(data, status) => data,
        }
    }
}
impl ToString for StatusCode {
    fn to_string(&self) -> String {
        match self {
            StatusCode::Ok200 => "HTTP/1.1 200 OK".to_string(),
            StatusCode::NotFound404 => "HTTP/1.1 404 Not Found".to_string(),
        }
    }
}
pub struct HttpBuilder<'a> {
    raw_http: String,
    handler: HttpRequest,
    route: &'a RouteType,
}

impl<'a> HttpBuilder<'a> {
    pub fn new(handler: HttpRequest, route: &'a RouteType) -> Self {
        Self {
            raw_http: String::new(),
            handler: handler,
            route: route,
        }
    }

    pub fn build(mut self) -> String {
        match self.route {
            RouteType::Controller(FnController) => FnController(self.handler.data).process(),
            _ => "not implemented yet".to_string(),
        }
    }

    pub fn build_badrequest() -> String {
        format!("HTTP/1.1 400 Bad Request\r\n").to_string()
    }
}
