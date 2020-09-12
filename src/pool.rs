use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::json_types::U128;
use num_integer::Roots;
use std::cmp;
use crate::mintable_fungible_token;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Pool {
    token0: AccountId,
    token1: AccountId,
    reserve0: u128,
    reserve1: u128,
    min_liquidity: u128,

    // TODO: all these attributes will be moved to fungible token
    token: mintable_fungible_token::MintableFungibleToken,
    // total_supply: u128,
    // balances: UnorderedMap<AccountId, u128>
}

impl Default for Pool {
    fn default() -> Self {
        panic!("Fun token should be initialized before usage")
    }
}

#[near_bindgen]
impl Pool {

    #[init]
    pub fn new(token0: AccountId, token1: AccountId) -> Self {
        Self {
            token0,
            token1,
            reserve0: 0,
            reserve1: 0,
            min_liquidity: (10 as u128).pow(3),
            
            token: mintable_fungible_token::MintableFungibleToken::new(),
            // total_supply: 0,
            // balances: UnorderedMap::new(b"test".to_vec())
        }
    }

    fn update(&mut self, new_balance0: u128, new_balance1: u128) {
        self.reserve0 += new_balance0;
        self.reserve1 += new_balance1;
    }

    pub fn add_liquidity(&mut self, amount0: U128, amount1: U128) {
        let mut liquidity;
        let amount0: u128 = amount0.into();
        let amount1: u128 = amount1.into();

        // TODO: transfer tokens to this contract

        if self.token.total_supply == 0 {
            liquidity = (amount0 * amount1).sqrt() - self.min_liquidity;
            self.token.mint(&"NULL_ADDRESS".to_string(), self.min_liquidity);
        } else {
            liquidity = cmp::min(
                amount0 * self.token.total_supply / self.reserve0, 
                amount1 * self.token.total_supply / self.reserve1
            );
        }

        assert!(liquidity >  0, "not enough liquidity suplied");

        self.token.mint(&env::predecessor_account_id(), liquidity);
        self.update(self.reserve0 + amount0, self.reserve1 + amount1);
    }

    pub fn remove_liquidity(&mut self) {
        let liquidity: u128 = self.token.get_balance(env::predecessor_account_id()).into();
        assert!(liquidity > 0, "user has no liquidity to remove");

        let amount0 = liquidity * self.reserve0 / self.token.total_supply;
        let amount1 = liquidity * self.reserve1 / self.token.total_supply;

        assert!(amount0 > 0 && amount1 > 0, "not enough liquidity burned");

        // TODO transfer t0 and t1 to user

        self.update(self.reserve0 - amount0, self.reserve1 - amount1);
        self.token.burn(&env::predecessor_account_id(), liquidity);
    }

    pub fn swap_t0_for_t1(&mut self, amount_out: U128) {
        let amount_out: u128 = amount_out.into();
        assert!(amount_out < self.reserve1, "not enough liquidity");
        assert!(amount_out > 0, "invalid output amount");

        let r0 = self.reserve0;
        let r1 = self.reserve1;
        let k = r0 * r1;

        let new_r1 = r1 - amount_out;
        let new_r0 = k / new_r1;

        let cost = new_r0 - r0;

        // TODO: transfer cost of r0 to contract
        // TODO: transfer amount_out to sender

        self.update(new_r0, new_r1);
    }
    
    pub fn swap_t1_for_t0(&mut self, amount_out: U128) {
        let amount_out: u128 = amount_out.into();
        assert!(amount_out < self.reserve0, "not enough liquidity");
        assert!(amount_out > 0, "invalid output amount");

        let r0 = self.reserve0;
        let r1 = self.reserve1;
        let k = r0 * r1;

        let new_r0 = r0 - amount_out;
        let new_r1 = k / new_r0;

        let cost = new_r1 - r1;

        // TODO: transfer cost of r0 to contract
        // TODO: transfer amount_out to sender

        self.update(new_r0, new_r1);
    }

    // getter functions

    //  pub fn get_curr_price_t0(&self) -> u128 {
    //     self.
    //  }

    // TODO: expose standard erc20 interface so that pool tokens can be transfered etc.
}

