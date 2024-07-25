use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::time;
use ic_cdk_macros::{init, update, query};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
struct Password {
    value: String,
    created_at: u64,
    owner: Principal,
}

#[derive(CandidType, Deserialize, Clone)]
struct PasswordPayload {
    value: String,
}

thread_local! {
    static PASSWORD_STORE: RefCell<HashMap<u64, Password>> = RefCell::new(HashMap::new());
    static ID_COUNTER: RefCell<u64> = RefCell::new(0);
}

#[init]
fn init() {
    ic_cdk::print("Password Generator canister initialized.");
}

#[update]
fn generate_password(length: u64) -> Result<Password, String> {
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
    let generated_password = Password {
        value: password,
        created_at: time(),
        owner: ic_cdk::caller(),
    };
    store_password(generated_password.clone());
    Ok(generated_password)
}

#[update]
fn store_password(password: Password) {
    PASSWORD_STORE.with(|store| {
        let mut store = store.borrow_mut();
        let id = ID_COUNTER.with(|counter| {
            let id = *counter.borrow() + 1;
            *counter.borrow_mut() = id;
            id
        });
        store.insert(id, password);
    });
}

#[update]
fn update_password(id: u64, new_password: PasswordPayload) -> Result<String, String> {
    PASSWORD_STORE.with(|store| {
        let mut store = store.borrow_mut();
        if let Some(existing_password) = store.get_mut(&id) {
            if existing_password.owner == ic_cdk::caller() {
                existing_password.value = new_password.value;
                existing_password.created_at = time();
                Ok("Password updated successfully.".to_string())
            } else {
                Err("Unauthorized: You do not own this password.".to_string())
            }
        } else {
            Err("Password not found.".to_string())
        }
    })
}

#[update]
fn delete_password(id: u64) -> Result<String, String> {
    PASSWORD_STORE.with(|store| {
        let mut store = store.borrow_mut();
        if let Some(password) = store.get(&id) {
            if password.owner == ic_cdk::caller() {
                store.remove(&id);
                Ok("Password deleted successfully.".to_string())
            } else {
                Err("Unauthorized: You do not own this password.".to_string())
            }
        } else {
            Err("Password not found.".to_string())
        }
    })
}

#[query]
fn get_password(id: u64) -> Result<Password, String> {
    PASSWORD_STORE.with(|store| {
        store
            .borrow()
            .get(&id)
            .cloned()
            .ok_or_else(|| "Password not found.".to_string())
    })
}

#[query]
fn list_passwords() -> Vec<Password> {
    PASSWORD_STORE.with(|store| {
        store
            .borrow()
            .values()
            .cloned()
            .collect()
    })
}

#[query]
fn validate_password(password: PasswordPayload) -> Result<String, String> {
    let value = password.value;
    if value.len() < 8 {
        return Err("Password must be at least 8 characters long.".to_string());
    }
    if !value.chars().any(|c| c.is_uppercase()) {
        return Err("Password must contain at least one uppercase letter.".to_string());
    }
    if !value.chars().any(|c| c.is_lowercase()) {
        return Err("Password must contain at least one lowercase letter.".to_string());
    }
    if !value.chars().any(|c| c.is_numeric()) {
        return Err("Password must contain at least one number.".to_string());
    }
    if !value.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:'\",.<>?/".contains(c)) {
        return Err("Password must contain at least one special character.".to_string());
    }
    Ok("Password is valid.".to_string())
}

#[query]
fn check_password_uniqueness(password: PasswordPayload) -> Result<bool, String> {
    PASSWORD_STORE.with(|store| {
        let store = store.borrow();
        let is_unique = !store.values().any(|p| p.value == password.value);
        Ok(is_unique)
    })
}

ic_cdk::export_candid!();
