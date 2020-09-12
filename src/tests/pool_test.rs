use super::*;

#[test]
fn init() {
    let context = get_context(carol());
    testing_env!(context);

    let mut contract = pool::Pool::new("t1".to_string(), "t2".to_string());

    contract.add_liquidity(U128(10), U128(10));


}