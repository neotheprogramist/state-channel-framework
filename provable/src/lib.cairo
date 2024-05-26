use utils::fib;

mod utils;

fn main(n: felt252) -> felt252 {
    fib(16)
}

#[cfg(test)]
mod tests {
    use super::fib;

    #[test]
    fn it_works() {
        assert(fib(16) == 987, 'it works!');
    }
}
