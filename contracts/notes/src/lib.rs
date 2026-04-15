#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec,
};

const CHEESE_DATA: Symbol = symbol_short!("CHEESE");

#[contracttype]
#[derive(Clone)]
pub struct Cheese {
    pub id: u64,
    pub name: String,
    pub origin: String,
    pub description: String,
}

#[contract]
pub struct CheeseContract;

#[contractimpl]
impl CheeseContract {
    pub fn get_cheeses(env: Env) -> Vec<Cheese> {
        env.storage()
            .instance()
            .get(&CHEESE_DATA)
            .unwrap_or_else(|| Vec::new(&env))
    }


    pub fn add_cheese(
        env: Env,
        name: String,
        origin: String,
        description: String,
    ) -> String {
        let mut cheeses: Vec<Cheese> = env
            .storage()
            .instance()
            .get(&CHEESE_DATA)
            .unwrap_or_else(|| Vec::new(&env));

        let id = cheeses.len() as u64 + 1;

        let cheese = Cheese {
            id,
            name,
            origin,
            description,
        };

        cheeses.push_back(cheese);
        env.storage().instance().set(&CHEESE_DATA, &cheeses);

        String::from_str(&env, "Cheese added successfully 🧀")
    }

    pub fn delete_cheese(env: Env, id: u64) -> String {
        let mut cheeses: Vec<Cheese> = env
            .storage()
            .instance()
            .get(&CHEESE_DATA)
            .unwrap_or_else(|| Vec::new(&env));

        for i in 0..cheeses.len() {
            if let Some(cheese) = cheeses.get(i) {
                if cheese.id == id {
                    cheeses.remove(i);
                    env.storage().instance().set(&CHEESE_DATA, &cheeses);

                    return String::from_str(&env, "Cheese deleted 🗑️");
                }
            }
        }

        String::from_str(&env, "Cheese not found")
    }

    pub fn update_cheese(
        env: Env,
        id: u64,
        new_name: String,
        new_origin: String,
        new_description: String,
    ) -> String {
        let mut cheeses: Vec<Cheese> = env
            .storage()
            .instance()
            .get(&CHEESE_DATA)
            .unwrap_or_else(|| Vec::new(&env));

        for i in 0..cheeses.len() {
            if let Some(mut cheese) = cheeses.get(i) {
                if cheese.id == id {
                    cheese.name = new_name;
                    cheese.origin = new_origin;
                    cheese.description = new_description;

                    cheeses.set(i, cheese);
                    env.storage().instance().set(&CHEESE_DATA, &cheeses);

                    return String::from_str(&env, "Cheese updated ✏️");
                }
            }
        }

        String::from_str(&env, "Cheese not found")
    }
}