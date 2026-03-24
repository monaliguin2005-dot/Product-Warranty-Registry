#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, String, Map};

#[contract]
pub struct WarrantyRegistry;

#[contractimpl]
impl WarrantyRegistry {

    // Register a product warranty
    pub fn register_warranty(
        env: Env,
        product_id: String,
        owner: String,
        expiry_date: String,
    ) {
        let mut storage: Map<String, (String, String)> =
            env.storage().instance().get(&Symbol::short("REG")).unwrap_or(Map::new(&env));

        storage.set(product_id.clone(), (owner, expiry_date));
        env.storage().instance().set(&Symbol::short("REG"), &storage);
    }

    // Get warranty details
    pub fn get_warranty(env: Env, product_id: String) -> (String, String) {
        let storage: Map<String, (String, String)> =
            env.storage().instance().get(&Symbol::short("REG")).unwrap();

        storage.get(product_id).unwrap()
    }
}