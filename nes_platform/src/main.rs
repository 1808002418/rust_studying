#![allow(unused_variables)]
#[allow(dead_code)]

mod cpu;
mod memory;


use cpu::*;

fn main() {
    println!("Hello, world!");
}



#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_lda_immediate_load_data(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a,0x05);
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
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.load_and_run(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }



}
