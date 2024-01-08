use crate::cpu::CPU;
use crate::instruction::addressing::AddressingMode;

pub(crate) fn ldy(cpu: &mut CPU, addressing_mode: &AddressingMode) {
    let address = cpu.get_operand_address(addressing_mode);
    cpu.set_register_y(cpu.memory_read(address));
}