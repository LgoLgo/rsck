#[derive(Debug, PartialEq)]
pub enum Opcode {
    SHR = 0x3E,
    SHL = 0x3C,
    ADD = 0x2B,
    SUB = 0x2D,
    PUTCHAR = 0x2E,
    GETCHAR = 0x2C,
    LB = 0x5B,
    RB = 0x5D,
}

impl From<u8> for Opcode {
    fn from(u: u8) -> Self {
        match u {
            0x3E => Opcode::SHR,
            0x3C => Opcode::SHL,
            0x2B => Opcode::ADD,
            0x2D => Opcode::SUB,
            0x2E => Opcode::PUTCHAR,
            0x2C => Opcode::GETCHAR,
            0x5B => Opcode::LB,
            0x5D => Opcode::RB,
            _ => panic!(),
        }
    }
}

impl Into<u8> for Opcode {
    fn into(self) -> u8 {
        match self {
            Opcode::SHR => 0x3E,
            Opcode::SHL => 0x3C,
            Opcode::ADD => 0x2B,
            Opcode::SUB => 0x2D,
            Opcode::PUTCHAR => 0x2E,
            Opcode::GETCHAR => 0x2C,
            Opcode::LB => 0x5B,
            Opcode::RB => 0x5D,
        }
    }
}

pub struct Code {
    pub instruct: Vec<Opcode>,
    pub j_table: std::collections::HashMap<usize, usize>,
}

impl Code {
    pub fn from(data: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        let dict: Vec<u8> = vec![
            Opcode::SHL.into(),
            Opcode::SHR.into(),
            Opcode::ADD.into(),
            Opcode::SUB.into(),
            Opcode::GETCHAR.into(),
            Opcode::PUTCHAR.into(),
            Opcode::LB.into(),
            Opcode::RB.into(),
        ];
        let instruct: Vec<Opcode> = data
            .iter()
            .filter(|x| dict.contains(x))
            .map(|x| Opcode::from(*x))
            .collect();
        let mut jstack: Vec<usize> = Vec::new();
        let mut j_table: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for (i, e) in instruct.iter().enumerate() {
            if Opcode::LB == *e {
                jstack.push(i);
            }
            if Opcode::RB == *e {
                let j = jstack.pop().ok_or("pop from empty list")?;
                j_table.insert(j, i);
                j_table.insert(i, j);
            }
        }
        Ok(Code { instruct: instruct, j_table: j_table })
    }
}