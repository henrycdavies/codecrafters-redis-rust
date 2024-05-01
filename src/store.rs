use std::{collections::HashMap, sync::{Arc, Mutex}};

pub type KeyValueStore = Arc<Mutex<HashMap<String, String>>>;

pub fn create_key_value_store() -> KeyValueStore {
    Arc::new(Mutex::new(HashMap::new()))
}