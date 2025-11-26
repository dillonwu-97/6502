use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

// TODO: move this to a different file and allow it to be imported / reused elsewhere in the code
#[derive(Debug, Serialize)]
struct Opcode {
    inst: String,
    mode: String,
    cycle_count: u8,
    opcode: u8,
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
    let mut opcode: u8;
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
        opcode = u8::from_str_radix(tokens[tokens.len() - 3].trim(), 16).unwrap();

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
    opcode_vec = construct_opcode_vec(lines_to_parse, &addr_modes);
    println!("Done");

    let mut opcode_vec_2: Vec<Opcode> = Vec::new();
    println!("{}", illegal_opcode_lines.len());
    opcode_vec_2 = construct_opcode_vec(illegal_opcode_lines, &addr_modes);
    // opcode_vec_2
    //     .iter()
    //     .for_each(|f| println!("hi {}", f.opcode));

    opcode_vec.extend(opcode_vec_2);

    let json_str: String = String::from("./temp.json");
    dump_to_json(json_str, &opcode_vec);
}

// TODO: need special case to handle all the halt instructions as well
//
//
//
//
//
