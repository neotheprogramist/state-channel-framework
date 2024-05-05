from input import Agreement

func aggregate(agreements_len: felt, agreements: Agreement**, a: felt, b: felt) -> (a: felt, b: felt) {
    if (agreements_len == 0) {
        return (a=a, b=b);
    }

    return aggregate(
        agreements_len - 1,
        agreements + 1,
        a + agreements[0].quantity,
        b - agreements[0].quantity * agreements[0].price
    );
}
