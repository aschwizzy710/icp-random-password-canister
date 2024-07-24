use candid::{CandidType, Deserialize};
use ic_cdk::api::time;
use ic_cdk_macros::{init, update};

#[derive(CandidType, Deserialize, Clone)]
struct Password {
    value: String,
}

#[init]
fn init() {
    ic_cdk::print("Password Generator canister initialized.");
}

#[update]
fn generate_password(length: u64) -> Password {
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                           abcdefghijklmnopqrstuvwxyz\
                           0123456789)(*&^%$#@!~";
    let mut password = String::new();
    let mut seed = time() as usize;
    for _ in 0..length {
        seed = seed.wrapping_add(seed * 1103515245 + 12345) % charset.len();
        let idx = seed % charset.len();
        password.push(charset[idx] as char);
    }
    Password { value: password }
}
