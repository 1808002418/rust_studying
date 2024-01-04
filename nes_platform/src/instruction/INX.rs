use crate::cpu::CPU;
use crate::instruction::addressing::AddressingMode;

pub fn inx(cpu: &mut CPU, mode: &AddressingMode) {
    cpu.set_register_x(cpu.register_x.wrapping_add(1));
}