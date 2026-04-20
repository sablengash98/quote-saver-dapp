#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, String, Symbol, Vec
};

// =======================
// STRUCT
// =======================
#[contracttype]
#[derive(Clone, Debug)]
pub struct Quote {
    pub id: u64,
    pub text: String,
    pub author: String,
    pub is_favorite: bool,
    pub created_at: u64,
}

// =======================
// STORAGE KEY
// =======================
const QUOTE_DATA: Symbol = symbol_short!("QUOTES");

// =======================
// CONTRACT
// =======================
#[contract]
pub struct QuoteContract;

// =======================
// IMPLEMENTATION
// =======================
#[contractimpl]
impl QuoteContract {

    // =======================
    // GET ALL QUOTES
    // =======================
    pub fn get_quotes(env: Env) -> Vec<Quote> {
        env.storage()
            .instance()
            .get(&QUOTE_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // =======================
    // CREATE QUOTE
    // =======================
    pub fn create_quote(env: Env, text: String, author: String) -> String {
        let mut quotes: Vec<Quote> = env.storage()
            .instance()
            .get(&QUOTE_DATA)
            .unwrap_or(Vec::new(&env));

        let quote = Quote {
            id: env.prng().gen::<u64>(),
            text: text,
            author: author,
            is_favorite: false,
            created_at: env.ledger().timestamp(),
        };

        quotes.push_back(quote);

        env.storage().instance().set(&QUOTE_DATA, &quotes);

        String::from_str(&env, "Quote berhasil ditambahkan")
    }

    // =======================
    // DELETE QUOTE
    // =======================
    pub fn delete_quote(env: Env, id: u64) -> String {
        let mut quotes: Vec<Quote> = env.storage()
            .instance()
            .get(&QUOTE_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..quotes.len() {
            if quotes.get(i).unwrap().id == id {
                quotes.remove(i);

                env.storage().instance().set(&QUOTE_DATA, &quotes);

                return String::from_str(&env, "Quote dihapus");
            }
        }

        String::from_str(&env, "Quote tidak ditemukan")
    }

    // =======================
    // TOGGLE FAVORITE
    // =======================
    pub fn toggle_favorite(env: Env, id: u64) -> String {
        let mut quotes: Vec<Quote> = env.storage()
            .instance()
            .get(&QUOTE_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..quotes.len() {
            let mut quote = quotes.get(i).unwrap();

            if quote.id == id {
                quote.is_favorite = !quote.is_favorite;

                quotes.set(i, quote);

                env.storage().instance().set(&QUOTE_DATA, &quotes);

                return String::from_str(&env, "Status favorite diubah");
            }
        }

        String::from_str(&env, "Quote tidak ditemukan")
    }
}