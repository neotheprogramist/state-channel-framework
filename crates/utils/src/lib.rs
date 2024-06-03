pub mod account;
pub mod args;
pub mod client;
pub mod declare;
pub mod deploy;
pub mod invoke;
pub mod models;
pub mod receipt;
pub mod runner_error;
pub mod server;
pub mod shutdown;
pub mod sncast;
pub fn add(left: usize, right: usize) -> usize {
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
