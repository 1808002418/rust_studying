use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::cpu::CPU;
use crate::instruction::TAX;

/**
这是一个针对CPU指令集中的寻址模式（Addressing Mode）的枚举定义。在计算机体系结构中，寻址模式用于指定如何计算或获取操作数（数据）的地址。下面是对每种寻址模式的解释：

Immediate: 立即寻址模式。操作数直接包含在指令中，例如：LDA #10，表示将值10加载到累加器（Accumulator）寄存器中。

ZeroPage: 零页寻址模式。操作数的地址位于零页（地址范围为0x0000-0x00FF）内，只需一个字节来表示地址。例如：LDA $45，表示将地址为0x45的内存单元的值加载到累加器寄存器中。

ZeroPage_X: 零页X变址寻址模式。操作数的地址为零页内的一个字节，而X寄存器的值会被加到这个地址上。例如：LDX $25,X，表示将地址为0x25+X的内存单元的值加载到X寄存器中。

ZeroPage_Y: 零页Y变址寻址模式。操作数的地址为零页内的一个字节，而Y寄存器的值会被加到这个地址上。例如：LDY $30,Y，表示将地址为0x30+Y的内存单元的值加载到Y寄存器中。

Absolute: 绝对寻址模式。操作数的地址通过一个完整的地址表示。例如：LDA $2000，表示将地址为0x2000的内存单元的值加载到累加器寄存器中。

Absolute_X: 绝对X变址寻址模式。操作数的地址为一个完整的地址，而X寄存器的值会被加到这个地址上。例如：STA $3000,X，表示将累加器寄存器的值存储到地址为0x3000+X的内存单元中。

Absolute_Y: 绝对Y变址寻址模式。操作数的地址为一个完整的地址，而Y寄存器的值会被加到这个地址上。例如：STA $4000,Y，表示将累加器寄存器的值存储到地址为0x4000+Y的内存单元中。

Indirect_X: 间接X变址寻址模式。操作数的地址通过一个间接寻址的方式计算得到。首先，将一个字节与X寄存器相加得到一个地址，然后使用这个地址作为间接寻址的目标地址。例如：JMP ($20,X)，表示通过将0x20+X得到的地址所指向的内存单元中的值作为新的指令地址，实现间接跳转。

Indirect_Y: 间接Y变址寻址模式。操作数的地址通过一个间接寻址的方式计算得到。首先，使用一个地址作为间接寻址的目标地址，然后将这个地址与Y寄存器相加得到最终的地址。例如：STA ($30),Y，表示将累加器寄存器的值存储到以地址0x30为间接寻址目标，再加上Y寄存器的值得到的最终地址所指向的内存单元中。

NoneAddressing: 无寻址模式。表示该指令没有操作数，或者操作数不需要通过寻址方式获取。
 */
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

pub struct OpCode {
    pub code: u8,
    pub mnemonic: &'static str,
    /**
    这条指令的长度
    TAX 长度为1
    LDA 0x11 长度就是2
     */
    pub len: u8,
    /**
    操作数的长度,不包含指令本身
    TAX 长度为0
    LDA 0x11 长度就是1
     */
    pub operand_len: u8,
    /**
    "周期"（Cycles）是指完成一个特定指令或操作所需的时间单位。它通常用于描述处理器的时钟周期数或指令执行的时间。
    每个处理器的时钟周期长度是固定的，它定义了处理器的基本时钟速度。指令的执行时间可以通过时钟周期数来度量。例如，如果一个指令需要 4 个时钟周期才能完成执行，那么它的周期数就是 4。
     */
    pub cycles: u8,
    pub mode: AddressingMode,
    pub exec: Box<dyn Fn(&mut CPU, &AddressingMode)>,
}

impl OpCode {
    fn new(code: u8,
           mnemonic: &'static str,
           len: u8,
           cycles: u8,
           mode: AddressingMode,
           exec: Box<dyn Fn(&mut CPU, &AddressingMode)>) -> Self {
        return OpCode { code, mnemonic, len, operand_len: len - 1, cycles, mode, exec };
    }
}

lazy_static! {
    pub static ref CPU_OPS_CODES:Vec<OpCode>=vec![
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),
        OpCode::new(0xAA, "TAX", 1, 2, AddressingMode::NoneAddressing,Box::new(TAX::tax)),
        OpCode::new(0xE8, "INX", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xC8, "INY", 1, 2, AddressingMode::NoneAddressing),

        OpCode::new(0xe6, "INC", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0xf6, "INC", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new(0xee, "INC", 3, 6, AddressingMode::Absolute),
        OpCode::new(0xfe, "INC", 3, 7, AddressingMode::Absolute_X),

        OpCode::new(0xc6, "DEC", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0xd6, "DEC", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new(0xce, "DEC", 3, 6, AddressingMode::Absolute),
        OpCode::new(0xde, "DEC", 3, 7, AddressingMode::Absolute_X),

        OpCode::new(0xca, "DEX", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x88, "DEY", 1, 2, AddressingMode::NoneAddressing),

        OpCode::new(0xc9, "CMP", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xc5, "CMP", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xd5, "CMP", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xcd, "CMP", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xdd, "CMP", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_X),
        OpCode::new(0xd9, "CMP", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_Y),
        OpCode::new(0xc1, "CMP", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0xd1, "CMP", 2, 5/*+1 if page crossed*/, AddressingMode::Indirect_Y),

        OpCode::new(0xc0, "CPY", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xc4, "CPY", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xcc, "CPY", 3, 4, AddressingMode::Absolute),

        OpCode::new(0xe0, "CPX", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xe4, "CPX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xec, "CPX", 3, 4, AddressingMode::Absolute),


        /* Branching */

        OpCode::new(0x4c, "JMP", 3, 3, AddressingMode::NoneAddressing), //AddressingMode that acts as Immidiate
        OpCode::new(0x6c, "JMP", 3, 5, AddressingMode::NoneAddressing), //AddressingMode:Indirect with 6502 bug

        OpCode::new(0x20, "JSR", 3, 6, AddressingMode::NoneAddressing),
        OpCode::new(0x60, "RTS", 1, 6, AddressingMode::NoneAddressing),

        OpCode::new(0x40, "RTI", 1, 6, AddressingMode::NoneAddressing),



        OpCode::new(0xA9, "LDA", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xA5, "LDA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xB5, "LDA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xAD, "LDA", 3, 4, AddressingMode::Absolute),
        /*
        "Absolute_X"：使用绝对地址（16 位）和 X 寄存器的值来计算内存地址。X 寄存器的值会与绝对地址相加，得到最终的内存地址。

        "+1 if page crossed"：如果在计算得到的内存地址跨越了页（page），则将最终的内存地址再加上 1。

        在 6502 微处理器中，内存被划分为多个页，每个页的大小为 256 字节。当使用 "Absolute_X" 寻址模式时，如果计算得到的内存地址跨越了当前页的边界，就会发生页跨越（page crossing）。
        为了处理页跨越情况，需要将最终的内存地址再加上 1。
        */
        OpCode::new(0xBD, "LDA", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_X),
        OpCode::new(0xB9, "LDA", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_Y),
        OpCode::new(0xA1, "LDA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0xB1, "LDA", 2, 5/*+1 if page crossed*/, AddressingMode::Indirect_Y),

        OpCode::new(0xA0,"LDY",2,2,AddressingMode::Immediate),
        OpCode::new(0xA4,"LDY",2,3,AddressingMode::ZeroPage),
        OpCode::new(0xB4,"LDY",2,4,AddressingMode::ZeroPage_X),
        OpCode::new(0xAC,"LDY",3,4,AddressingMode::Absolute),
        OpCode::new(0xBC,"LDY",3,4,AddressingMode::Absolute_X),

        OpCode::new(0xA2,"LDX",2,2,AddressingMode::Immediate),
        OpCode::new(0xA6,"LDX",2,3,AddressingMode::ZeroPage),
        OpCode::new(0xB6,"LDX",2,4,AddressingMode::ZeroPage_Y),
        OpCode::new(0xAE,"LDX",3,4,AddressingMode::Absolute),
        OpCode::new(0xBE,"LDX",3,4,AddressingMode::Absolute_Y),

        OpCode::new(0x29,"AND",2,2,AddressingMode::Immediate),
        OpCode::new(0x25,"AND",2,3,AddressingMode::ZeroPage),
        OpCode::new(0x35,"AND",2,4,AddressingMode::ZeroPage_X),
        OpCode::new(0x2D,"AND",3,4,AddressingMode::Absolute),
        OpCode::new(0x3D,"AND",3,4,AddressingMode::Absolute_X),
        OpCode::new(0x39,"AND",3,4,AddressingMode::Absolute_Y),
        OpCode::new(0x21,"AND",2,6,AddressingMode::Indirect_X),
        OpCode::new(0x31,"AND",2,5,AddressingMode::Indirect_Y),

        OpCode::new(0x09,"ORA",2,2,AddressingMode::Immediate),
        OpCode::new(0x05,"ORA",2,3,AddressingMode::ZeroPage),
        OpCode::new(0x15,"ORA",2,4,AddressingMode::ZeroPage_X),
        OpCode::new(0x0D,"ORA",3,4,AddressingMode::Absolute),
        OpCode::new(0x1D,"ORA",3,4,AddressingMode::Absolute_X),
        OpCode::new(0x19,"ORA",3,4,AddressingMode::Absolute_Y),
        OpCode::new(0x01,"ORA",2,6,AddressingMode::Indirect_X),
        OpCode::new(0x11,"ORA",2,5,AddressingMode::Indirect_Y),

        OpCode::new(0x49,"EOR",2,2,AddressingMode::Immediate),
        OpCode::new(0x45,"EOR",2,3,AddressingMode::ZeroPage),
        OpCode::new(0x55,"EOR",2,4,AddressingMode::ZeroPage_Y),
        OpCode::new(0x4D,"EOR",3,4,AddressingMode::Absolute),
        OpCode::new(0x5D,"EOR",3,4,AddressingMode::Absolute_X),
        OpCode::new(0x59,"EOR",3,4,AddressingMode::Absolute_Y),
        OpCode::new(0x41,"EOR",2,6,AddressingMode::Indirect_X),
        OpCode::new(0x51,"EOR",2,5,AddressingMode::Indirect_Y),

        OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x8D, "STA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x9D, "STA", 3, 5, AddressingMode::Absolute_X),
        OpCode::new(0x99, "STA", 3, 5, AddressingMode::Absolute_Y),
        OpCode::new(0x81, "STA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x91, "STA", 2, 6, AddressingMode::Indirect_Y),

        OpCode::new(0xB6, "LDX", 2, 4, AddressingMode::ZeroPage_Y),

        OpCode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x75, "ADC", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x6D, "ADC", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x7D, "ADC", 3, 4, AddressingMode::Absolute_X),
        OpCode::new(0x79, "ADC", 3, 4, AddressingMode::Absolute_Y),
        OpCode::new(0x61, "ADC", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x71, "ADC", 2, 5, AddressingMode::Indirect_Y),

        OpCode::new(0x2a, "ROL", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x26, "ROL", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x36, "ROL", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new(0x2e, "ROL", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x3e, "ROL", 3, 7, AddressingMode::Absolute_X),

        OpCode::new(0x6a, "ROR", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x66, "ROR", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x76, "ROR", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new(0x6e, "ROR", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x7e, "ROR", 3, 7, AddressingMode::Absolute_X),

        OpCode::new(0x0A,"ASL",1,2,AddressingMode::NoneAddressing),
        OpCode::new(0x06,"ASL",2,5,AddressingMode::ZeroPage),
        OpCode::new(0x16,"ASL",2,6,AddressingMode::ZeroPage_X),
        OpCode::new(0x0E,"ASL",3,6,AddressingMode::Absolute),
        OpCode::new(0x1E,"ASL",3,7,AddressingMode::Absolute_X),

        OpCode::new(0x4A,"LSR",1,2,AddressingMode::NoneAddressing),
        OpCode::new(0x46,"LSR",2,5,AddressingMode::ZeroPage),
        OpCode::new(0x56,"LSR",2,6,AddressingMode::ZeroPage_X),
        OpCode::new(0x4E,"LSR",3,6,AddressingMode::Absolute),
        OpCode::new(0x5E,"LSR",3,7,AddressingMode::Absolute_X),

        OpCode::new(0xE9,"SBC",2,2,AddressingMode::Immediate),
        OpCode::new(0xE5,"SBC",2,3,AddressingMode::ZeroPage),
        OpCode::new(0xF5,"SBC",2,4,AddressingMode::ZeroPage_X),
        OpCode::new(0xED,"SBC",3,4,AddressingMode::Absolute),
        OpCode::new(0xFD,"SBC",3,4,AddressingMode::Absolute_X),
        OpCode::new(0xF9,"SBC",3,4,AddressingMode::Absolute_Y),
        OpCode::new(0xE1,"SBC",2,6,AddressingMode::Indirect_X),
        OpCode::new(0xF1,"SBC",2,5,AddressingMode::Indirect_Y),
    ];

    pub static ref OPCODE_MAP:HashMap<u8,&'static OpCode>={
      let mut map:HashMap<u8,&'static OpCode>=HashMap::new();
        for opcode in &*CPU_OPS_CODES{
            map.insert(opcode.code,opcode);
        }
        return map;
    };
}