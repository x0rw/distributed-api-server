use std::collections::HashMap;

use crate::http_handler::{self, Data};

pub fn parse_params(url: &str) -> (&str, Data) {
    let Some((uri, res)) = url.split_once("?") else {
        return (url, Data::Params(None));
    };
    println!("{url}kkk");
    let m = res
        .split("&")
        .filter_map(|x| x.split_once("="))
        .collect::<HashMap<_, _>>();
    return (uri, Data::Params(Some(m)));
}
#[cfg(test)]
mod tests {
    use std::arch::x86_64::_mm_getcsr;

    use super::*;
    #[test]
    fn normal_test() {
        let url = "/article?id=43&sort=true";
        let pp = parse_params(url);
        assert_eq!(pp.0, "/article");
        if let Data::Params(Some(e)) = pp.1 {
            let id = e.get("id").unwrap();
            let sort = e.get("sort").unwrap();
            assert_eq!(id.parse::<String>().unwrap(), "43".to_string());
            assert_eq!(sort.parse::<String>().unwrap(), "true".to_string());
        };
    }
    #[test]
    fn manipulated_uri_test() {
        let url = "/article?id==43&sort=true=;'";
        let pp = parse_params(url);
        assert_eq!(pp.0, "/article");
        if let Data::Params(Some(e)) = pp.1 {
            let id = e.get("id").unwrap();
            let sort = e.get("sort").unwrap();
            assert_eq!(id.parse::<String>().unwrap(), "=43".to_string());
            assert_eq!(sort.parse::<String>().unwrap(), "true=;'".to_string());
        };
    }
    #[test]
    fn manipulated_chaos_uri_test() {
        let url = "/art?ds=dd&&?&&&&dsdsd=&&&";
        let pp = parse_params(url);
        assert_eq!(pp.0, "/art");
        if let Data::Params(Some(e)) = pp.1 {
            let id = e.get("ds").unwrap();
            let sort = e.get("dsdsd").unwrap();
            assert_eq!(id.parse::<String>().unwrap(), "dd".to_string());
            assert_eq!(sort.parse::<String>().unwrap(), "".to_string());
        };
    }
}
