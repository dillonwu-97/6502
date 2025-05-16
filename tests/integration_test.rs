use mos::emulator::CPU;
use serde_json::Value;
use std::fs::File;
use std::io::{BufReader,Read};
use std::path::PathBuf;
use std::env;
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
        // this only checks the ram 
        for (j, v) in tc.initial.ram.iter().enumerate() {
            let (addr,val) = v;
            cpu.memory[ *addr ] = *val;
            // println!("Before *addr: {}, cpu val: {}", *addr, cpu.memory[*addr]);
        }
        cpu.execute();

        // Check everything include sp / pc
        println!("{}, {}, {}, {}, {}, {}", cpu.pc, cpu.sp, cpu.ac, cpu.x, cpu.y, cpu.sr.bits());
        assert_eq!(tc.r#final.pc, cpu.pc);
        assert_eq!(tc.r#final.s, cpu.sp);
        assert_eq!(tc.r#final.a, cpu.ac);
        assert_eq!(tc.r#final.x, cpu.x);
        assert_eq!(tc.r#final.y, cpu.y);
        assert_eq!(tc.r#final.p, cpu.sr.bits());
        for (j, v) in tc.r#final.ram.iter().enumerate() {
            let (addr, val) = v;
            // println!("After {}, {}, {}", *addr, cpu.memory[*addr], *val);
            assert_eq!(cpu.memory[*addr], *val);
            if cpu.memory[*addr] != *val {
                return Err(format!("CPU Memory at addr:{}, {}; Expected val: {}", addr, cpu.memory[*addr], *val));
            }
        }
    } 
    Ok(())
}


// To use:
// TESTCASE="00" cargo test --test integration_test test_single -- 
// --nocapture --quiet 
#[test]
fn test_single() -> Result<(), String> {
    let dirpath: String = "/home/darklaw/Desktop/6502-tests/65x02/6502/v1/".to_string();
    let mut testcase: String = "00".to_string();
    match env::var("TESTCASE") {
        Ok(v) => {testcase = v},
        Err(e) => {},
    }
    let filepath: String = dirpath + &testcase + ".json";
    let file = File::open(filepath).expect("Unable to open file");
    let mut reader = BufReader::new(file);
    let mut content: String = String::new();
    let _ = reader.read_to_string(&mut content);
    let deserialized:Vec<TestCase> = serde_json::from_str(&content).unwrap();
    let mut cpu: CPU = CPU::new();
    if let Err(err) = check(&mut cpu, &deserialized) {
        // eprintln!("deserialized: {:#?}", deserialized);
        // eprintln!("Failed for opcode: {}", pathname);
        return Err(format!("Failure: {}", err));
    }

    // println!("{}", testfile);
    Ok(())
}

// TODO: come up with a debug print for failed opcodes and passed opcodes
//      > so printing out more debug information from the test cases essentially
//      > solution is to create an error wrapper around the function instead
//      > and the reason this works is because if we crash out, then the ok() function is not
//      reached because of the error assert statement; from the error assert statement, we 
//      so in other words, we should be doing a better job at handling errors 
//  TODO: find a way to test each opcode individually with a command line argument or something
//  like that
//      > not sure how to specify to as a test_batch param though 
//      > workaround: write a separate testcase handler for individual testing 
//      > then add command line arguments for the individual test case
//      > specify that specific test when running integration tests
// const PATH: &str = "/home/darklaw/Desktop/6502-tests/65x02/6502/v1/a1.json";
//
// TODO: double check the start of the stack, and specifically 08 opcode
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

