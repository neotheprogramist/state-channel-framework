#[cfg(test)]
mod tests {
    const URL_ACCEPT_CONTRACT: &str = "/acceptContract";
    const URL_REQUEST_QUOTE: &str = "/requestQuoteWithPrice";
    const URL_REQUEST_SETTLEMENT_PROOF: &str = "/requestSettlementProofWithPrice";
    use crate::requests::{create_agreement, request_settlement_proof_with_price};
    use axum::Router;
    use server::request::account::MockAccount;
    use server::request::models::AppState;
    use surrealdb::engine::local::Mem;
    use surrealdb::Surreal;
    #[tokio::test]
    async fn test_main_simple() -> Result<(), Box<dyn std::error::Error>> {
        let address = "test_case";
        let db = Surreal::new::<Mem>(())
            .await
            .expect("Failed to initialize the database");
        let _ = db.use_ns("test").use_db("test").await;

        let mock_account = MockAccount::new();
        let state: AppState = AppState { db, mock_account };

        let router: Router = server::request::router(&state);

        create_agreement(
            1,
            1000,
            address,
            URL_REQUEST_QUOTE,
            URL_ACCEPT_CONTRACT,
            router.clone(),
        )
        .await?;

        let quantity: i64 = 2;
        let buying_price = 1000;
        create_agreement(
            quantity,
            buying_price,
            address,
            URL_REQUEST_QUOTE,
            URL_ACCEPT_CONTRACT,
            router.clone(),
        )
        .await?;

        let quantity: i64 = -2;
        let buying_price = 1000;
        create_agreement(
            quantity,
            buying_price,
            address,
            URL_REQUEST_QUOTE,
            URL_ACCEPT_CONTRACT,
            router.clone(),
        )
        .await?;

        let quantity: i64 = -1;
        let buying_price = 1000;
        create_agreement(
            quantity,
            buying_price,
            address,
            URL_REQUEST_QUOTE,
            URL_ACCEPT_CONTRACT,
            router.clone(),
        )
        .await?;

        let settlement_price: i64 = 1500;
        // Request settlement
        let settlement_proof = request_settlement_proof_with_price(
            URL_REQUEST_SETTLEMENT_PROOF,
            &address.to_string(),
            settlement_price,
            router.clone(),
        )
        .await?;

        let expected_gain = 0;
        assert_eq!(settlement_proof.diff, expected_gain);

        Ok(())
    }

    #[tokio::test]
    async fn test_idroo() -> Result<(), Box<dyn std::error::Error>> {
        let address = "test_case";
        let db = Surreal::new::<Mem>(())
            .await
            .expect("Failed to initialize the database");
        let _ = db.use_ns("test").use_db("test").await;

        let mock_account = MockAccount::new();
        let state: AppState = AppState { db, mock_account };

        let router: Router = server::request::router(&state);

        let quantity: i64 = 1;
        let buying_price = 1000;
        create_agreement(
            quantity,
            buying_price,
            address,
            URL_REQUEST_QUOTE,
            URL_ACCEPT_CONTRACT,
            router.clone(),
        )
        .await?;

        let quantity: i64 = 1;
        let buying_price = 1100;
        create_agreement(
            quantity,
            buying_price,
            address,
            URL_REQUEST_QUOTE,
            URL_ACCEPT_CONTRACT,
            router.clone(),
        )
        .await?;

        let quantity: i64 = -1;
        let buying_price = 1200;
        create_agreement(
            quantity,
            buying_price,
            address,
            URL_REQUEST_QUOTE,
            URL_ACCEPT_CONTRACT,
            router.clone(),
        )
        .await?;

        let quantity: i64 = -1;
        let buying_price = 1300;
        create_agreement(
            quantity,
            buying_price,
            address,
            URL_REQUEST_QUOTE,
            URL_ACCEPT_CONTRACT,
            router.clone(),
        )
        .await?;

        let settlement_price: i64 = 1500;
        // Request settlement
        let settlement_proof = request_settlement_proof_with_price(
            URL_REQUEST_SETTLEMENT_PROOF,
            &address.to_string(),
            settlement_price,
            router.clone(),
        )
        .await?;

        let expected_gain = 400;
        assert_eq!(settlement_proof.diff, expected_gain);

        Ok(())
    }

    #[tokio::test]
    async fn test_pool() -> Result<(), Box<dyn std::error::Error>> {
        let address = "test_case";
        let db = Surreal::new::<Mem>(())
            .await
            .expect("Failed to initialize the database");
        let _ = db.use_ns("test").use_db("test").await;

        let mock_account = MockAccount::new();
        let state: AppState = AppState { db, mock_account };

        let router: Router = server::request::router(&state);
        let mut buying_prices = vec![1000, 1000, 1000, 1000];
        let mut buy_or_sell = vec![1, 1, -1, -1];

        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();

        while !buying_prices.is_empty() {
            let index = rng.gen_range(0..buying_prices.len());
            let quantity = buy_or_sell.remove(index);
            let buying_price = buying_prices.remove(index);
            let router_copy = router.clone();

            create_agreement(
                quantity,
                buying_price,
                address,
                URL_REQUEST_QUOTE,
                URL_ACCEPT_CONTRACT,
                router_copy,
            )
            .await?;
        }

        let settlement_price: i64 = 1500;
        let settlement_proof = request_settlement_proof_with_price(
            URL_REQUEST_SETTLEMENT_PROOF,
            &address.to_string(),
            settlement_price,
            router,
        )
        .await?;

        let expected_gain = 0;
        assert_eq!(settlement_proof.diff, expected_gain);

        Ok(())
    }
}

#[cfg(test)]
mod prop_testing {
    use crate::requests::{create_agreement, request_settlement_proof_with_price};
    use futures::future::try_join_all;
    use proptest::collection::vec;
    use proptest::prelude::*;
    use proptest::proptest;
    use proptest::strategy::{Just, Strategy};
    use rand::seq::SliceRandom; // Import SliceRandom to use shuffle
    use rand::thread_rng; // Import thread_rng for a random number generator
    use seq_macro::seq;
    use server::request::models::AppState;
    use surrealdb::engine::local::Mem;
    use surrealdb::Surreal;
    //TOO many global rejecs
    use axum::Router;
    const URL_ACCEPT_CONTRACT: &str = "/acceptContract";
    const URL_REQUEST_QUOTE: &str = "/requestQuoteWithPrice";
    const URL_REQUEST_SETTLEMENT_PROOF: &str = "/requestSettlementProofWithPrice";
    use rand_core::OsRng;
    use server::request::account::MockAccount;
    //TEST 50 buys and 50 sells of the same price which should be equall to 0
    fn fixed_composition_strategy() -> BoxedStrategy<Vec<i64>> {
        let ones = vec(Just(1), 50); // Creates a vector of five `1`s
        let minus_ones = vec(Just(-1), 50); // Creates a vector of five `-1`s

        // Combine the two vectors and shuffle
        (ones, minus_ones)
            .prop_map(|(mut ones, minus_ones)| {
                ones.extend(minus_ones);
                ones
            })
            .prop_shuffle()
            .boxed()
    }

    seq!(N in 0..1 {
        proptest! {
                #![proptest_config(ProptestConfig::with_cases(1))]
                #[test]
                fn test_50buys_50sells~N(ops in fixed_composition_strategy()) {
                    let  runtime = tokio::runtime::Runtime::new().unwrap();

                    runtime.block_on(async {
                        let db = Surreal::new::<Mem>(()).await.expect("Failed to initialize the database");
                        let _ = db.use_ns("test").use_db("test").await;

                        let mock_account = MockAccount::new();
                        let state: AppState = AppState { db,mock_account };


                        let router:Router = server::request::router(&state);

                        let address = "test_address".to_string();
                        let futures = ops.into_iter().map(|quantity| {
                            let address = "test_address".to_string();
                            let router_clone =router.clone();
                            async move {
                                let price =1000;
                                create_agreement(quantity, price, &address, URL_REQUEST_QUOTE, URL_ACCEPT_CONTRACT,router_clone).await
                            }
                        }).collect::<Vec<_>>();

                        let results = try_join_all(futures).await;
                        assert!(results.is_ok(), "One or more agreements failed to create successfully.");

                        // Check settlement proof
                        let settlement_price = 1500i64;
                        let settlement_proof = request_settlement_proof_with_price(URL_REQUEST_SETTLEMENT_PROOF, &address, settlement_price, router.clone()).await;
                        assert_eq!(settlement_proof.unwrap().diff, 0, "Expected gain did not match.");
                });

            }
        }
    });

    //Test 100 buys and 0 sells, the expected diff is 100*settlement_price - sum(buying_prices)
    /// Generates a vector of 100 prices, each ranging from 1000 to 1500.
    fn generate_prices_strategy_gain() -> BoxedStrategy<Vec<i32>> {
        // Define the range of prices
        let price_range = 1000..=1500;

        vec(price_range, 100).boxed()
    }

    seq!(N in 0..1 {
        proptest! {
                #![proptest_config(ProptestConfig::with_cases(1))]
                #[test]
                fn test_100_buys_with_gain~N(prices  in  generate_prices_strategy_gain()) {
                    assert_eq!(prices.len(), 100, "There should be exactly 100 prices");
                    let  runtime = tokio::runtime::Runtime::new().unwrap();

                    runtime.block_on(async {
                        let db = Surreal::new::<Mem>(()).await.expect("Failed to initialize the database");
                        let _ = db.use_ns("test").use_db("test").await;

                        let mock_account = MockAccount::new();
                        let state: AppState = AppState { db,mock_account };


                        let router:Router = server::request::router(&state);
                        let prices_clone = prices.clone();

                        let address = "test_address".to_string();
                        let futures = prices.into_iter().map(|price| {

                            let address = "test_address".to_string();
                            let router_clone =router.clone();
                            async move {
                                create_agreement(1, price.into(), &address, URL_REQUEST_QUOTE, URL_ACCEPT_CONTRACT,router_clone).await
                            }
                        }).collect::<Vec<_>>();

                        let results = try_join_all(futures).await;
                        assert!(results.is_ok(), "One or more agreements failed to create successfully.");
                        let total_buying_prices: i64 = prices_clone.iter().map(|&x| x as i64).sum();
                        let settlement_price = 1500i64;
                        let expected_diff = 100 * settlement_price - total_buying_prices;

                        // Check settlement proof
                        let settlement_price = 1500i64;
                        let settlement_proof = request_settlement_proof_with_price(URL_REQUEST_SETTLEMENT_PROOF, &address, settlement_price, router.clone()).await;
                        assert_eq!(settlement_proof.unwrap().diff, expected_diff, "Expected gain did not match.");
                });

            }
        }
    });

    //Test 100 buys and 0 sells, the expected diff is 100*settlement_price - sum(buying_prices), loss -> the settlement_price is lower than buying price
    /// Generates a vector of 100 prices, each ranging from 1000 to 1500.
    fn generate_prices_strategy_loss() -> BoxedStrategy<Vec<i32>> {
        // Define the range of prices
        let price_range = 1500..=2000;

        // Create a vector of 100 prices within the specified range
        vec(price_range, 100).boxed()
    }
    seq!(N in 0..1 {
        proptest! {
                #![proptest_config(ProptestConfig::with_cases(1))]
                #[test]
                fn test_100_buys_with_loss~N(prices  in  generate_prices_strategy_loss()) {
                    assert_eq!(prices.len(), 100, "There should be exactly 100 prices");
                    let  runtime = tokio::runtime::Runtime::new().unwrap();

                    runtime.block_on(async {
                        let db = Surreal::new::<Mem>(()).await.expect("Failed to initialize the database");
                        let _ = db.use_ns("test").use_db("test").await;

                        let mock_account = MockAccount::new();
                        let state: AppState = AppState { db,mock_account };


                        let router:Router = server::request::router(&state);
                        let prices_clone = prices.clone();

                        let address = "test_address".to_string();
                        let futures = prices.into_iter().map(|price| {

                            let address = "test_address".to_string();
                            let router_clone =router.clone();
                            async move {
                                create_agreement(1, price.into(), &address, URL_REQUEST_QUOTE, URL_ACCEPT_CONTRACT,router_clone).await
                            }
                        }).collect::<Vec<_>>();

                        let results = try_join_all(futures).await;
                        assert!(results.is_ok(), "One or more agreements failed to create successfully.");
                        let total_buying_prices: i64 = prices_clone.iter().map(|&x| x as i64).sum();
                        let settlement_price = 1500i64;
                        let expected_diff = 100 * settlement_price - total_buying_prices;

                        // Check settlement proof
                        let settlement_price = 1500i64;
                        let settlement_proof = request_settlement_proof_with_price(URL_REQUEST_SETTLEMENT_PROOF, &address, settlement_price, router.clone()).await;
                        assert_eq!(settlement_proof.unwrap().diff, expected_diff, "Expected gain did not match.");

                     });

                }
        }
    });

    /// Generates two vectors of 50 prices each, ranging from 1500 to 2000, with the same sums but shuffled differently.
    fn generate_identical_but_shuffled_prices() -> BoxedStrategy<(Vec<i32>, Vec<i32>)> {
        let price_range = 1500..=2000;

        // Create a vector of 50 prices within the specified range
        vec(price_range, 50)
            .prop_map(|prices| {
                let mut rng = thread_rng(); // Create a random number generator
                let mut shuffled_prices = prices.clone(); // Clone the original prices
                shuffled_prices.shuffle(&mut rng); // Shuffle the cloned prices
                (prices, shuffled_prices) // Return the original and shuffled prices
            })
            .boxed()
    }
    seq!(N in 0..1 {
        proptest! {
                #![proptest_config(ProptestConfig::with_cases(1))]
                #[test]
                fn test_first_buys_then_sells~N((buy_prices, sell_prices)  in  generate_identical_but_shuffled_prices()) {
                    assert_eq!(buy_prices.len(), 50, "There should be exactly 50 buy prices");
                    assert_eq!(sell_prices.len(), 50, "There should be exactly 50 sell prices");
                    let  runtime = tokio::runtime::Runtime::new().unwrap();

                    runtime.block_on(async {
                        let db = Surreal::new::<Mem>(()).await.expect("Failed to initialize the database");
                        let _ = db.use_ns("test").use_db("test").await;

                        let mock_account = MockAccount::new();
                        let state: AppState = AppState { db,mock_account };


                        let router:Router = server::request::router(&state);

                        let address = "test_address".to_string();
                        let buy_futures = buy_prices.into_iter().map(|price| {
                            let address = address.clone();
                            let router_clone = router.clone();
                            async move {
                                create_agreement(1, price.into(), &address, URL_REQUEST_QUOTE, URL_ACCEPT_CONTRACT, router_clone).await
                            }
                        }).collect::<Vec<_>>();

                        let sell_futures = sell_prices.into_iter().map(|price| {
                            let address = address.clone();
                            let router_clone = router.clone();
                            async move {
                                create_agreement(-1, price.into(), &address, URL_REQUEST_QUOTE, URL_ACCEPT_CONTRACT, router_clone).await
                            }
                        }).collect::<Vec<_>>();
                        let results = try_join_all(buy_futures).await;
                        assert!(results.is_ok(), "One or more buy agreements failed to create successfully.");
                        let results = try_join_all(sell_futures).await;
                        assert!(results.is_ok(), "One or more sell agreements failed to create successfully.");

                        let settlement_price = 1500i64;
                        let settlement_proof = request_settlement_proof_with_price(URL_REQUEST_SETTLEMENT_PROOF, &address, settlement_price, router.clone()).await;
                        assert_eq!(settlement_proof.unwrap().diff, 0, "Expected gain did not match.");
                });

            }
        }
    });
}
