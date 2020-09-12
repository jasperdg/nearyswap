use super::*;

#[test]
fn init() {
    let context = get_context(carol());
    testing_env!(context);
    
    let mut contract = pool::Pool::new("t1".to_string(), "t2".to_string());
    
    contract.add_liquidity(U128(100000000), U128(100000000));
    let balance = contract.get_balance(carol());
    assert_eq!(balance, U128(100000000 - min_liq()));
    
    // let context = get_context(alice());
    // testing_env!(context);
    // contract.add_liquidity(U128(10000), U128(10000));
    // let balance = contract.get_balance(alice());
    // assert_eq!(balance, U128(10000));
    
    contract.swap_t0_for_t1(U128(100));
    contract.swap_t0_for_t1(U128(100));
    contract.swap_t0_for_t1(U128(100));
    contract.swap_t1_for_t0(U128(100));
}