use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use petname::petname;

pub fn gen_petname() -> String {
    petname(2, "-")
}

pub fn next_global_counter(global_counter: &Lazy<Arc<Mutex<u64>>>) -> u64 {
    let mut counter = global_counter.lock().unwrap();
    let current_val = *counter;
    *counter += 1;

    current_val
}
