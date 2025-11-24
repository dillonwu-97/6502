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

fn main() {
    let path_name: String = format!("{}/scripts/opcode_docs.txt", env!("CARGO_MANIFEST_DIR"));

    let path = Path::new(&path_name);
    let display = path.display();
    println!("{}", display);

    let mut file = File::open(&path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s);

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

    let mut lines_to_parse: Vec<String> = Vec::new();
    let mut opcode_vec: Vec<Opcode> = Vec::new();

    match read_lines(path) {
        Ok(lines) => {
            println!("okay");
            // this works for a single word
            // let a = lines.filter(|line| line.as_ref().unwrap().starts_with("immediate"));
            // a.for_each(|b| println!("{}", b.as_ref().unwrap()));

            // for multiple words:
            let c = lines.filter(|line| {
                line.as_ref()
                    .map(|s| addr_modes.clone().into_keys().any(|val| s.starts_with(val)))
                    .unwrap()
            });
            c.for_each(|b| {
                lines_to_parse.push(b.unwrap().clone());
                // println!("{}", b.as_ref().unwrap());
            });
        }
        Err(_) => println!("bad"),
    }

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
        // debug printing
        // tokens
        //     .clone()
        //     .into_iter()
        //     .for_each(|f| println!("val: {} ", f));

        mode = tokens[0].to_string().clone();
        // println!(
        //     "mode 1: {} {} {}",
        //     mode,
        //     tokens[tokens.len() - 3],
        //     tokens[tokens.len() - 2]
        // );
        // match addr_modes.get(&cur_inst as &str) {
        //     Some(val) => println!("{}", val),
        //     None => panic!("None found"),
        // }
        mode = addr_modes.get(&mode as &str).unwrap().to_string();
        inst = tokens[1].to_string();
        // From parsing the file, the opcode is the third to last char, and cycle count is last
        // char
        let s_cycle_cnt: String = tokens[tokens.len() - 1].to_string();
        if (s_cycle_cnt.contains("*")) {
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
        println!("{:#?}", opcode_vec[opcode_vec.len() - 1]);
    }

    let json_str: String = String::from("./temp.json");
    dump_to_json(json_str, &opcode_vec);
}
