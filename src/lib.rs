#![allow(warnings)]
pub mod emulator;
pub mod ops {
    include!(concat!(env!("OUT_DIR"), "/opcodes.rs"));
}
