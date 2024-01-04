use std::collections::HashMap;
use bitflags::bitflags;
use crate::instruction::addressing::{AddressingMode, OpCode, OPCODE_MAP};
use crate::invoke_module_method;
use crate::memory::Memory;

const PROGRAM_START_ADDRESS: u16 = 0x0600;

const STACK: u16 = 0x0100;
const STACK_RESET: u8 = 0xfd;

bitflags! {
    /// # Status Register (P) http://wiki.nesdev.com/w/index.php/Status_flags
    ///
    ///  7 6 5 4 3 2 1 0
    ///  N V _ B D I Z C
    ///  | |   | | | | +--- Carry Flag  如果减法运算中没有借位，则进位标志位被设置为 1；否则，设置为 0。
    ///  | |   | | | +----- Zero Flag   如果结果为零，则将零标志位设置为 1；否则，设置为 0
    ///  | |   | | +------- Interrupt Disable
    ///  | |   | +--------- Decimal Mode (not used on NES)
    ///  | |   +----------- Break Command
    ///  | +--------------- Overflow Flag   根据减法运算的结果判断是否发生溢出。如果溢出了，则溢出标志位被设置为 1；否则，设置为 0。
    ///  +----------------- Negative Flag   将结果的最高位存储到负数标志位
    ///
    pub struct CPUFlags:u8{
        const CARRY             =0b0000_0001;
        const ZERO              =0b0000_0010;
        const INTERRUPT_DISABLE =0b0000_0100;
        const DECIMAL_MODE      =0b0000_1000;
        const BREAK             =0b0001_0000;
        const BREAK2            =0b0010_0000;
        const OVERFLOW          =0b0100_0000;
        const NEGATIV           =0b1000_0000;
    }

}

pub struct CPU {
    // 负载累加器
    pub register_a: u8,
    // X 寄存器
    pub register_x: u8,
    // Y 寄存器
    pub register_y: u8,
    // 这个变量可能是一个字节大小的整数，其中的每个位对应一个特定的标志位。通过将特定的位设置为 1 或 0，可以表示相应的标志位状态
    // 通过按位或操作 | 和按位与操作 &，可以根据需要设置或取消设置特定的标志位，而不需要使用多个单独的变量
    pub status: CPUFlags,
    pub memory: Memory,
    pub stack_pointer: u8,
    // 这个相当于指令寄存器
    pub program_counter: u16,
}

/**
https://www.nesdev.org/obelisk-6502-guide/reference.html
ADC - Add with Carry
    加进位,该指令将存储单元的内容与进位位一起添加到累加器中。如果发生溢出，则设置进位位，这使得能够执行多字节加法。
    ADC #10 向存储累加器加10
    操作码 $69
AND - Logical AND
    逻辑 AND 是使用内存字节的内容逐位对累加器内容执行的。
ORA - Logical Inclusive OR
    使用内存字节的内容对累加器内容逐位执行包含 OR
EOR - Exclusive OR
    使用内存字节的内容对累加器内容逐位执行独占 OR(异或)。
    将操作数与累加器（A 寄存器）进行异或运算,将运算结果存储回累加器。
LDA - Load Accumulator
    负载累加器,将内存字节加载到累加器中，并根据需要设置零和负标志.
    LDA #$c0
    操作码: $A9
LDY - Load Y Register
    将一个字节的内存加载到 Y 寄存器中，并根据需要设置零和负标志。
LDX - Load X Register
    将一个字节的内存加载到 X 寄存器中，并根据需要设置零和负标志。
TAX - Transfer Accumulator to X
    将累加器转移到 X 寄存器
    操作码: $AA
INX - Increment X Register
    将 X 寄存器加 1，根据需要设置零和负标志
    操作码: $E8
INY - Increment Y Register
    将 Y 寄存器加 1，根据需要设置零和负标志
BRK - Force Interrupt
    强制中断
    操作码: $00
STA - Store Accumulator
    存储累加器(负载累加器),将累加器的内容存储到内存中。
    STA $2000 存储累加器的值到绝对地址 $2000
    操作码: $85
ASL - Arithmetic Shift Left
    算术左移.此操作将累加器或存储器内容的所有位向左移动一位。
    位 0 设置为 0，位 7 放置在进位标志中。此操作的效果是将内存内容乘以 2（忽略 2 的补码考虑），如果结果不适合 8 位，则设置进位。
    ASL A     ; 将累加器 A 的值左移一位
    ASL $1234 ; 将内存地址 $1234 处的值左移一位
    ASL X     ; 将变量 X 的值左移一位
LSR - Logical Shift Right
    A 或 M 中的每个位都向右移动一位。位 0 中的位被移入进位标志。位 7 设置为零。
SBC - Subtract with Carry
    该指令将内存位置的内容连同进位的位数一起减到累加器中。如果发生溢出，进位清除，这允许执行多字节减法。
    从累加器（A 寄存器）中减去操作数的值以及进位标志位（C）的值,将结果存储回累加器
 */


/**
CPU以恒定的周期工作：
从指令存储器中取出下一条执行指令
    解码指令
    执行指令
    重复循环
 */
impl CPU {
    pub fn new() -> Self {
        CPU { register_a: 0, register_x: 0, register_y: 0, status: CPUFlags::from_bits_truncate(0b0010_0100), memory: Memory::default(), stack_pointer: STACK_RESET, program_counter: 0 }
    }
    pub fn interpret(&mut self) {
        let ref opcodes: HashMap<u8, &'static OpCode> = *OPCODE_MAP;

        loop {
            let ops_code = self.offset_program();
            let opcode = opcodes.get(&ops_code).expect(&format!("Opcode {:x} is not recognized", ops_code));
            invoke_module_method!(
                opcode.mnemonic,
                opcode.mnemonic.to_lowercase(),
                self,
                &opcode.mode
            );
            /*            match ops_code {
                            // ADC
                            0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                                self.adc(&opcode.mode);
                            }
                            // LDA
                            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                                self.lda(&opcode.mode);
                            }
                            // LDA
                            0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
                                self.ldy(&opcode.mode);
                            }
                            // LDX
                            0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
                                self.ldx(&opcode.mode);
                            }
                            // ORA
                            0x09| 0x05| 0x15| 0x0D| 0x1D| 0x19| 0x01| 0x11=>{
                                self.ora(&opcode.mode);
                            }
                            // AND
                            0x29| 0x25| 0x35| 0x2D| 0x3D| 0x39| 0x21| 0x31=>{
                                self.and(&opcode.mode);
                            }
                            // EOR
                            0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => {
                                self.eor(&opcode.mode);
                            }
                            // STA
                            0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                                self.sta(&opcode.mode);
                            }
                            // ASL
                            0x0A | 0x06 | 0x16 | 0x0E | 0x1E => {
                                self.asl(&opcode.mode);
                            }
                            // SBC
                            0xE9| 0xE5| 0xF5| 0xED| 0xFD| 0xF9| 0xE1| 0xF1=>{
                                self.sbc(&opcode.mode);
                            }
                            // TAX
                            0xAA => {
                                self.tax();
                            }
                            // INX
                            0xE8 => {
                                self.inx();
                            }
                            // INY
                            0xC8 => {
                                self.iny();
                            }
                            // BRK
                            0x00 => {
                                return;
                            }
                            _ => {}
                        }*/
            // 操作数偏移
            self.program_counter += opcode.operand_len as u16;
        }
    }

    /*
    通过寻址方式获取到操作数的内存地址,不负责修改程序段偏移
     */
    pub fn get_operand_address(&mut self, addressing_mode: &AddressingMode) -> u16 {
        return match addressing_mode {
            /*
            Immediate: 立即寻址模式。操作数直接包含在指令中，例如：LDA #10，表示将值10加载到累加器（Accumulator）寄存器中。
            操作数地址为指令的下一个字节
             */
            AddressingMode::Immediate => { self.program_counter }
            /*
            ZeroPage: 零页寻址模式。操作数的地址位于零页（地址范围为0x0000-0x00FF）内，只需一个字节来表示地址。例如：LDA $45，表示将地址为0x45的内存单元的值加载到累加器寄存器中。
             */
            AddressingMode::ZeroPage => { self.memory_read(self.program_counter) as u16 }
            /*
            ZeroPage_X: 零页X变址寻址模式。操作数的地址为零页内的一个字节，而X寄存器的值会被加到这个地址上。例如：LDX $25,X，表示将地址为0x25+X的内存单元的值加载到X寄存器中。
             */
            AddressingMode::ZeroPage_X => {
                let pos = self.memory_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                addr
            }
            /*
            ZeroPage_Y: 零页Y变址寻址模式。操作数的地址为零页内的一个字节，而Y寄存器的值会被加到这个地址上。例如：LDY $30,Y，表示将地址为0x30+Y的内存单元的值加载到Y寄存器中。
             */
            AddressingMode::ZeroPage_Y => { self.memory_read(self.program_counter).wrapping_add(self.register_y) as u16 }
            /*
            Absolute: 绝对寻址模式。操作数的地址通过一个完整的地址表示。例如：LDA $2000，表示将地址为0x2000的内存单元的值加载到累加器寄存器中。
            操作数地址为指令的下两个字节
             */
            AddressingMode::Absolute => { self.memory_read_u16(self.program_counter) }
            /*
            Absolute_X: 绝对X变址寻址模式。操作数的地址为一个完整的地址，而X寄存器的值会被加到这个地址上。例如：STA $3000,X，表示将累加器寄存器的值存储到地址为0x3000+X的内存单元中。
             */
            AddressingMode::Absolute_X => { self.memory_read_u16(self.program_counter).wrapping_add(self.register_x as u16) }
            /*
            绝对Y变址寻址模式。操作数的地址为一个完整的地址，而Y寄存器的值会被加到这个地址上。例如：STA $4000,Y，表示将累加器寄存器的值存储到地址为0x4000+Y的内存单元中。
             */
            AddressingMode::Absolute_Y => { self.memory_read_u16(self.program_counter).wrapping_add(self.register_y as u16) }
            /*
            间接X变址寻址模式。操作数的地址通过一个间接寻址的方式计算得到。首先，将一个字节与X寄存器相加得到一个地址，然后使用这个地址作为间接寻址的目标地址。
            例如：JMP ($20,X)，表示通过将0x20+X得到的地址所指向的内存单元中的值作为新的指令地址，实现间接跳转。
             */
            AddressingMode::Indirect_X => {
                let base = self.memory_read(self.program_counter);
                let ptr = base.wrapping_add(self.register_x);
                // 获取低位字节
                let lo = self.memory_read(ptr as u16);
                // +1 偏移到下一个字节所在位置
                // 获取高位字节
                let hi = self.memory_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | lo as u16
            }
            /*
            间接Y变址寻址模式。操作数的地址通过一个间接寻址的方式计算得到。首先，使用一个地址作为间接寻址的目标地址，然后将这个地址与Y寄存器相加得到最终的地址。
            例如：STA ($30),Y，表示将累加器寄存器的值存储到以地址0x30为间接寻址目标，再加上Y寄存器的值得到的最终地址所指向的内存单元中。
             */
            AddressingMode::Indirect_Y => {
                let base = self.memory_read(self.program_counter);
                // 这个地方涉及到进位,所以不能直接用read_u16
                let lo = self.memory_read(base as u16);
                let hi = self.memory_read(base.wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | lo as u16;
                let deref = deref_base.wrapping_add(self.register_y as u16);
                deref
            }
            /*
            无寻址模式。表示该指令没有操作数，或者操作数不需要通过寻址方式获取。
             */
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", addressing_mode);
            }
        };
    }

    fn offset_program(&mut self) -> u8 {
        let ops_code = self.memory_read(self.program_counter);
        self.program_counter += 1;
        return ops_code;
    }

    pub fn add_to_register_a_address(&mut self, data: u8) {
        let sum = self.register_a as u16 + data as u16 +
            (   // 进位检测
                if self.status.contains(CPUFlags::CARRY) {
                    1
                } else {
                    0
                }
            ) as u16;

        // 进位标志
        if sum > 0xff {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        // 截断进位数据
        let result = sum as u8;
        // &的优先级是比!=高的  加括号是别误解了
        /*
        a:              1111_1110
        data:           0000_1111
        sum:       0001_0000_1101
        result:         0000_1101
        data ^ result   0000_0010   1
        result ^ a      1111_0011   2
        1 & 2           0000_0010   3
        3 & 0x80        0000_0000

         */
        if ((data ^ result) & (result ^ self.register_a) & 0x80) != 0 {
            self.set_overflow_flag();
        } else {
            self.clear_overflow_flag();
        }

        self.set_register_a(result);
    }

    pub fn update_zero_and_negative_flags(&mut self, result: u8) {
        // 必须根据结果设置或取消设置 CPU 标志状态。
        if result == 0 {
            self.set_zero_flag();
        } else {
            self.clear_zero_flag()
        }

        if result & 0x80 == 0 {
            self.clear_negative_flag()
        } else {
            self.set_negative_flag();
        }
    }
}

impl CPU {
    pub fn set_carry_flag(&mut self) {
        self.status.insert(CPUFlags::CARRY);
    }
    pub fn clear_carry_flag(&mut self) {
        self.status.remove(CPUFlags::CARRY);
    }

    pub fn set_zero_flag(&mut self) {
        self.status.insert(CPUFlags::ZERO);
    }

    pub fn clear_zero_flag(&mut self) {
        self.status.remove(CPUFlags::ZERO);
    }

    pub fn set_overflow_flag(&mut self) {
        self.status.insert(CPUFlags::OVERFLOW);
    }

    pub fn clear_overflow_flag(&mut self) {
        self.status.remove(CPUFlags::OVERFLOW);
    }

    pub fn set_negative_flag(&mut self) {
        self.status.insert(CPUFlags::NEGATIV);
    }

    pub fn clear_negative_flag(&mut self) {
        self.status.remove(CPUFlags::NEGATIV);
    }
}

impl CPU {
    pub fn set_register_a(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub fn set_register_y(&mut self, value: u8) {
        self.register_y = value;
        self.update_zero_and_negative_flags(self.register_y);
    }

    pub fn set_register_x(&mut self, value: u8) {
        self.register_x = value;
        self.update_zero_and_negative_flags(self.register_x);
    }
}

impl CPU {
    pub fn memory_read(&self, addr: u16) -> u8 {
        return self.memory.read(addr);
    }

    pub fn memory_write(&mut self, addr: u16, data: u8) {
        self.memory.write(addr, data);
    }

    /*
    小端顺序读
    */
    pub fn memory_read_u16(&mut self, pos: u16) -> u16 {
        let lo = self.memory_read(pos) as u16;
        let hi = self.memory_read(pos + 1) as u16;
        (hi << 8) | lo
    }

    /*
    小端顺序写
    */
    pub fn memory_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.memory_write(pos, lo);
        self.memory_write(pos + 1, hi);
    }

    /*
    load 方法应将程序加载到 PRG ROM 空间并将代码引用保存到 0xFFFC 存储单元中
    */
    pub fn memory_load_program(&mut self, program: Vec<u8>) {
        self.memory.load_program(PROGRAM_START_ADDRESS, program);
        // 设置指令寄存器为程序的起始地址
        // self.program_counter = PROGRAM_START_ADDRESS;
        self.memory_write_u16(0xFFFC, PROGRAM_START_ADDRESS);
    }
}

impl CPU {
    /*
    复位方法应该恢复所有寄存器的状态，并用存储在0xFFFC的2字节值初始化program_counter
    */
    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = CPUFlags::from_bits_truncate(0b0010_0100);
        self.program_counter = self.memory_read_u16(0xFFFC);
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
        where F: FnMut(&mut CPU) {
        let ref opcodes: HashMap<u8, &'static OpCode> = *OPCODE_MAP;
        loop {
            callback(self);
            // match code {  }
        }
    }


    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.memory_load_program(program);
        self.reset();
        self.interpret();
    }
}

impl CPU {
    pub fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        return self.memory_read(STACK + self.stack_pointer as u16);
    }

    pub fn stack_push(&mut self, data: u8) {
        // 从高地址往低地址写
        self.memory_write(STACK + self.stack_pointer as u16, data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    pub fn stack_pop_u16(&mut self) -> u16 {
        let lo = self.stack_pop() as u16;
        let hi = self.stack_pop() as u16;
        return (hi << 8) | lo;
    }
    pub fn stack_push_u16(&mut self, data: u16) {
        let hi = (data >> 8) as u8;
        let oi = (data & 0xff) as u8;
        self.stack_push(hi);
        self.stack_push(oi);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.status.contains(CPUFlags::ZERO), false);
        assert_eq!(cpu.status.contains(CPUFlags::NEGATIV), false);
    }

    #[test]
    fn test_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert_eq!(cpu.status.contains(CPUFlags::ZERO), true);
    }

    #[test]
    fn test_sta_immediate() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x01, 0x85, 0xff, 0x00]);
        assert_eq!(cpu.memory_read(0xff), 0x01);
    }

    #[test]
    fn test_sta_absolute_x() {
        let mut cpu = CPU::new();
        /*
        0xa9 0x10 向负载累加器写入0x10
        0xaa      将负载累加器的值负责到X寄存器
        0x9d 0x00 0xff  将负载累加器的值复制到0xff10的内存位置
        */
        // 多字节操作数要按小端顺序写入                         // 这两个操作数要按小端顺序写入
        cpu.load_and_run(vec![0xa9, 0x10, 0xaa, 0x9d, 0x00, 0xff, 0x00]);

        assert_eq!(cpu.memory_read(0xff10), 0x10);
    }

    #[test]
    fn test_inx_overflow() {
        // 越界测试
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0);
        assert_eq!(cpu.status.contains(CPUFlags::ZERO), true);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_write_u16() {
        let mut cpu = CPU::new();
        cpu.memory_write_u16(0xff00, 0x1234);
        assert_eq!(cpu.memory.read(0xff00), 0x34);
        assert_eq!(cpu.memory.read(0xff01), 0x12);
    }

    #[test]
    fn test_read_u16() {
        let x = 0x1234u16;
        let mut cpu = CPU::new();
        cpu.memory.write(0xff00, 0x34);
        cpu.memory.write(0xff01, 0x12);
        assert_eq!(cpu.memory_read_u16(0xff00), 0x1234);
    }
}

#[cfg(test)]
mod test_stack {
    use super::*;

    #[test]
    fn test_pop() {
        let mut cpu = CPU::new();

        let mut top = STACK + STACK_RESET as u16;
        cpu.memory_write(top, 0x12);
        top -= 1;
        cpu.memory_write(top, 0x13);
        top -= 1;
        cpu.stack_pointer = top as u8;
        assert_eq!(cpu.stack_pop(), 0x13);
        assert_eq!(cpu.stack_pop(), 0x12);
    }

    #[test]
    fn test_push() {
        let mut cpu = CPU::new();
        cpu.stack_push(100);
        cpu.stack_push(50);
        assert_eq!(cpu.stack_pop(), 50);
        assert_eq!(cpu.stack_pop(), 100);
    }

    #[test]
    fn test_pop_u16() {
        let mut cpu = CPU::new();
        let mut top = STACK + STACK_RESET as u16;
        cpu.memory_write_u16(top, 0x1234);
        top -= 1;
        cpu.stack_pointer = top as u8;
        assert_eq!(cpu.stack_pop_u16(), 0x1234);
    }
}