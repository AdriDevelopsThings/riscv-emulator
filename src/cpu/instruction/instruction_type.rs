use super::Instruction;

/// there are some fixed instruction types
pub trait TypeInstruction {
    fn parse_instruction(instruction: Instruction) -> Self;
}

/// get a value of type $type with the size of $size at position $shift (counted from the right) of $instruction
macro_rules! get_from_instruction {
    ($instruction:expr, $size:tt, $shift:tt, $type:ty) => {
        ($instruction >> $shift & (u32::pow(2, $size) - 1)) as $type
    };
}

pub struct RTypeInstruction {
    pub rd: usize,
    pub funct3: u8,
    pub rs1: usize,
    pub rs2: usize,
    pub funct7: u8,
}

pub struct ITypeInstruction {
    pub rd: usize,
    pub funct3: u8,
    pub rs1: usize,
    pub imm: u16,
}

pub struct SpecialITypeInstruction {
    pub rd: usize,
    pub funct3: u8,
    pub rs1: usize,
    pub imm_1: u8,
    pub imm_2: u8,
}

pub struct STypeInstruction {
    pub imm_1: u8,
    pub funct3: u8,
    pub rs1: usize,
    pub rs2: usize,
    pub imm_2: u8,
}

pub struct BTypeInstruction {
    pub imm_1: u8,
    pub imm_2: u8,
    pub funct3: u8,
    pub rs1: usize,
    pub rs2: usize,
    pub imm_3: u8,
    pub imm_4: u8,
}

pub struct UTypeInstruction {
    pub rd: usize,
    pub imm: u32,
}

pub struct JTypeInstruction {
    pub rd: usize,
    pub imm_1: u8,
    pub imm_2: u8,
    pub imm_3: u16,
    pub imm_4: u8,
}

pub fn parse_opcode(instruction: Instruction) -> u8 {
    // the opcode is the most right value of an instruction
    get_from_instruction!(instruction, 7, 0, u8)
}

pub fn parse_funct3(instruction: Instruction) -> u8 {
    get_from_instruction!(instruction, 3, 12, u8)
}

pub fn parse_rd(instruction: Instruction) -> usize {
    get_from_instruction!(instruction, 5, 7, usize)
}

impl TypeInstruction for RTypeInstruction {
    fn parse_instruction(instruction: Instruction) -> Self {
        Self {
            rd: parse_rd(instruction),
            funct3: parse_funct3(instruction),
            rs1: (instruction >> 15 & 0x1F) as usize,
            rs2: (instruction >> 20 & 0x1F) as usize,
            funct7: (instruction >> 25 & 0x7F) as u8,
        }
    }
}

impl TypeInstruction for ITypeInstruction {
    fn parse_instruction(instruction: Instruction) -> Self {
        Self {
            rd: parse_rd(instruction),
            funct3: parse_funct3(instruction),
            rs1: get_from_instruction!(instruction, 5, 15, usize),
            imm: get_from_instruction!(instruction, 12, 20, u16),
        }
    }
}

impl TypeInstruction for SpecialITypeInstruction {
    fn parse_instruction(instruction: Instruction) -> Self {
        Self {
            rd: parse_rd(instruction),
            funct3: parse_funct3(instruction),
            rs1: get_from_instruction!(instruction, 5, 15, usize),
            imm_1: get_from_instruction!(instruction, 5, 20, u8),
            imm_2: get_from_instruction!(instruction, 7, 25, u8),
        }
    }
}

impl TypeInstruction for STypeInstruction {
    fn parse_instruction(instruction: Instruction) -> Self {
        Self {
            imm_1: get_from_instruction!(instruction, 5, 7, u8),
            funct3: parse_funct3(instruction),
            rs1: get_from_instruction!(instruction, 5, 15, usize),
            rs2: get_from_instruction!(instruction, 5, 20, usize),
            imm_2: get_from_instruction!(instruction, 7, 25, u8),
        }
    }
}

impl TypeInstruction for BTypeInstruction {
    fn parse_instruction(instruction: Instruction) -> Self {
        Self {
            imm_1: get_from_instruction!(instruction, 1, 7, u8),
            imm_2: get_from_instruction!(instruction, 4, 8, u8),
            funct3: parse_funct3(instruction),
            rs1: get_from_instruction!(instruction, 5, 15, usize),
            rs2: get_from_instruction!(instruction, 5, 20, usize),
            imm_3: get_from_instruction!(instruction, 6, 25, u8),
            imm_4: get_from_instruction!(instruction, 1, 31, u8),
        }
    }
}

impl TypeInstruction for UTypeInstruction {
    fn parse_instruction(instruction: Instruction) -> Self {
        Self {
            rd: parse_rd(instruction),
            imm: get_from_instruction!(instruction, 20, 12, u32),
        }
    }
}

impl TypeInstruction for JTypeInstruction {
    fn parse_instruction(instruction: Instruction) -> Self {
        Self {
            rd: parse_rd(instruction),
            imm_1: get_from_instruction!(instruction, 8, 12, u8),
            imm_2: get_from_instruction!(instruction, 1, 20, u8),
            imm_3: get_from_instruction!(instruction, 10, 21, u16),
            imm_4: get_from_instruction!(instruction, 1, 31, u8),
        }
    }
}

impl STypeInstruction {
    pub fn get_full_immediate(&self) -> u16 {
        (self.imm_1 as u16) | ((self.imm_2 as u16) << 5)
    }
}

impl BTypeInstruction {
    pub fn get_full_immediate(&self) -> u16 {
        (self.imm_2 as u16)
            | ((self.imm_3 as u16) << 4)
            | ((self.imm_1 as u16) << 10)
            | ((self.imm_4 as u16) << 11)
    }
}

impl JTypeInstruction {
    pub fn get_full_immediate(&self) -> u32 {
        (self.imm_3 as u32)
            | ((self.imm_2 as u32) << 10)
            | ((self.imm_1 as u32) << 11)
            | ((self.imm_4 as u32) << 19)
    }
}
