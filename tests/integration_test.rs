use mos::emulator::CPU;
use serde_json::Value;
use std::fs::File;
use std::io::{BufReader,Read};
use std::path::PathBuf;
mod common;
use common::{TestCase};
use rstest::rstest;

// that we can create a cpu without issue, etc.
// health check serves to make sure that serialize / deserialize operations are working 
#[test] 
fn health_check() {
    let mut cpu = CPU::new(); 
    cpu.reset();


    const PATH: &str = "/home/darklaw/Desktop/6502-tests/65x02/6502/v1/00.json";
    let file = File::open(PATH).expect("Unable to open file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    let _ = buf_reader.read_to_string(&mut contents);
    let slice = &contents[..10];
    // println!("{}", slice);
    // let v: Value = serde_json::from_str(json_data).unwrap();
}

fn check(cpu: &mut CPU, testcases: &Vec<TestCase>) -> Result<(), String> {
    cpu.reset();
    for (i,tc) in testcases.iter().enumerate() {
        cpu.update(tc.initial.pc, tc.initial.a, tc.initial.x, tc.initial.y, tc.initial.p, tc.initial.s);
        for (j, v) in tc.initial.ram.iter().enumerate() {
            let (addr,val) = v;
            cpu.memory[ *addr ] = *val;
            // println!("Before {}, {}, {}", *addr, cpu.memory[*addr], *val);
        }
        cpu.execute();
        for (j, v) in tc.r#final.ram.iter().enumerate() {
            let (addr, val) = v;
            // println!("After {}, {}, {}", *addr, cpu.memory[*addr], *val);
            // assert_eq!(cpu.memory[*addr], *val);
            if cpu.memory[*addr] != *val {
                return Err(format!("CPU Memory at addr:{}, {}; Expected val: {}", addr, cpu.memory[*addr], *val));
            }
        }
    } 
    Ok(())
}

// TODO: come up with a debug print for failed opcodes and passed opcodes
//      > so printing out more debug information from the test cases essentially
//      > solution is to create an error wrapper around the function instead
//      > and the reason this works is because if we crash out, then the ok() function is not
//      reached because of the error assert statement; from the error assert statement, we 
//      so in other words, we should be doing a better job at handling errors 
const PATH: &str = "/home/darklaw/Desktop/6502-tests/65x02/6502/v1/";
// const PATH: &str = "/home/darklaw/Desktop/6502-tests/65x02/6502/v1/a1.json";
#[rstest]
fn test_batch(#[files("../6502-tests/65x02/6502/v1/*.json")] path: PathBuf) -> Result<(), String> {
    let pathname: String = path.clone().into_os_string().into_string().unwrap();
    let file = File::open(path).expect("Unable to open file");  
    let mut reader = BufReader::new(file);
    let mut content: String = String::new();
    let _ = reader.read_to_string(&mut content);
    let deserialized:Vec<TestCase> = serde_json::from_str(&content).unwrap();
    let mut cpu: CPU = CPU::new();
    if let Err(err) = check(&mut cpu, &deserialized) {
        // eprintln!("deserialized: {:#?}", deserialized);
        eprintln!("Failed for opcode: {}", pathname);
        return Err(format!("Failure: {}", err));
    }
    Ok(())
}

