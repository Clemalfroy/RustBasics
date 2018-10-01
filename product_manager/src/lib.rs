extern crate uuid;
use uuid::Uuid;
use std::collections::BTreeSet;
use std::cmp::Ordering;

#[derive(Debug, Eq)]
pub struct ProductFamily {

    pub product_family_id: Uuid,

    pub variants: BTreeSet<Product>,
}

impl PartialOrd for ProductFamily {
    fn partial_cmp(&self, other: &ProductFamily) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ProductFamily {
    fn cmp(&self, other: &ProductFamily) -> Ordering {
        self.product_family_id.cmp(&other.product_family_id)
    }
}

impl PartialEq for ProductFamily {
    fn eq(&self, other: &ProductFamily) -> bool {
        self.product_family_id == other.product_family_id
    }
}

impl ProductFamily {
    pub fn new(product_family_id: Uuid) -> Self {
        ProductFamily{product_family_id, variants: BTreeSet::new()}
    }

    pub fn add(&mut self, new_variant: Product) -> bool {
        return self.variants.insert(new_variant);
    }
}

#[derive(Debug, Eq)]
pub struct Product {

    pub product_id: Uuid,

    pub price: u32,
    pub quantity: u32,
}

impl Product {

    pub fn new(price: u32, quantity: u32, product_id: Uuid) -> Self {
        Product { price, quantity, product_id }
    }

    pub fn add(&mut self) {
        self.quantity += 1;
    }

    pub fn remove(&mut self) -> bool {

        if !self.has_stock() { false }
        else {
        self.quantity -= 1;
        true
        }
    }

    fn has_stock(&self) -> bool {
        if self.quantity == 0 { false } else { true }
    }
}

impl PartialOrd for Product {
    fn partial_cmp(&self, other: &Product) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Product {
    fn cmp(&self, other: &Product) -> Ordering {
        self.product_id.cmp(&other.product_id)
    }
}

impl PartialEq for Product {
    fn eq(&self, other: &Product) -> bool {
        self.product_id == other.product_id
    }
}