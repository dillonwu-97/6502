use std::fs::File;
use std::path::Path;
use std::fs;

// Simple test program to open file, and split the file as bytes into an array of 4 byte chunks 

// interesting, the stack underflows 
//

//N	Negative
//V	Overflow
//-	ignored
//B	Break
//D	Decimal (use BCD for arithmetics)
//I	Interrupt (IRQ disable)
//Z	Zero
//C	Carry
fn emulate() {
    let pc:u16 = 0;
    let ac:u8 = 0;
    let x:u8 = 0;
    let y:u8 = 0;
    let sr:u8 = 0;
    let sp:u8 = 0;

}

fn main() {
    let path = Path::new("./data/pacman");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open file {} because {}", display, why),
        Ok(file) => file,
    };

    println!("Hello, world!");
    println!("{}", display);

    let data: Vec<u8> = match fs::read("./data/pacman") {
        Err(why) => panic!("data fail"),
        Ok(data) => data,
    };

    // naive method is to separate into 2,3,4 partitions 
    // then match with each of them 
    // do a recursive traversal algorithm?
    for (i, &item) in data.iter().enumerate() {

        println!("{}", item);

    }
    println!("print is done!");

}


