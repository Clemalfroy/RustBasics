extern crate uuid;

use self::uuid::Uuid;

pub mod cache;

#[derive(Serialize, Deserialize)]
pub struct CreateProduct {
    id: Option<Uuid>,

    price: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    id: Uuid,

    price: u32, 
}

impl Product {
    pub fn remove(self) -> Option<Self> {
        cache::remove(self.id)
    }
}

impl CreateProduct {
    pub fn new(self) {
        let uuid = Uuid::new_v4();
        let product = Product {
            id: uuid,
            price: self.price,
        };
        cache::add(uuid, product);
    }
}