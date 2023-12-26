pub struct Memory {
    // 65535个u8类型的元素, 相当于64KB的内存
    pub bytes: [u8; 0xffff],
}

impl Default for Memory {
    fn default() -> Self {
        return Memory { bytes: [0; 0xffff] };
    }
}

impl Memory {
    pub fn read(&self, addr: u16) -> u8 {
        return self.bytes[addr as usize];
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.bytes[addr as usize] = data;
    }

    // 加载程序到内存位置
    pub fn load_program(&mut self, addr: u16, program: Vec<u8>) {
        let offset = (addr + program.len() as u16) as usize;
        self.bytes[addr as usize..offset].copy_from_slice(&program[..]);
    }
}