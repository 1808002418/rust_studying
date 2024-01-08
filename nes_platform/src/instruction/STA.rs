use crate::cpu::CPU;
use crate::instruction::addressing::AddressingMode;

pub(crate) fn sta(cpu: &mut CPU, addressing_mode: &AddressingMode) {
    let address = cpu.get_operand_address(addressing_mode);
    cpu.memory_write(address, cpu.register_a);
}