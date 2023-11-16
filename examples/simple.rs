use risc_v_emulator::{Cpu, Bus};

fn main() {
    let mut cpu = Cpu::new(Bus::new(1024 * 1024, vec![0, 0, 0, 0]));
    loop {
        let ret = cpu.run_next_instruction();
        if ret.is_err() {
            println!("Error:\n{cpu}");
            break;
        }
    }
}