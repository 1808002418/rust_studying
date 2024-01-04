use crate::cpu::CPU;
use crate::instruction::addressing::AddressingMode;

fn sbc(cpu: &mut CPU, addressing_mode: &AddressingMode) {
    let address = cpu.get_operand_address(addressing_mode);
    let data = cpu.memory_read(address);
    cpu.add_to_register_a_address(
        data.wrapping_neg().wrapping_sub(1)
    );
}