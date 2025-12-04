use mos::emulator::opcodes;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::format;
use std::fs::{read, File};
use std::io::{self, BufRead};
use std::io::{prelude::*, Lines};
use std::path::Path;

// TODO: move this to a different file and allow it to be imported / reused elsewhere in the code
#[derive(Debug, Serialize, Clone)]
struct Opcode {
    inst: String,
    mode: String,
    cycle_count: u8,
    opcode: String,
    pagex: bool, // checks if the instruction crosses page boundary and incurs an extra cycle count
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let f = File::open(filename)?;
    Ok(io::BufReader::new(f).lines())
}

fn dump_to_json(filename: String, op_vec: &Vec<Opcode>) {
    /**
     * Dump vector of structs to a json file
     *
     * Arguments
     */
    let json = serde_json::to_string_pretty(op_vec).unwrap();
    let mut file = File::create(filename).unwrap();
    file.write_all(json.as_bytes());
}

fn reformat_lines<P>(path: P, addr_modes: &HashMap<&str, &str>) -> Vec<String>
where
    P: AsRef<Path>,
{
    let mut lines_to_parse: Vec<String> = Vec::new();

    match read_lines(path) {
        Ok(lines) => {
            println!("okay");
            // this works for a single word
            // lines.for_each(|b| println!("{}", b.as_ref().unwrap()));

            // Filter for anything that starts with something in addr_modes keys
            let c = lines.filter(|line| {
                line.as_ref()
                    .map(|s| addr_modes.clone().into_keys().any(|val| s.starts_with(val)))
                    .expect("error")
            });
            c.for_each(|b| {
                lines_to_parse.push(b.expect("error 2").clone());
                // println!("{}", b.as_ref().unwrap());
            });
        }
        Err(_) => println!("bad"),
    }
    println!("Finished parsing");
    lines_to_parse
}

// TODO: is there a way to move the hashmap to a higher scope so that i don't have to keep
// passing it as user argument?
fn construct_opcode_vec(
    lines_to_parse: Vec<String>,
    addr_modes: &HashMap<&str, &str>,
) -> Vec<Opcode> {
    let mut opcode_vec: Vec<Opcode> = Vec::new();
    let mut inst: String;
    let mut mode: String;
    let mut opcode: String;
    let mut cycle_count: u8;
    let mut pagex: bool;

    for line in lines_to_parse {
        println!("{}", line);

        let tokens: Vec<&str>;
        tokens = line
            .split_whitespace()
            .filter(|f| !f.contains("oper"))
            .collect::<Vec<&str>>();

        mode = tokens[0].to_string().clone();
        mode = addr_modes.get(&mode as &str).unwrap().to_string();
        inst = tokens[1].to_string();
        // From parsing the file, the opcode is the third to last char, and cycle count is last
        // char
        let s_cycle_cnt: String = tokens[tokens.len() - 1].to_string();
        if s_cycle_cnt.contains("*") {
            pagex = true;
            cycle_count = s_cycle_cnt.replace("*", "").parse::<u8>().unwrap();
        } else {
            pagex = false;
            cycle_count = s_cycle_cnt.parse::<u8>().unwrap();
        }
        // opcode = u8::from_str_radix(tokens[tokens.len() - 3].trim(), 16).unwrap();
        // opcode = u8::from_str_radix(tokens[tokens.len() - 3].trim(), 16)
        opcode = String::from(tokens[tokens.len() - 3].trim());

        // println!("mode 2: {} {} {}", mode, inst, opcode);

        opcode_vec.push(Opcode {
            inst: inst,
            mode: mode,
            opcode: opcode,
            cycle_count: cycle_count,
            pagex: pagex,
        });
        // println!("{:#?}", opcode_vec[opcode_vec.len() - 1]);
    }

    return opcode_vec;
}

fn add_nop_codes<P>(path: P, addr_modes: &HashMap<&str, &str>) -> Vec<Opcode>
where
    P: AsRef<Path>,
{
    // NOP 1A	implied	1	2
    // NOP 3A	implied	1	2
    // NOP 5A	implied	1	2

    let mut lines_to_iter: Vec<String> = Vec::new();
    match read_lines(path) {
        Ok(lines) => {
            lines
                .filter(|f| {
                    if let Ok(line) = f {
                        line.split_whitespace().next() == Some("NOP")
                    } else {
                        false
                    }
                })
                .for_each(|v| lines_to_iter.push(v.unwrap()));
        }
        Err(_) => {
            println!("bad")
        }
    }
    let mut nop_vec: Vec<Opcode> = Vec::new();
    // println!("add nop codes");
    let mut inst: String;
    let mut mode: String;
    let mut cycle_count: u8;
    let mut opcode: String;
    let mut pagex: bool;

    for line in lines_to_iter {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        inst = tokens[0].to_string();
        opcode = tokens[1].to_string();
        mode = addr_modes.get(tokens[2]).unwrap().to_string();
        if tokens[tokens.len() - 1].contains("*") {
            pagex = true;
            cycle_count = tokens[tokens.len() - 1]
                .replace("*", "")
                .parse::<u8>()
                .unwrap();
        } else {
            pagex = false;
            cycle_count = tokens[tokens.len() - 1].parse::<u8>().unwrap();
        }
        nop_vec.push(Opcode {
            inst: inst,
            mode: mode,
            opcode: opcode,
            cycle_count: cycle_count,
            pagex: pagex,
        });
        println!("nop {}", line);
    }

    // for op in nop_vec.clone() {
    //     println!("{:?}", op);
    // }
    nop_vec
}
/*
 * In the document, the nop codes are formatted differently, so we have another function to add
 * just those
 */

fn add_jam_codes() -> Vec<Opcode> {
    /*
     * In the document, the JAM opcodes are formatted differently, so we have another fn to handle
     * those
     */
    let jam: Vec<u8> = vec![
        0x02, 0x12, 0x22, 0x32, 0x42, 0x52, 0x62, 0x72, 0x92, 0xB2, 0xD2, 0xF2,
    ];
    let s_jam = jam.iter().map(|f| format!("{:02X}", f));

    let mut jam_op: Vec<Opcode> = Vec::new();
    for op in s_jam {
        jam_op.push(Opcode {
            inst: String::from("JAM"),
            mode: String::from("NA"),
            opcode: op,
            cycle_count: 0,
            pagex: false,
        })
    }
    jam_op
}

// TODO:modify the code so that we open the file only once and grab the data without constantly
// reopening
fn main() {
    let addr_modes = HashMap::from([
        ("implied", "IMP"),
        ("immediate", "IMM"),
        ("accumulator", "ACC"),
        ("zeropage", "ZPG"),
        ("zeropage,X", "ZPX"),
        ("zeropage,Y", "ZPY"),
        ("relative", "REL"),
        ("absolute", "ABS"),
        ("absolute,X", "ABX"),
        ("absolute,Y", "ABY"),
        ("indirect", "IND"),
        ("(indirect,X)", "IDX"),
        ("(indirect),X", "INX"),
        ("(indirect,Y)", "IDY"),
        ("(indirect),Y", "INY"),
    ]);

    let path_name: String = format!("{}/scripts/opcode_docs.txt", env!("CARGO_MANIFEST_DIR"));

    let illegal_op_file: String = format!(
        "{}/scripts/illegal_opcode_patch.txt",
        env!("CARGO_MANIFEST_DIR")
    );

    let path = Path::new(&path_name);
    // let mut file = File::open(&path).unwrap();
    // let mut s = String::new();
    // file.read_to_string(&mut s);

    let lines_to_parse: Vec<String> = reformat_lines(path, &addr_modes);

    let ill_path: &Path = Path::new(&illegal_op_file);
    let illegal_opcode_lines: Vec<String> = reformat_lines(ill_path, &addr_modes);

    let mut opcode_vec: Vec<Opcode> = Vec::new();

    // TODO: need a clean cp of the opened file
    let mut nop_lines = lines_to_parse.clone();
    opcode_vec = construct_opcode_vec(lines_to_parse, &addr_modes);

    // TODO: use of moved value

    let mut opcode_vec_2: Vec<Opcode> = Vec::new();
    println!("{}", illegal_opcode_lines.len());
    opcode_vec_2 = construct_opcode_vec(illegal_opcode_lines, &addr_modes);

    let mut nop_vec: Vec<Opcode> = add_nop_codes(ill_path, &addr_modes);
    let mut jam_vec: Vec<Opcode> = add_jam_codes();
    opcode_vec.extend(opcode_vec_2);
    opcode_vec.extend(nop_vec);
    opcode_vec.extend(jam_vec);

    // opcode_vec_2
    //     .clone()
    //     .iter()
    //     .for_each(|f| println!("{}", f.opcode));
    opcode_vec.sort_by(|a, b| a.opcode.cmp(&b.opcode));
    println!("Length: {}", opcode_vec.len());
    let json_str: String = String::from("./temp.json");
    dump_to_json(json_str, &opcode_vec);
}

// TODO: need special case to handle all the NOP and halt instructions as well
