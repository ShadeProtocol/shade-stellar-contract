#![cfg(test)]
use crate::shade::{Shade, ShadeClient};
use soroban_sdk::testutils::{Address as _, Events as _};
use soroban_sdk::{Address, Env, Symbol, TryIntoVal};

#[test]
fn debug_events() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Shade, ());
    let client = ShadeClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.initialize(&admin);

    let events = env.events().all();
    for (i, (cid, topics, data)) in events.iter().enumerate() {
        if cid == contract_id {
            std::println!("Event {}:", i);
            for (j, topic) in topics.iter().enumerate() {
                let sym: Result<Symbol, _> = topic.try_into_val(&env);
                if let Ok(s) = sym {
                    std::println!("  Topic {}: Symbol({:?})", j, s);
                } else {
                    std::println!("  Topic {}: Not a symbol", j);
                }
            }
        }
    }
}
