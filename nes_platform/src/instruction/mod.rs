#![allow(non_snake_case)]

use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::cpu::CPU;
use crate::instruction::addressing::{AddressingMode, OpCode};

pub struct InstructionBuiltin {
    pub op: OpCode,
    pub execute: fn(&mut CPU, &AddressingMode),
}

impl InstructionBuiltin {
    fn new(op: OpCode, execute: fn(&mut CPU, &AddressingMode)) -> Self {
        return InstructionBuiltin { op, execute };
    }
}
lazy_static! {
        pub static ref CPU_INSTRUCTION_BUILTIN:Vec<InstructionBuiltin>=vec![
        InstructionBuiltin::new(OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),|cpu, mode|{}),
        InstructionBuiltin::new(OpCode::new(0xAA, "TAX", 1, 2, AddressingMode::NoneAddressing),TAX::tax),
        InstructionBuiltin::new(OpCode::new(0xE8, "INX", 1, 2, AddressingMode::NoneAddressing),INX::inx),
        InstructionBuiltin::new(OpCode::new(0xC8, "INY", 1, 2, AddressingMode::NoneAddressing),INY::iny),
        InstructionBuiltin::new(OpCode::new(0xe6, "INC", 2, 5, AddressingMode::ZeroPage),INC::inc),
        InstructionBuiltin::new(OpCode::new(0xf6, "INC", 2, 6, AddressingMode::ZeroPage_X),INC::inc),
        InstructionBuiltin::new(OpCode::new(0xee, "INC", 3, 6, AddressingMode::Absolute),INC::inc),
        InstructionBuiltin::new(OpCode::new(0xfe, "INC", 3, 7, AddressingMode::Absolute_X),INC::inc),
        InstructionBuiltin::new(OpCode::new(0xe6, "INC", 2, 5, AddressingMode::ZeroPage),INC::inc),
        InstructionBuiltin::new(OpCode::new(0xf6, "INC", 2, 6, AddressingMode::ZeroPage_X),INC::inc),
        InstructionBuiltin::new(OpCode::new(0xee, "INC", 3, 6, AddressingMode::Absolute),INC::inc),
        InstructionBuiltin::new(OpCode::new(0xfe, "INC", 3, 7, AddressingMode::Absolute_X),INC::inc),

        InstructionBuiltin::new(OpCode::new(0xc6, "DEC", 2, 5, AddressingMode::ZeroPage),DEC::dec),
        InstructionBuiltin::new(OpCode::new(0xd6, "DEC", 2, 6, AddressingMode::ZeroPage_X),DEC::dec),
        InstructionBuiltin::new(OpCode::new(0xce, "DEC", 3, 6, AddressingMode::Absolute),DEC::dec),
        InstructionBuiltin::new(OpCode::new(0xde, "DEC", 3, 7, AddressingMode::Absolute_X),DEC::dec),

        InstructionBuiltin::new(OpCode::new(0xca, "DEX", 1, 2, AddressingMode::NoneAddressing),DEX::dex),
        InstructionBuiltin::new(OpCode::new(0x88, "DEY", 1, 2, AddressingMode::NoneAddressing),DEY::dey),
        InstructionBuiltin::new(OpCode::new(0xc9, "CMP", 2, 2, AddressingMode::Immediate),CMP::cmp),
        InstructionBuiltin::new(OpCode::new(0xc5, "CMP", 2, 3, AddressingMode::ZeroPage),CMP::cmp),
        InstructionBuiltin::new(OpCode::new(0xd5, "CMP", 2, 4, AddressingMode::ZeroPage_X),CMP::cmp),
        InstructionBuiltin::new(OpCode::new(0xcd, "CMP", 3, 4, AddressingMode::Absolute),CMP::cmp),
        InstructionBuiltin::new(OpCode::new(0xdd, "CMP", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_X),CMP::cmp),
        InstructionBuiltin::new(OpCode::new(0xd9, "CMP", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_Y),CMP::cmp),
        InstructionBuiltin::new(OpCode::new(0xc1, "CMP", 2, 6, AddressingMode::Indirect_X),CMP::cmp),
        InstructionBuiltin::new(OpCode::new(0xd1, "CMP", 2, 5/*+1 if page crossed*/, AddressingMode::Indirect_Y),CMP::cmp),
        InstructionBuiltin::new(OpCode::new(0xc0, "CPY", 2, 2, AddressingMode::Immediate),CPY::cpy),
        InstructionBuiltin::new(OpCode::new(0xc4, "CPY", 2, 3, AddressingMode::ZeroPage),CPY::cpy),
        InstructionBuiltin::new(OpCode::new(0xcc, "CPY", 3, 4, AddressingMode::Absolute),CPY::cpy),
        InstructionBuiltin::new(OpCode::new(0xe0, "CPX", 2, 2, AddressingMode::Immediate),CPX::cpx),
        InstructionBuiltin::new(OpCode::new(0xe4, "CPX", 2, 3, AddressingMode::ZeroPage),CPX::cpx),
        InstructionBuiltin::new(OpCode::new(0xec, "CPX", 3, 4, AddressingMode::Absolute),CPX::cpx),
        //AddressingMode that acts as Immediate
        InstructionBuiltin::new(OpCode::new(0x4c, "JMP", 3, 3, AddressingMode::NoneAddressing), JMP::jmp),
        //AddressingMode:Indirect with 6502 bug
        InstructionBuiltin::new(OpCode::new(0x6c, "JMP", 3, 5, AddressingMode::NoneAddressing), JMP::jmp),
        InstructionBuiltin::new(OpCode::new(0x20, "JSR", 3, 6, AddressingMode::NoneAddressing),JSR::jsr),
        InstructionBuiltin::new(OpCode::new(0x60, "RTS", 1, 6, AddressingMode::NoneAddressing),RIS::ris),
        InstructionBuiltin::new(OpCode::new(0x40, "RTI", 1, 6, AddressingMode::NoneAddressing),RTI::rti),
        InstructionBuiltin::new(OpCode::new(0xA9, "LDA", 2, 2, AddressingMode::Immediate),LDA::lda),
        InstructionBuiltin::new(OpCode::new(0xA5, "LDA", 2, 3, AddressingMode::ZeroPage),LDA::lda),
        InstructionBuiltin::new(OpCode::new(0xB5, "LDA", 2, 4, AddressingMode::ZeroPage_X),LDA::lda),
        InstructionBuiltin::new(OpCode::new(0xAD, "LDA", 3, 4, AddressingMode::Absolute),LDA::lda),
        /*
        "Absolute_X"：使用绝对地址（16 位）和 X 寄存器的值来计算内存地址。X 寄存器的值会与绝对地址相加，得到最终的内存地址。

        "+1 if page crossed"：如果在计算得到的内存地址跨越了页（page），则将最终的内存地址再加上 1。

        在 6502 微处理器中，内存被划分为多个页，每个页的大小为 256 字节。当使用 "Absolute_X" 寻址模式时，如果计算得到的内存地址跨越了当前页的边界，就会发生页跨越（page crossing）。
        为了处理页跨越情况，需要将最终的内存地址再加上 1。
        */
        InstructionBuiltin::new(OpCode::new(0xBD, "LDA", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_X),LDA::lda),
        InstructionBuiltin::new(OpCode::new(0xB9, "LDA", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_Y),LDA::lda),
        InstructionBuiltin::new(OpCode::new(0xA1, "LDA", 2, 6, AddressingMode::Indirect_X),LDA::lda),
        InstructionBuiltin::new(OpCode::new(0xB1, "LDA", 2, 5/*+1 if page crossed*/, AddressingMode::Indirect_Y),LDA::lda),
        InstructionBuiltin::new(OpCode::new(0xA0,"LDY",2,2,AddressingMode::Immediate),LDY::ldy),
        InstructionBuiltin::new(OpCode::new(0xA4,"LDY",2,3,AddressingMode::ZeroPage),LDY::ldy),
        InstructionBuiltin::new(OpCode::new(0xB4,"LDY",2,4,AddressingMode::ZeroPage_X),LDY::ldy),
        InstructionBuiltin::new(OpCode::new(0xAC,"LDY",3,4,AddressingMode::Absolute),LDY::ldy),
        InstructionBuiltin::new(OpCode::new(0xBC,"LDY",3,4,AddressingMode::Absolute_X),LDY::ldy),
        InstructionBuiltin::new(OpCode::new(0xA2,"LDX",2,2,AddressingMode::Immediate),LDX::ldx),
        InstructionBuiltin::new(OpCode::new(0xA6,"LDX",2,3,AddressingMode::ZeroPage),LDX::ldx),
        InstructionBuiltin::new(OpCode::new(0xB6,"LDX",2,4,AddressingMode::ZeroPage_Y),LDX::ldx),
        InstructionBuiltin::new(OpCode::new(0xAE,"LDX",3,4,AddressingMode::Absolute),LDX::ldx),
        InstructionBuiltin::new(OpCode::new(0xBE,"LDX",3,4,AddressingMode::Absolute_Y),LDX::ldx),
        InstructionBuiltin::new(OpCode::new(0x29,"AND",2,2,AddressingMode::Immediate),AND::and),
        InstructionBuiltin::new(OpCode::new(0x25,"AND",2,3,AddressingMode::ZeroPage),AND::and),
        InstructionBuiltin::new(OpCode::new(0x35,"AND",2,4,AddressingMode::ZeroPage_X),AND::and),
        InstructionBuiltin::new(OpCode::new(0x2D,"AND",3,4,AddressingMode::Absolute),AND::and),
        InstructionBuiltin::new(OpCode::new(0x3D,"AND",3,4,AddressingMode::Absolute_X),AND::and),
        InstructionBuiltin::new(OpCode::new(0x39,"AND",3,4,AddressingMode::Absolute_Y),AND::and),
        InstructionBuiltin::new(OpCode::new(0x21,"AND",2,6,AddressingMode::Indirect_X),AND::and),
        InstructionBuiltin::new(OpCode::new(0x31,"AND",2,5,AddressingMode::Indirect_Y),AND::and),
        InstructionBuiltin::new(OpCode::new(0x09,"ORA",2,2,AddressingMode::Immediate),ORA::ora),
        InstructionBuiltin::new(OpCode::new(0x05,"ORA",2,3,AddressingMode::ZeroPage),ORA::ora),
        InstructionBuiltin::new(OpCode::new(0x15,"ORA",2,4,AddressingMode::ZeroPage_X),ORA::ora),
        InstructionBuiltin::new(OpCode::new(0x0D,"ORA",3,4,AddressingMode::Absolute),ORA::ora),
        InstructionBuiltin::new(OpCode::new(0x1D,"ORA",3,4,AddressingMode::Absolute_X),ORA::ora),
        InstructionBuiltin::new(OpCode::new(0x19,"ORA",3,4,AddressingMode::Absolute_Y),ORA::ora),
        InstructionBuiltin::new(OpCode::new(0x01,"ORA",2,6,AddressingMode::Indirect_X),ORA::ora),
        InstructionBuiltin::new(OpCode::new(0x11,"ORA",2,5,AddressingMode::Indirect_Y),ORA::ora),
        InstructionBuiltin::new(OpCode::new(0x49,"EOR",2,2,AddressingMode::Immediate),EOR::eor),
        InstructionBuiltin::new(OpCode::new(0x45,"EOR",2,3,AddressingMode::ZeroPage),EOR::eor),
        InstructionBuiltin::new(OpCode::new(0x55,"EOR",2,4,AddressingMode::ZeroPage_Y),EOR::eor),
        InstructionBuiltin::new(OpCode::new(0x4D,"EOR",3,4,AddressingMode::Absolute),EOR::eor),
        InstructionBuiltin::new(OpCode::new(0x5D,"EOR",3,4,AddressingMode::Absolute_X),EOR::eor),
        InstructionBuiltin::new(OpCode::new(0x59,"EOR",3,4,AddressingMode::Absolute_Y),EOR::eor),
        InstructionBuiltin::new(OpCode::new(0x41,"EOR",2,6,AddressingMode::Indirect_X),EOR::eor),
        InstructionBuiltin::new(OpCode::new(0x51,"EOR",2,5,AddressingMode::Indirect_Y),EOR::eor),
        InstructionBuiltin::new(OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),STA::sta),
        InstructionBuiltin::new(OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X),STA::sta),
        InstructionBuiltin::new(OpCode::new(0x8D, "STA", 3, 4, AddressingMode::Absolute),STA::sta),
        InstructionBuiltin::new(OpCode::new(0x9D, "STA", 3, 5, AddressingMode::Absolute_X),STA::sta),
        InstructionBuiltin::new(OpCode::new(0x99, "STA", 3, 5, AddressingMode::Absolute_Y),STA::sta),
        InstructionBuiltin::new(OpCode::new(0x81, "STA", 2, 6, AddressingMode::Indirect_X),STA::sta),
        InstructionBuiltin::new(OpCode::new(0x91, "STA", 2, 6, AddressingMode::Indirect_Y),STA::sta),
        InstructionBuiltin::new(OpCode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate),ADC::adc),
        InstructionBuiltin::new(OpCode::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage),ADC::adc),
        InstructionBuiltin::new(OpCode::new(0x75, "ADC", 2, 4, AddressingMode::ZeroPage_X),ADC::adc),
        InstructionBuiltin::new(OpCode::new(0x6D, "ADC", 3, 4, AddressingMode::Absolute),ADC::adc),
        InstructionBuiltin::new(OpCode::new(0x7D, "ADC", 3, 4, AddressingMode::Absolute_X),ADC::adc),
        InstructionBuiltin::new(OpCode::new(0x79, "ADC", 3, 4, AddressingMode::Absolute_Y),ADC::adc),
        InstructionBuiltin::new(OpCode::new(0x61, "ADC", 2, 6, AddressingMode::Indirect_X),ADC::adc),
        InstructionBuiltin::new(OpCode::new(0x71, "ADC", 2, 5, AddressingMode::Indirect_Y),ADC::adc),
        InstructionBuiltin::new(OpCode::new(0x2a, "ROL", 1, 2, AddressingMode::NoneAddressing),ROL::rol),
        InstructionBuiltin::new(OpCode::new(0x26, "ROL", 2, 5, AddressingMode::ZeroPage),ROL::rol),
        InstructionBuiltin::new(OpCode::new(0x36, "ROL", 2, 6, AddressingMode::ZeroPage_X),ROL::rol),
        InstructionBuiltin::new(OpCode::new(0x2e, "ROL", 3, 6, AddressingMode::Absolute),ROL::rol),
        InstructionBuiltin::new(OpCode::new(0x3e, "ROL", 3, 7, AddressingMode::Absolute_X),ROL::rol),
        InstructionBuiltin::new(OpCode::new(0x6a, "ROR", 1, 2, AddressingMode::NoneAddressing),ROR::ror),
        InstructionBuiltin::new(OpCode::new(0x66, "ROR", 2, 5, AddressingMode::ZeroPage),ROR::ror),
        InstructionBuiltin::new(OpCode::new(0x76, "ROR", 2, 6, AddressingMode::ZeroPage_X),ROR::ror),
        InstructionBuiltin::new(OpCode::new(0x6e, "ROR", 3, 6, AddressingMode::Absolute),ROR::ror),
        InstructionBuiltin::new(OpCode::new(0x7e, "ROR", 3, 7, AddressingMode::Absolute_X),ROR::ror),
        InstructionBuiltin::new(OpCode::new(0x0A,"ASL",1,2,AddressingMode::NoneAddressing),ASL::asl),
        InstructionBuiltin::new(OpCode::new(0x06,"ASL",2,5,AddressingMode::ZeroPage),ASL::asl),
        InstructionBuiltin::new(OpCode::new(0x16,"ASL",2,6,AddressingMode::ZeroPage_X),ASL::asl),
        InstructionBuiltin::new(OpCode::new(0x0E,"ASL",3,6,AddressingMode::Absolute),ASL::asl),
        InstructionBuiltin::new(OpCode::new(0x1E,"ASL",3,7,AddressingMode::Absolute_X),ASL::asl),
        InstructionBuiltin::new(OpCode::new(0x4A,"LSR",1,2,AddressingMode::NoneAddressing),LSR::lsr),
        InstructionBuiltin::new(OpCode::new(0x46,"LSR",2,5,AddressingMode::ZeroPage),LSR::lsr),
        InstructionBuiltin::new(OpCode::new(0x56,"LSR",2,6,AddressingMode::ZeroPage_X),LSR::lsr),
        InstructionBuiltin::new(OpCode::new(0x4E,"LSR",3,6,AddressingMode::Absolute),LSR::lsr),
        InstructionBuiltin::new(OpCode::new(0x5E,"LSR",3,7,AddressingMode::Absolute_X),LSR::lsr),
        InstructionBuiltin::new(OpCode::new(0xE9,"SBC",2,2,AddressingMode::Immediate),SBC::sbc),
        InstructionBuiltin::new(OpCode::new(0xE5,"SBC",2,3,AddressingMode::ZeroPage),SBC::sbc),
        InstructionBuiltin::new(OpCode::new(0xF5,"SBC",2,4,AddressingMode::ZeroPage_X),SBC::sbc),
        InstructionBuiltin::new(OpCode::new(0xED,"SBC",3,4,AddressingMode::Absolute),SBC::sbc),
        InstructionBuiltin::new(OpCode::new(0xFD,"SBC",3,4,AddressingMode::Absolute_X),SBC::sbc),
        InstructionBuiltin::new(OpCode::new(0xF9,"SBC",3,4,AddressingMode::Absolute_Y),SBC::sbc),
        InstructionBuiltin::new(OpCode::new(0xE1,"SBC",2,6,AddressingMode::Indirect_X),SBC::sbc),
        InstructionBuiltin::new(OpCode::new(0xF1,"SBC",2,5,AddressingMode::Indirect_Y),SBC::sbc),
    ];
        pub static ref CPU_INSTRUCTION_BUILTIN_MAP:HashMap<u8,&'static InstructionBuiltin>={
      let mut map:HashMap<u8,&'static InstructionBuiltin>=HashMap::new();
        for builtin in &*CPU_INSTRUCTION_BUILTIN{
            map.insert(builtin.op.code,builtin);
        }
        return map;
    };
    }

pub mod addressing;

pub mod TAX;
pub mod INX;
pub mod INY;
pub mod STA;
pub mod ADC;
pub mod EOR;
pub mod ORA;
pub mod AND;
pub mod LDX;
pub mod LDY;
pub mod SBC;
pub mod LDA;
pub mod ASL;
mod LSR;
mod ROR;
mod ROL;
mod RTI;
mod RIS;
mod JSR;
mod JMP;
mod CPX;
mod CPY;
mod INC;
mod DEC;
mod DEX;
mod DEY;
mod CMP;
