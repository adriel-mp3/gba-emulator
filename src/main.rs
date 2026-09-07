mod cpu;

use cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();

    cpu.registers[0] = 10;
    cpu.registers[1] = 20;

    cpu.set_sp(0x0300_7F00);
    cpu.set_lr(0x0800_0100);
    cpu.set_pc(0x0800_0000);

    cpu.flags.zero = true;

    println!("R0 = {}", cpu.registers[0]);
    println!("R1 = {}", cpu.registers[1]);

    println!("SP = {:#010X}", cpu.sp());
    println!("LR = {:#010X}", cpu.lr());
    println!("PC = {:#010X}", cpu.pc());

    println!("Z = {}", cpu.flags.zero);
}