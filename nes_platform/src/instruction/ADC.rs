use crate::cpu::CPU;
use crate::instruction::addressing::AddressingMode;

pub fn adc(cpu: &mut CPU, addressing_mode: &AddressingMode) {
    let address = cpu.get_operand_address(addressing_mode);
    let val = cpu.memory_read(address);
    cpu.add_to_register_a_address(val);
}