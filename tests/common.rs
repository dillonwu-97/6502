use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TestCase {
    pub name: String,
    pub initial: Parameters,
    pub r#final: Parameters,
    pub cycles: Vec<(usize, u8, String)>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Parameters {
    pub pc: u16,
    pub s: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub p: u8,
    pub ram: Vec<(usize, u8)>,
}
