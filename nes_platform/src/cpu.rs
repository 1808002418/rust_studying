use std::collections::HashMap;
use crate::addressing::{AddressingMode, OpCode, OPCODE_MAP};
use crate::memory::Memory;

const PROGRAM_START_ADDRESS: u16 = 0x8000;

pub struct CPU {
    // 负载累加器
    pub register_a: u8,
    // X 寄存器
    pub register_x: u8,
    // Y 寄存器
    pub register_y: u8,
    // 这个变量可能是一个字节大小的整数，其中的每个位对应一个特定的标志位。通过将特定的位设置为 1 或 0，可以表示相应的标志位状态
    // 通过按位或操作 | 和按位与操作 &，可以根据需要设置或取消设置特定的标志位，而不需要使用多个单独的变量
    pub status: u8,
    pub memory: Memory,
    // 这个相当于指令寄存器
    pub program_counter: u16,
}

/**
https://www.nesdev.org/obelisk-6502-guide/reference.html
LDA - Load Accumulator
    负载累加器,将内存字节加载到累加器中，并根据需要设置零和负标志.
    LDA #$c0
    操作码: $A9
TAX - Transfer Accumulator to X
    将累加器转移到 X 寄存器
    操作码: $AA
INX - Increment X Register
    将 X 寄存器加 1，根据需要设置零和负标志
    操作码: $E8
BRK - Force Interrupt
    强制中断
    操作码: $00
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
        CPU { register_a: 0, register_x: 0, register_y: 0, status: 0, memory: Memory::default(), program_counter: 0 }
    }
    pub fn interpret(&mut self) {
        let ref opcodes: HashMap<u8, &'static OpCode> = *OPCODE_MAP;

        loop {
            let ops_code = self.offset_program();
            let opcode = opcodes.get(&ops_code).expect(&format!("Opcode {:x} is not recognized", ops_code));

            match ops_code {
                // LDA
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.lda(&opcode.mode);
                }
                // TAX
                0xAA => {
                    self.tax();
                }
                // INX
                0xE8 => {
                    self.inx();
                }
                // BRK
                0x00 => {
                    return;
                }
                _ => {}
            }
            // 操作数偏移
            self.program_counter += opcode.operand_len as u16;
        }
    }

    /*
    通过寻址方式获取到操作数的内存地址,不负责修改程序段偏移
     */
    fn get_operand_address(&mut self, addressing_mode: &AddressingMode) -> u16 {
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
    fn lda(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_operand_address(addressing_mode);
        self.register_a = self.memory_read(address);
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }
    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }
    fn update_zero_and_negative_flags(&mut self, result: u8) {
        // 必须根据结果设置或取消设置 CPU 标志状态。
        if result == 0 {
            self.status = self.status | 0x02;
        } else {
            self.status = self.status | 0xfd;
        }

        if result & 0x80 == 0 {
            self.status = self.status & 0x7f;
        } else {
            self.status = self.status | 0x80;
        }
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
        self.status = 0;
        self.program_counter = self.memory_read_u16(0xFFFC);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.memory_load_program(program);
        self.reset();
        self.interpret();
    }
}
