use std::collections::HashMap;
use lazy_static::lazy_static;

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
    pub operand_len:u8,
    /**
    "周期"（Cycles）是指完成一个特定指令或操作所需的时间单位。它通常用于描述处理器的时钟周期数或指令执行的时间。
    每个处理器的时钟周期长度是固定的，它定义了处理器的基本时钟速度。指令的执行时间可以通过时钟周期数来度量。例如，如果一个指令需要 4 个时钟周期才能完成执行，那么它的周期数就是 4。
     */
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OpCode {
    fn new(code: u8, mnemonic: &'static str, len: u8, cycles: u8, mode: AddressingMode) -> Self {
        return OpCode { code, mnemonic, len, operand_len: len-1, cycles, mode };
    }
}

lazy_static! {
    pub static ref CPU_OPS_CODES:Vec<OpCode>=vec![
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),
        OpCode::new(0xAA, "TAX", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xE8, "INX", 1, 2, AddressingMode::NoneAddressing),

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

        OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x8D, "STA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x9D, "STA", 3, 5, AddressingMode::Absolute_X),
        OpCode::new(0x99, "STA", 3, 5, AddressingMode::Absolute_Y),
        OpCode::new(0x81, "STA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x91, "STA", 2, 6, AddressingMode::Indirect_Y),

        OpCode::new(0xB6, "LDX", 2, 4, AddressingMode::ZeroPage_Y),
    ];

    pub static ref OPCODE_MAP:HashMap<u8,&'static OpCode>={
      let mut map:HashMap<u8,&'static OpCode>=HashMap::new();
        for opcode in &*CPU_OPS_CODES{
            map.insert(opcode.code,opcode);
        }
        return map;
    };
}