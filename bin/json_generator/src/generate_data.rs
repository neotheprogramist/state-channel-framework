use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::thread_rng;
/// Generates two vectors of 50 prices each, ranging from 1500 to 2000, with the same sums but shuffled differently.
pub fn generate_identical_but_shuffled_prices(agreements_count: u64) -> (Vec<i32>, Vec<i32>) {
    let mut rng = thread_rng();
    let price_range = 1500..=2000;
    let range_distribution = Uniform::from(price_range);
    let prices: Vec<i32> = (0..agreements_count)
        .map(|_| range_distribution.sample(&mut rng))
        .collect();

    let mut shuffled_prices = prices.clone();
    shuffled_prices.shuffle(&mut rng);

    (prices, shuffled_prices)
}
