use crate::cpu::CPU;
use crate::instruction::addressing::AddressingMode;

fn ldy(cpu: &mut CPU, addressing_mode: &AddressingMode) {
    let address = cpu.get_operand_address(addressing_mode);
    cpu.set_register_y(cpu.memory_read(address));
}