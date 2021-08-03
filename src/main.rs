mod cpu;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io;
use crate::cpu::Cpu;
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: rustrvemu filename");
    }
    let mut file = File::open(&args[1])?;
    let mut code = Vec::new();
    file.read_to_end(&mut code)?;
    let memory_size: u64 = 1024 * 1024 * 128;
    let mut cpu = Cpu::new(memory_size,code);

    while !cpu.finished() {
        cpu.step();
    }
    cpu.dump_regs();
    std::result::Result::Ok(())
}
