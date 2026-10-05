mod cpu;

use cpu::Cpu;

fn print_registers(cpu: &Cpu) {
    println!(
        "R0 = {:#010X} ({}) | R1 = {:#010X} ({}) | R2 = {:#010X} ({})",
        cpu.registers[0],
        cpu.registers[0],
        cpu.registers[1],
        cpu.registers[1],
        cpu.registers[2],
        cpu.registers[2]
    );
    println!(
        "N = {} | Z = {} | C = {} | V = {}",
        cpu.flags.negative, cpu.flags.zero, cpu.flags.carry, cpu.flags.overflow
    );
}

fn print_operation(cpu: &mut Cpu, name: &str, operation: impl FnOnce(&mut Cpu)) {
    println!("\n--- {} ---", name);
    println!("Antes:");
    print_registers(cpu);

    operation(cpu);

    println!("Depois:");
    print_registers(cpu);
}

fn main() {
    let mut cpu = Cpu::new();

    cpu.set_sp(0x0300_7F00);
    cpu.set_lr(0x0800_0100);
    cpu.set_pc(0x0800_0000);

    println!("=== Estado inicial da CPU ===");
    print_registers(&cpu);
    println!("SP = {:#010X}", cpu.sp());
    println!("LR = {:#010X}", cpu.lr());
    println!("PC = {:#010X}", cpu.pc());

    print_operation(&mut cpu, "MOV R0, #10", |cpu| {
        cpu.mov(0, 10);
    });

    print_operation(&mut cpu, "MOV R1, #20", |cpu| {
        cpu.mov(1, 20);
    });

    print_operation(&mut cpu, "ADD R2, R0, R1", |cpu| {
        cpu.add(2, 0, 1);
    });

    print_operation(&mut cpu, "SUB R2, R1, R0", |cpu| {
        cpu.sub(2, 1, 0);
    });

    println!("\n=== Estado final da CPU ===");
    print_registers(&cpu);
}
