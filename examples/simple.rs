use std::{env::args, fs, process::exit};

use risc_v_emulator::{Bus, Cpu};

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Run <command> <path to binary>");
        exit(1);
    }
    let path = args.get(1).unwrap();
    let code = fs::read(path).expect("Error while reading binary file");
    let mut cpu = Cpu::new(Bus::new(1024 * 1024, code));
    loop {
        let ret = cpu.run_next_instruction();
        if ret.is_err() {
            println!("Error:\n{cpu}");
            break;
        }
    }
}
