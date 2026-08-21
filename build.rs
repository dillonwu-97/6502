use serde_json::Value;
use std::collections::{BTreeSet, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

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
    let mut ret = format!("#[allow(non_camel_case_types)]\n#[derive(Debug, PartialEq, Eq, Clone)]\npub enum {} {{\n", name);
    let mut h: HashSet<String> = HashSet::new();

    for (i, v) in src.into_iter().enumerate() {
        if !h.contains(&v) {
            h.insert(v.clone());
            ret.push_str(&format!("   {} = 0x{:02x},\n", v, i));
        }
    }
    ret.push_str("}\n");
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
    let mut h: HashSet<String> = HashSet::new();
    for (i,v) in src.into_iter().enumerate() {
        if !h.contains(&v) {
            h.insert(v.clone());
            ret.push_str(&format!("\t 0x{:02x} => {}::{},\n", i, name, v));
        }
    }
    ret.push_str("_ => panic!(\"Error for value {}\", value)");
    ret.push_str("\t\t}\n\t}\n}");
    ret
}

fn build_op() -> String {
    "#[derive(Clone)]
    pub struct Op {
        pub op: Opcode,
        pub inst: Inst,
        pub addr_mode: AddrMode,
        pub cycle: u8,
        pub pagex: bool,
        pub opnum: u8,
    }\n
    impl Op {
        pub fn new(op: Opcode, inst: Inst, addr_mode: AddrMode, cycle: u8, pagex: bool, opnum: u8) -> Self {
            Self {
                op: op,
                inst: inst,
                addr_mode: addr_mode,
                cycle: cycle,
                pagex: pagex, 
                opnum: opnum
            }
        }
    }
    ".to_string()
}

// Build out the code for the vector
// for each thing in the serde_json opcode file,
//  build struct using input
fn build_op_vec(ops: Value) -> String {
      
    let mut ret: String = "fn build_opcodes() -> Vec<Op> {let opcode_arr: Vec<Op> = vec![".to_string();
    for (i,v) in ops.as_array().unwrap().iter().enumerate() {
        // let inst: String = v["inst"];
        println!("{:?}\n", v);
        // println!("{}\n", i);
        println!("{}", v["mode"]);
        let new_struct = format!("Op {{ op: Opcode::{}_{}, inst: Inst::{}, addr_mode: AddrMode::{}, cycle: {}, pagex: {}, opnum: 0x{:x} }},", 
            v["inst"].as_str().unwrap(), 
            v["mode"].as_str().unwrap(), 
            v["inst"].as_str().unwrap(), 
            v["mode"].as_str().unwrap(), 
            v["cycle_count"],
            v["pagex"],
            i);
        ret.push_str(&new_struct);
    }
    ret.push_str("];assert!(opcode_arr.len() == 0xff);opcode_arr}");
    ret
}


fn main() {
    // Tell cargo to rerun this build script if opcodes.txt changes
    // Put into fn 
    let file = File::open("./assets/opcodes.json").unwrap();
    let reader = BufReader::new(file);
    let ops: Value = serde_json::from_reader(reader).unwrap();

    let addr_modes: BTreeSet<String> = get_iters(ops.clone(), vec!["mode".to_string()]);
    let instructions: BTreeSet<String> = get_iters(ops.clone(), vec!["inst".to_string()]);
    let opcodes: Vec<String> = get_iters(ops.clone(), vec!["inst".to_string(), "mode".to_string()]);

    let addr_enum = build_enum(addr_modes.clone(), "AddrMode".to_string());
    let inst_enum = build_enum(instructions.clone(), "Inst".to_string());
    let opcode_enum = build_enum(opcodes.clone(), "Opcode".to_string());
    // println!("{:?}", addr_modes);


    let addr_impl = build_impl(addr_modes, "AddrMode".to_string());
    let inst_impl = build_impl(instructions, "Inst".to_string());
    let opcode_impl = build_impl(opcodes, "Opcode".to_string());
    // println!("{:?}", addr_impl);

    let op_struct = build_op();
    let op_vec = build_op_vec(ops.clone());

    let to_write_arr: Vec<String> = vec![
        addr_enum, inst_enum, opcode_enum,
        addr_impl, inst_impl, opcode_impl,
        op_struct, op_vec
    ];


    let mut to_write: String = String::new();
    for (_,v) in to_write_arr.into_iter().enumerate() {
        to_write.push_str(&v); 
        to_write.push_str("\n"); 

    }


    let out_dir = env::var("OUT_DIR").unwrap();
    let dst_path = Path::new(&out_dir).join("opcodes.rs");
    println!("{}", to_write);
    let mut file = File::create(&dst_path).unwrap();
    file.write_all(to_write.as_bytes());
    println!("cargo:warning=The active OUT_DIR is: {}", out_dir);
    

}


// Good reading:
// https://stackoverflow.com/questions/73673613/what-is-the-difference-between-optionnone-in-rust-and-null-in-other-languages

