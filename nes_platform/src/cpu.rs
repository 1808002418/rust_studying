use crate::memory::Memory;

const PROGRAM_START_ADDRESS: u16 = 0x8000;

pub struct CPU {
    // 负载累加器
    pub register_a: u8,
    // X 寄存器
    pub register_x: u8,
    // 这个变量可能是一个字节大小的整数，其中的每个位对应一个特定的标志位。通过将特定的位设置为 1 或 0，可以表示相应的标志位状态
    // 通过按位或操作 | 和按位与操作 &，可以根据需要设置或取消设置特定的标志位，而不需要使用多个单独的变量
    pub status: u8,
    pub memory: Memory,
    // 这个相当于指令寄存器
    pub program_counter: u16,
}

/*
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

/*
CPU以恒定的周期工作：
从指令存储器中取出下一条执行指令
    解码指令
    执行指令
    重复循环
*/
impl CPU {
    pub fn new() -> Self {
        CPU { register_a: 0, register_x: 0, status: 0, memory: Memory::default(), program_counter: 0 }
    }
    pub fn interpret(&mut self) {
        loop {
            let ops_code = self.offset_program();
            match ops_code {
                // LDA
                0xA9 => {
                    let param = self.offset_program();
                    self.lda(param);
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
        }
    }
    fn offset_program(&mut self) -> u8 {
        let ops_code = self.memory_read(self.program_counter);
        self.program_counter += 1;
        return ops_code;
    }
    fn lda(&mut self, param: u8) {
        self.register_a = param;
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
        self.memory_write_u16(0xFFFC,PROGRAM_START_ADDRESS);
    }
}

impl CPU {
    /*
    复位方法应该恢复所有寄存器的状态，并用存储在0xFFFC的2字节值初始化program_counter
    */
    pub fn reset(&mut self) {
        self.register_a=0;
        self.register_x=0;
        self.status=0;
        self.program_counter=self.memory_read_u16(0xFFFC);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.memory_load_program(program);
        self.reset();
        self.interpret();
    }
}
