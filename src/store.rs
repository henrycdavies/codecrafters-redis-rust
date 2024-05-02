use std::{collections::HashMap, sync::{Arc, Mutex}};

pub type KeyValueStore<T> = Arc<Mutex<HashMap<String, T>>>;

pub fn create_key_value_store<T>() -> KeyValueStore<T> {
    Arc::new(Mutex::new(HashMap::new()))
}