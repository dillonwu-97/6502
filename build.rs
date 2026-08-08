use serde_json::Value;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::BufReader;

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

fn get_iters<C: FromIterator<String>>(ops: Value, fields: Vec<String>) -> C {
    ops.as_array()
        .unwrap()
        .into_iter()
        .filter_map(|v| v.get("inst")?.as_str())
        .map(|s| s.to_string())
        .collect()
}

fn main() {
    // Tell cargo to rerun this build script if opcodes.txt changes
    //
    let file = File::open("./assets/opcodes.json").unwrap();
    let reader = BufReader::new(file);
    let ops: Value = serde_json::from_reader(reader).unwrap();
    let instructions: BTreeSet<String> = ops
        .as_array()
        .unwrap()
        .into_iter()
        .filter_map(|v| v.get("inst")?.as_str())
        .map(|s| s.to_string())
        .collect();
    //
    // for (i,v) in instructions.into_iter().enumerate() {
    //     println!("{}, {:?}",i, v);
    // }

    let addr_modes: BTreeSet<String> = ops
        .as_array()
        .unwrap()
        .into_iter()
        .filter_map(|v| v.get("mode")?.as_str())
        .map(|s| s.to_string())
        .collect();

    let opcodes: Vec<String> = ops
        .as_array()
        .unwrap()
        .into_iter()
        .filter_map(|v| {
            Some(format!(
                "{}_{}",
                v.get("inst")?.as_str()?,
                v.get("mode")?.as_str()?
            ))
        })
        .collect();

    println!("{:?}", opcodes);
    _ = build_enum(addr_modes, "AddrMode".to_string());
    _ = build_enum(instructions, "Inst".to_string());
    _ = build_enum(opcodes, "Opcode".to_string());

    // println!("{}", addr_modes.len());
    // for (i,v) in addr_modes.into_iter().enumerate() {
    //     println!("{}, {:?}",i, v);
    // }

    // build_addr_mode();

    // let mut addr_modes = String::from("#[derive(Debug, PartialEq, Eq)]\npub enum AddrMode {\n");

    // for line in addr_modes.lines() {
    //     let name = line.trim();
    //     if !name.is_empty() {
    //         enum_code.push_str(&format!("    {},\n", name));
    //     }
    // }
    // enum_code.push_str("}\n");

    // fs::write(&dest_path, enum_code).unwrap();
}
