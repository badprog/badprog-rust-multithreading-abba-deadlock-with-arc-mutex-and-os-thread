// badprog.com

// mod
mod core;

// lib
use std::env;

// crate
use crate::core::transfer::run;

// ------------------------------------
// main
// ------------------------------------
fn main() {
    let args: Vec<String> = env::args().collect();
    run(&args);
}
