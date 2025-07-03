use std::env;
use std::sync::Once;

use dotenvy::dotenv;
use once_cell::sync::Lazy;

static INIT: Once = Once::new();

fn init_dotenv() {
    INIT.call_once(|| {
        dotenv().ok();
    });
}

pub static SECRET_KEY: Lazy<String> = Lazy::new(|| {
    init_dotenv();
    env::var("SECRET_KEY").expect("SECRET_KEY must be set")
});

pub static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    init_dotenv();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
});
