#![allow(unused_variables)]

#[allow(dead_code)]
mod cpu;
mod memory;
mod addressing;


use cpu::*;

/**
https://bugzmanov.github.io/nes_ebook/chapter_1.html
 */
fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.status & 0b0000_0010, 0b00);
        assert_eq!(cpu.status & 0b1000_0000, 0);
    }

    #[test]
    fn test_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert_eq!(cpu.status & 0b0000_0010, 0b10);
    }

    #[test]
    fn test_sta_immediate() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x01,0x85, 0xff, 0x00]);
        assert_eq!(cpu.memory_read(0xff),0x01 );
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
        cpu.load_and_run(vec![0xa9, 0x10,0xaa,0x9d,0x00,0xff, 0x00]);

        assert_eq!(cpu.memory_read(0xff10),0x10 );
    }

    #[test]
    fn test_inx_overflow() {
        // 越界测试
        let mut cpu = CPU::new();
        let mut program = Vec::new();
        for i in 0..0xff {
            program.push(0xE8);
        }
        program.push(0xE8);
        program.push(0x00);
        cpu.load_and_run(program);

        assert_eq!(cpu.register_x, 0);
        assert_eq!(cpu.status, 127)
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
