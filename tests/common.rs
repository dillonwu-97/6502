#[derive(Debug, Deserialize, Serialize)]
pub struct TestCase {
    pub name: String,
    pub initial: Parameters,
    pub final: Parameters,
    pub cycles: Vec<u16, u8, String)>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Parameters {
    pc: u16,
    s: u8,
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    ram: Vec<(u16, u8)>,
}
