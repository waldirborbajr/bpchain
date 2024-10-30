use std::collections::BTreeMap;

#[warn(dead_code)]

pub struct Pallet {
    block_number: u32,
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }
}
