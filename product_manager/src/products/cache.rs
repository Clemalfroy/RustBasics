extern crate uuid;

use self::uuid::Uuid;
use ::Product;

use std::sync::Mutex;
use std::collections::HashMap;

lazy_static! {
    pub static ref HASHMAP: Mutex<HashMap<Uuid, Product>> =  Mutex::new(HashMap::new()); 
}

pub fn add(id: Uuid, product: Product) {
    let mut hashmap = HASHMAP.lock().unwrap();
    hashmap.entry(id).or_insert(product);
}

pub fn remove(id: Uuid) -> Option<Product> {
    let mut hashmap = HASHMAP.lock().unwrap();
    hashmap.remove(&id)
}

pub fn display_all() {
    for (key, val) in HASHMAP.lock().unwrap().iter() {
        println!("key: {} val: {:?}", key, val);
    }
}