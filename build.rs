use serde_json::Value;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::BufReader;

// Build the string to use 
fn get_iters<C: FromIterator<String>>(ops: Value, fields: Vec<String>) -> C {
    ops.as_array()
        .unwrap()
        .into_iter()
        .filter_map(|v| {
            let mut ret: String = String::new();
            for (i,j) in fields.iter().enumerate() {
                ret.push_str(v.get(j)?.as_str()?);
                if i != fields.len()-1 {
                    ret.push_str("_");
                }
            }
            Some(ret)
        })
        .collect()
}

fn build_enum<I>(src: I, name: String) -> String
where
    I: IntoIterator<Item = String>,
{
    let mut ret = format!("#[derive(Debug, PartialEq, Eq)]\npub enum {} {{\n", name);

    for (i, v) in src.into_iter().enumerate() {
        ret.push_str(&format!("   {} = 0x{:02x}\n", v, i));
    }

    ret.push_str("}\n");
    println!("{}", ret);
    ret
}

// TODO: fix the way this looks build for arbitary i think 
fn build_impl<I> (src: I, name: String) -> String 
where
    I: IntoIterator<Item = String>
{
    let mut ret: String = format!("impl From <u8> for {} {{\n\t\
    fn from(value: u8) -> Self {{\n\t\t\
    match value {{\n\t\t\t\
    ", name).to_string();
    for (i,v) in src.into_iter().enumerate() {
        ret.push_str(&format!("\t 0x{:02x} => {}::{}\n", i, name, v));
    }
    ret.push_str("\t\t}\n\t}\n}");
    ret
}

fn build_struct() -> String {
    "#[derive(Clone)]
    pub struct OpWrapper {
        pub op: Opcode,
        pub inst: Inst,
        pub addr_mode: AddrMode,
        pub cycle: u8,
        pub pagex: bool,
    }".to_string()

  // {
  //   "inst": "SLO",
  //   "mode": "ABY",
  //   "cycle_count": 7,
  //   "opcode": "1B",
  //   "pagex": false
  // },
}

fn main() {
    // Tell cargo to rerun this build script if opcodes.txt changes
    // Put into fn 
    let file = File::open("./opcodes.json").unwrap();
    let reader = BufReader::new(file);
    let ops: Value = serde_json::from_reader(reader).unwrap();

    let addr_modes: BTreeSet<String> = get_iters(ops.clone(), vec!["mode".to_string()]);
    let instructions: BTreeSet<String> = get_iters(ops.clone(), vec!["inst".to_string()]);
    let opcodes: Vec<String> = get_iters(ops.clone(), vec!["inst".to_string(), "mode".to_string()]);
    _ = build_enum(addr_modes.clone(), "AddrMode".to_string());
    _ = build_enum(instructions.clone(), "Inst".to_string());
    // _ = build_enum(opcodes.clone(), "Opcode".to_string());
    let a = build_impl(addr_modes, "AddrMode".to_string());
    let b = build_impl(instructions, "Inst".to_string());
    let c = build_impl(opcodes, "Opcode".to_string());
    println!("{}",b);
}
