use crate::cpu::CPU;
use crate::instruction::addressing::AddressingMode;

fn asl(cpu: &mut CPU, addressing_mode: &AddressingMode) {
    let address = cpu.get_operand_address(addressing_mode);
    let data = cpu.memory_read(address);
    if data >> 7 == 1 {
        cpu.set_carry_flag();
    } else {
        cpu.clear_carry_flag()
    }
    let carry_data = data << 1;
    cpu.memory_write(address, carry_data);
    cpu.update_zero_and_negative_flags(carry_data);
}