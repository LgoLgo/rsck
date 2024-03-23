use super::opcode;

#[derive(Debug, PartialEq)]
pub enum IR {
    SHR(u32),
    SHL(u32),
    ADD(u8),
    SUB(u8),
    PUTCHAR,
    GETCHAR,
    JIZ(u32),
    JNZ(u32),
}

#[derive(Debug)]
pub struct Code {
    pub instruct: Vec<IR>,
}

impl Code {
    pub fn from(data: Vec<opcode::Opcode>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut instruct: Vec<IR> = Vec::new();
        let mut j_stack: Vec<u32> = Vec::new();
        for e in data {
            match e {
                opcode::Opcode::SHR => match instruct.last_mut() {
                    Some(IR::SHR(x)) => {
                        *x += 1;
                    }
                    _ => {
                        instruct.push(IR::SHR(1));
                    }
                },
                opcode::Opcode::SHL => match instruct.last_mut() {
                    Some(IR::SHL(x)) => {
                        *x += 1;
                    }
                    _ => {
                        instruct.push(IR::SHL(1));
                    }
                },
                opcode::Opcode::ADD => match instruct.last_mut() {
                    Some(IR::ADD(x)) => {
                        let (b, _) = x.overflowing_add(1);
                        *x = b;
                    }
                    _ => {
                        instruct.push(IR::ADD(1));
                    }
                },
                opcode::Opcode::SUB => match instruct.last_mut() {
                    Some(IR::SUB(x)) => {
                        let (b, _) = x.overflowing_add(1);
                        *x = b;
                    }
                    _ => {
                        instruct.push(IR::SUB(1));
                    }
                },
                opcode::Opcode::GETCHAR => {
                    instruct.push(IR::GETCHAR);
                }
                opcode::Opcode::PUTCHAR => {
                    instruct.push(IR::PUTCHAR);
                }
                opcode::Opcode::LB => {
                    instruct.push(IR::JIZ(0));
                    j_stack.push((instruct.len() - 1) as u32);
                }
                opcode::Opcode::RB => {
                    let j = j_stack.pop().ok_or("pop from empty list")?;
                    instruct.push(IR::JNZ(j));
                    let instrs_len = instruct.len();
                    match &mut instruct[j as usize] {
                        IR::JIZ(x) => {
                            *x = (instrs_len - 1) as u32;
                        }
                        _ => {
                            unimplemented!();
                        }
                    }
                }
            }
        }
        Ok(Code { instruct })
    }
}