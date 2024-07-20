#[derive(Debug)]
pub enum Instruction {
    ClearScreen,
    Jump(u16), // addr
    SetRegister(u8, u8),
    AddRegister(u8, u8),
    SetI(u16),
    Draw(u16, u16, u16),
    Call(u16), // addr
    Ret,

    ERROR(u16), // unknown opcode
}

impl Instruction {
    pub fn new(value: u16) -> Instruction {
        let lower1 = (value & 0x000F) >> 0;
        let upper1 = (value & 0x00F0) >> 4;
        let lower2 = (value & 0x0F00) >> 8;
        let upper2 = (value & 0xF000) >> 12;

        println!("{:#01x} {:#01x} {:#01x} {:#01x}", upper2, lower2, upper1, lower1);
        match (upper2, lower2, upper1, lower1) {
            (0x0, 0x0, 0xe, 0x0) => {
                return Instruction::ClearScreen;
            }

            (0x0, 0x0, 0xe, 0xe) => {
                return Instruction::Ret;
            }

            (0x1, n1, n2, n3) => {
                let val: u16 = n1 << 8 | n2 << 4 | n3;
                return Instruction::Jump(val);
            }

            (0x1, n1, n2, n3) => {
                let val: u16 = n1 << 8 | n2 << 4 | n3;
                return Instruction::Call(val);
            }

            (0x6, x, n1, n2) => {
                let val: u8 = (n1 << 4 | n2).try_into().unwrap();
                return Instruction::SetRegister(x.try_into().unwrap(), val);
            }

            (0x7, x, n1, n2) => {
                let val: u8 = (n1 << 4 | n2).try_into().unwrap();
                return Instruction::AddRegister(x.try_into().unwrap(), val);
            }

            (0xa, n1, n2, n3) => {
                let val: u16 = n1 << 8 | n2 << 4 | n3;
                return Instruction::SetI(val);
            }

            (0xd, x, y, n) => {
                return Instruction::Draw(x, y, n);
            }

            _ => {
                return Instruction::ERROR(value);
            }
        }
    }
}