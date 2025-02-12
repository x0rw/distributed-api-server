use crate::http_handler::HttpRequest;


pub struct HttpBuilder{
    raw_http: String,
    handler : HttpRequest,
}
impl HttpBuilder{
   fn new()-> Self{
        Self{
            raw_http: String::new()
        }
   } 
   fn set_handler(mut self, handler: HttpRequest) -> Self{
    self.handler = handler;
    self
   }

    pub fn build(route: &RouteType, handler: HttpRequest, router: &RoutesMap) -> HttpBuilder {
}
