pub mod auth;
pub mod controller;
pub mod error;
pub mod http;
pub mod routes;
pub mod utils;
use cluster;
#[cfg(test)]
fn add(a: u32, b: u32) -> u32 {
    32
}
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
