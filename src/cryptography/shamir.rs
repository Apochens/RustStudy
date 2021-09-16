pub struct Participant {
    id: u32,
    others_ids: Vec<u32>,
    secret: u32,
}

pub struct Shamir {
    coefficients: Vec<u32>,
    secret: u32,
    ids: Vec<32>,
}

impl Shamir {
    pub fn init() {
        
    }
}