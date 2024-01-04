use crate::cpu::CPU;
use crate::instruction::addressing::AddressingMode;

pub fn iny(cpu: &mut CPU, mode:&AddressingMode){
    cpu.set_register_y(cpu.register_y.wrapping_add(1));
}