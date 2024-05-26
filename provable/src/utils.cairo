use super::agreement::Agreement;

struct LineCoefficients {
    a: felt252,
    b: felt252,
}

pub fn aggregate(
    client_public_key: felt252, server_public_key: felt252, agreements: Array<Agreement>
) -> LineCoefficients {
    let mut a = 0;
    let mut b = 0;
    let mut i = 0;
    while i != agreements
        .len() {
            a += *agreements.at(i).quantity;
            b -= *agreements.at(i).quantity * *agreements.at(i).price;
            i += 1;
        };

    LineCoefficients { a, b }
}
