use mos::emulator::CPU;
use serde_json::Value;
use std::fs::File;
use std::io::{BufReader,Read};

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

const PATH: &str = "/home/darklaw/Desktop/6502-tests/65x02/6502/v1/00.json";
#[test]
fn test1() {
    let file = File::open(PATH).expect("Unable to open file");  
    let mut reader = BufReader::new(file);
    let mut content: String = String::new();
    let _ = reader.read_to_string(&mut content);
    let v: Value = serde_json::from_str(&content).expect("Failed to parse");
    // println!{"{}", &content[..10]};
    // println!("{:#?}", v);
    if let Value::Array(arr) = v {
        for (i,obj) in arr.iter().enumerate() {
            if let Value::Object(o) = obj {
                println!("{} {}", i, o["name"]);
                for key in o.keys() {
                    println!("key: {}", key);
                }
            } else {
                eprintln!("Not an object!");
            }
            break;
        }
    } else {
        eprintln!("Not an array")
    }
}


