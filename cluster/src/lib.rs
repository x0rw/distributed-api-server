//pub mod async_node;
pub mod cli;
pub mod gateway;
pub mod health;
pub mod node;
pub mod service;
pub mod sync_node;
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
