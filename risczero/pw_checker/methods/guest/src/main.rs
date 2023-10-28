#![no_main]

use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
};

risc0_zkvm::guest::entry!(main);

fn has_special_char(pw: &String) -> bool {
    for ch in pw.chars() {
        if ch.is_ascii_punctuation() {
            return true;
        }
    }
    false
}

pub fn main() {
    // read a password
    let pw: String = env::read();
    // check if it has any special characters
    if !has_special_char(&pw) {
        panic!();
    }
    // return hash
    let sha = Impl::hash_bytes(pw.as_bytes());
    env::commit(sha);
}
