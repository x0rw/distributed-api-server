
#[derive(Debug, PartialEq, Eq)]
pub enum HTTP_METHOD{
    POST,
    GET,
}
#[derive(Debug)]
pub struct http_req{
    pub uri: String,
    pub method: HTTP_METHOD,
}
impl http_req{
    fn new(method: HTTP_METHOD, uri: &str)->Self{
        Self{
            uri: String::from(uri),
            method: method,
        }
    }
}
pub fn handle_http(line:&str)-> http_req{
   let mut words = line.split_whitespace();

   if words.clone().count()!= 3 {
    panic!("unvalid http header size");
    }

   match words.next(){
        Some("GET") => {
            http_req::new(HTTP_METHOD::GET, words.next().unwrap())
        },
        Some("POST") => {
            http_req::new(HTTP_METHOD::POST, words.next().unwrap())
        },
        _ => panic!("Unvalid Http Methode"),
   }
}



#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn valid_http_get_request(){
        let http_header = String::from("GET / HTTP/1.1");
        let http_h = handle_http(&http_header);
        
        assert_eq!(http_h.uri, "/");
        assert_eq!(http_h.method, HTTP_METHOD::GET);
    }

    #[test]
    fn valid_http_post_request(){
        let http_header = String::from("POST / HTTP/1.1");
        let http_h = handle_http(&http_header);
        
        assert_eq!(http_h.uri, "/");
        assert_eq!(http_h.method, HTTP_METHOD::POST);
    }
    
    #[test]
    #[should_panic]
    fn unvalid_http_method_request(){
        let http_header = String::from("HACK / HTTP/1.1");
        let http_h = handle_http(&http_header);
    }

    #[test]
    #[should_panic]
    fn unvalid_header_size(){
        let http_header = String::from("POST / HTTP/1.1 HELLO");
        let http_h = handle_http(&http_header);
    }
}
