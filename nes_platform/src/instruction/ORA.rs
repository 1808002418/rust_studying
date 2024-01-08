use crate::cpu::CPU;
use crate::instruction::addressing::AddressingMode;

pub(crate) fn ora(cpu: &mut CPU, addressing_mode: &AddressingMode) {
    let address = cpu.get_operand_address(addressing_mode);
    let data = cpu.memory_read(address);
    cpu.set_register_x(data | cpu.register_a);
}