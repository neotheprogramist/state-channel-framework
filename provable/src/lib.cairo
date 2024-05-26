use agreement::Input;
use utils::{aggregate, LineCoefficients};

mod agreement;
mod utils;

fn main(input: Input) -> Array<felt252> {
    let LineCoefficients { a, b } = aggregate(input.client_public_key, input.server_public_key, input.agreements);
    let result = a * input.settlement_price + b;
    array![
        input.client_public_key,
        input.server_public_key,
        input.settlement_price,
        a,
        b,
        result,
    ]
}

#[cfg(test)]
mod tests {
    use super::{main, Input, agreement::Agreement};

    #[test]
    fn empty() {
        let input = Input {
            client_public_key: 0,
            server_public_key: 0,
            agreements: array![],
            settlement_price: 0,
        };

        let result = main(input);

        assert(*result.at(0) == 0, 'invalid client_public_key');
        assert(*result.at(1) == 0, 'invalid server_public_key');
        assert(*result.at(2) == 0, 'invalid settlement_price');
        assert(*result.at(3) == 0, 'invalid a');
        assert(*result.at(4) == 0, 'invalid b');
        assert(*result.at(5) == 0, 'invalid result');
    }

    #[test]
    fn simple() {
        let input = Input {
            client_public_key: 0,
            server_public_key: 0,
            agreements: array![
                Agreement {
                    quantity: 1,
                    nonce: 0,
                    price: 1,
                    server_signature_r: 0,
                    server_signature_s: 0,
                    client_signature_r: 0,
                    client_signature_s: 0,
                }
            ],
            settlement_price: 2,
        };

        let result = main(input);

        assert(*result.at(0) == 0, 'invalid client_public_key');
        assert(*result.at(1) == 0, 'invalid server_public_key');
        assert(*result.at(2) == 2, 'invalid settlement_price');
        assert(*result.at(3) == 1, 'invalid a');
        assert(*result.at(4) == -1, 'invalid b');
        assert(*result.at(5) == 1, 'invalid result');
    }
}
