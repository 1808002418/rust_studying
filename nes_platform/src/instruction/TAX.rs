use crate::cpu::CPU;
use crate::instruction::addressing::AddressingMode;

pub fn tax( cpu: &mut CPU,mode:&AddressingMode){
    cpu.set_register_x(cpu.register_a);
}