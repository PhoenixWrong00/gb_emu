use crate::inst::*;
use crate::registers::{Registers};
use crate::membus::{MemoryBus};

pub struct CPU {
    registers: Registers,
    pc: u16,
    sp: u16,
    bus: MemoryBus,
}

impl CPU {
    fn read_next_byte(&mut self) -> u8 {
        let byte = self.bus.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    pub fn step(&mut self) {
        let instruction_byte = self.bus.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let prefixed = instruction_byte == 0xCB;
        let real_instruction = if prefixed {
            let next = self.bus.read_byte(self.pc);
            self.pc = self.pc.wrapping_add(1);
            next
        } else {
            instruction_byte
        };

        if let Some(instruction) = Instruction::from_byte(real_instruction, prefixed) {
            self.execute(instruction);
        } else {
            panic!("Wgat...");
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::LD(load_type) => {
                match load_type {
                    LoadType::Byte(target, source) => {
                        let source_value = match source {
                            LoadByteSource::A => self.registers.a,
                            LoadByteSource::B => self.registers.b,
                            LoadByteSource::C => self.registers.c,
                            LoadByteSource::D => self.registers.d,
                            LoadByteSource::E => self.registers.e,
                            LoadByteSource::H => self.registers.h,
                            LoadByteSource::L => self.registers.l,
                            LoadByteSource::D8 => self.read_next_byte(),
                            LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
                        };
                        match target {
                            LoadByteTarget::A => self.registers.a = source_value,
                            LoadByteTarget::B => self.registers.b = source_value,
                            LoadByteTarget::C => self.registers.c = source_value,
                            LoadByteTarget::D => self.registers.d = source_value,
                            LoadByteTarget::E => self.registers.e = source_value,
                            LoadByteTarget::H => self.registers.h = source_value,
                            LoadByteTarget::L => self.registers.l = source_value,
                            LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(), source_value),
                        };
                    }
                    LoadType::Word(target) => {
                        let low = self.read_next_byte() as u16;
                        let high = self.read_next_byte() as u16;
                        let value = (high << 8) | low;
                        match target {
                            LoadWordTarget::BC => self.registers.set_bc(value),
                            LoadWordTarget::DE => self.registers.set_de(value),
                            LoadWordTarget::HL => self.registers.set_hl(value),
                            LoadWordTarget::SP => self.sp = value
                        }
                    }
                    _ => panic!("TODO: Implement other load types")
                }
            }

            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true
                };
                self.jump(jump_condition)
            }

            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value: u8 = self.registers.a;
                        let new_value: u8 = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::B => {
                        let value: u8 = self.registers.b;
                        let new_value: u8 = self.add(value);
                        self.registers.a = new_value;
                    }

                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::D => {
                        let value: u8 = self.registers.d;
                        let new_value: u8 = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::E => {
                        let value: u8 = self.registers.e;
                        let new_value: u8 = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::H => {
                        let value: u8 = self.registers.h;
                        let new_value: u8 = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::L => {
                        let value: u8 = self.registers.l;
                        let new_value: u8 = self.add(value);
                        self.registers.a = new_value;
                    }
                    _ => panic!("Invalid target"),

                }
            }
            Instruction::ADC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value: u8 = self.registers.a;
                        let new_value: u8 = self.adc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::B => {
                        let value: u8 = self.registers.b;
                        let new_value: u8 = self.adc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::C => {
                        let value: u8 = self.registers.c;
                        let new_value: u8 = self.adc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::D => {
                        let value: u8 = self.registers.d;
                        let new_value: u8 = self.adc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::E => {
                        let value: u8 = self.registers.e;
                        let new_value: u8 = self.adc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::H => {
                        let value: u8 = self.registers.h;
                        let new_value: u8 = self.adc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::L => {
                        let value: u8 = self.registers.l;
                        let new_value: u8 = self.adc(value);
                        self.registers.a = new_value;
                    }
                    _ => panic!("Invalid target"),
                }
            }
            Instruction::ADDHL(target) => {
                match target {
                    ArithmeticTarget::HL => {
                        let value: u16 = self.registers.get_hl();
                        let new_value: u16 = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                    ArithmeticTarget::BC => {
                        let value: u16 = self.registers.get_bc();
                        let new_value: u16 = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                    ArithmeticTarget::DE => {
                        let value: u16 = self.registers.get_de();
                        let new_value: u16 = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                    ArithmeticTarget::SP => {
                        let value: u16 = self.sp;
                        let new_value: u16 = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                    _ => panic!("Invalid target"),
                }
            }
            Instruction::SUB(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value: u8 = self.registers.a;
                        let new_value: u8 = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::B => {
                        let value: u8 = self.registers.b;
                        let new_value: u8 = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::C => {
                        let value: u8 = self.registers.c;
                        let new_value: u8 = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::D => {
                        let value: u8 = self.registers.d;
                        let new_value: u8 = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::E => {
                        let value: u8 = self.registers.e;
                        let new_value: u8 = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::H => {
                        let value: u8 = self.registers.h;
                        let new_value: u8 = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::L => {
                        let value: u8 = self.registers.l;
                        let new_value: u8 = self.sub(value);
                        self.registers.a = new_value;
                    }
                    _ => panic!("Invalid target"),
                }
            }
            Instruction::SBC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value: u8 = self.registers.a;
                        let new_value: u8 = self.sbc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::B => {
                        let value: u8 = self.registers.b;
                        let new_value: u8 = self.sbc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::C => {
                        let value: u8 = self.registers.c;
                        let new_value: u8 = self.sbc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::D => {
                        let value: u8 = self.registers.d;
                        let new_value: u8 = self.sbc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::E => {
                        let value: u8 = self.registers.e;
                        let new_value: u8 = self.sbc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::H => {
                        let value: u8 = self.registers.h;
                        let new_value: u8 = self.sbc(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::L => {
                        let value: u8 = self.registers.l;
                        let new_value: u8 = self.sbc(value);
                        self.registers.a = new_value;
                    }
                    _ => panic!("Invalid target"),

                }
            }
            Instruction::AND(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value: u8 = self.registers.a;
                        let new_value: u8 = self.and(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::B => {
                        let value: u8 = self.registers.b;
                        let new_value: u8 = self.and(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::C => {
                        let value: u8 = self.registers.c;
                        let new_value: u8 = self.and(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::D => {
                        let value: u8 = self.registers.d;
                        let new_value: u8 = self.and(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::E => {
                        let value: u8 = self.registers.e;
                        let new_value: u8 = self.and(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::H => {
                        let value: u8 = self.registers.h;
                        let new_value: u8 = self.and(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::L => {
                        let value: u8 = self.registers.l;
                        let new_value: u8 = self.and(value);
                        self.registers.a = new_value;
                    }
                    _ => panic!("Invalid target"),
                }
            }
            Instruction::OR(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = self.or(self.registers.a);
                    }
                    ArithmeticTarget::B => {
                        self.registers.a = self.or(self.registers.b);
                    }
                    ArithmeticTarget::C => {
                        self.registers.a = self.or(self.registers.c);
                    }
                    ArithmeticTarget::D => {
                        self.registers.a = self.or(self.registers.d);
                    }
                    ArithmeticTarget::E => {
                        self.registers.a = self.or(self.registers.e);
                    }
                    ArithmeticTarget::H => {
                        self.registers.a = self.or(self.registers.h);
                    }
                    ArithmeticTarget::L => {
                        self.registers.a = self.or(self.registers.l);
                    }
                    _ => panic!("Invalid target"),
                }
            }
            Instruction::XOR(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = self.xor(self.registers.a);
                    }
                    ArithmeticTarget::B => {
                        self.registers.a = self.xor(self.registers.b);
                    }
                    ArithmeticTarget::C => {
                        self.registers.a = self.xor(self.registers.c);
                    }
                    ArithmeticTarget::D => {
                        self.registers.a = self.xor(self.registers.d);
                    }
                    ArithmeticTarget::E => {
                        self.registers.a = self.xor(self.registers.e);
                    }
                    ArithmeticTarget::H => {
                        self.registers.a = self.xor(self.registers.h);
                    }
                    ArithmeticTarget::L => {
                        self.registers.a = self.xor(self.registers.l);
                    }
                    _ => panic!("Invalid target"),
                }
            }
            Instruction::CP(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.cp(self.registers.a);
                    }
                    ArithmeticTarget::B => {
                        self.cp(self.registers.b);
                    }
                    ArithmeticTarget::C => {
                        self.cp(self.registers.c);
                    }
                    ArithmeticTarget::D => {
                        self.cp(self.registers.d);
                    }
                    ArithmeticTarget::E => {
                        self.cp(self.registers.e);
                    }
                    ArithmeticTarget::H => {
                        self.cp(self.registers.h);
                    }
                    ArithmeticTarget::L => {
                        self.cp(self.registers.l);
                    }
                    _ => panic!("Invalid target"),
                }
            }
            Instruction::INC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = self.inc(self.registers.a);
                    }
                    ArithmeticTarget::B => {
                        self.registers.b = self.inc(self.registers.b);
                    }
                    ArithmeticTarget::C => {
                        self.registers.c = self.inc(self.registers.c);
                    }
                    ArithmeticTarget::D => {
                        self.registers.d = self.inc(self.registers.d);
                    }
                    ArithmeticTarget::E => {
                        self.registers.e = self.inc(self.registers.e);
                    }
                    ArithmeticTarget::H => {
                        self.registers.h = self.inc(self.registers.h);
                    }
                    ArithmeticTarget::L => {
                        self.registers.l = self.inc(self.registers.l);
                    }
                    _ => panic!("Invalid target"),
                }
            }
            Instruction::DEC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = self.dec(self.registers.a);
                    }
                    ArithmeticTarget::B => {
                        self.registers.b = self.dec(self.registers.b);
                    }
                    ArithmeticTarget::C => {
                        self.registers.c = self.dec(self.registers.c);
                    }
                    ArithmeticTarget::D => {
                        self.registers.d = self.dec(self.registers.d);
                    }
                    ArithmeticTarget::E => {
                        self.registers.e = self.dec(self.registers.e);
                    }
                    ArithmeticTarget::H => {
                        self.registers.h = self.dec(self.registers.h);
                    }
                    ArithmeticTarget::L => {
                        self.registers.l = self.dec(self.registers.l);
                    }
                    _ => panic!("Invalid target"),
                }
            }
            Instruction::CCF => {
                self.ccf();
            }
            Instruction::SCF => {
                self.scf();
            }
            Instruction::RRA => {
                self.rra();
            }
            Instruction::RLA => {
                self.rla();
            }
            Instruction::RRCA => {
                self.rrca();
            }
            Instruction::RLCA => {
                self.rlca();
            }
            Instruction::CPL => {
                self.cpl();
            }
            Instruction::BIT(bit, target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.bit(bit, self.registers.a);
                    }
                    ArithmeticTarget::B => {
                        self.bit(bit, self.registers.b);
                    }
                    ArithmeticTarget::C => {
                        self.bit(bit, self.registers.c);
                    }
                    ArithmeticTarget::D => {
                        self.bit(bit, self.registers.d);
                    }
                    ArithmeticTarget::E => {
                        self.bit(bit, self.registers.e);
                    }
                    ArithmeticTarget::H => {
                        self.bit(bit, self.registers.h);
                    }
                    ArithmeticTarget::L => {
                        self.bit(bit, self.registers.l);
                    }
                    _ => panic!("Invalid target"),
                }
            }
            Instruction::RESET(bit, target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = self.reset(bit, self.registers.a);
                    }
                    ArithmeticTarget::B => {
                        self.registers.b = self.reset(bit, self.registers.b);
                    }
                    ArithmeticTarget::C => {
                        self.registers.c = self.reset(bit, self.registers.c);
                    }
                    ArithmeticTarget::D => {
                        self.registers.d = self.reset(bit, self.registers.d);
                    }
                    ArithmeticTarget::E => {
                        self.registers.e = self.reset(bit, self.registers.e);
                    }
                    ArithmeticTarget::H => {
                        self.registers.h = self.reset(bit, self.registers.h);
                    }
                    ArithmeticTarget::L => {
                        self.registers.l = self.reset(bit, self.registers.l);
                    }
                    _ => panic!("Invalid target"),
                }
            }
            Instruction::SET(bit, target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = self.set(bit, self.registers.a);
                    }
                    ArithmeticTarget::B => {
                        self.registers.b = self.set(bit, self.registers.b);
                    }
                    ArithmeticTarget::C => {
                        self.registers.c = self.set(bit, self.registers.c);
                    }
                    ArithmeticTarget::D => {
                        self.registers.d = self.set(bit, self.registers.d);
                    }
                    ArithmeticTarget::E => {
                        self.registers.e = self.set(bit, self.registers.e);
                    }
                    ArithmeticTarget::H => {
                        self.registers.h = self.set(bit, self.registers.h);
                    }
                    ArithmeticTarget::L => {
                        self.registers.l = self.set(bit, self.registers.l);
                    }
                    _ => panic!("Invalid target"),
                }
            }
            Instruction::SRL(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = self.srl(self.registers.a);
                    }
                    ArithmeticTarget::B => {
                        self.registers.b = self.srl(self.registers.b);
                    }
                    ArithmeticTarget::C => {
                        self.registers.c = self.srl(self.registers.c);
                    }
                    ArithmeticTarget::D => {
                        self.registers.d = self.srl(self.registers.d);
                    }
                    ArithmeticTarget::E => {
                        self.registers.e = self.srl(self.registers.e);
                    }
                    ArithmeticTarget::H => {
                        self.registers.h = self.srl(self.registers.h);
                    }
                    ArithmeticTarget::L => {
                        self.registers.l = self.srl(self.registers.l);
                    }
                    _ => panic!("Invalid target"),
                }
            }
            Instruction::RR(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = self.rr(self.registers.a);
                    }
                    ArithmeticTarget::B => {
                        self.registers.b = self.rr(self.registers.b);
                    }
                    ArithmeticTarget::C => {
                        self.registers.c = self.rr(self.registers.c);
                    }
                    ArithmeticTarget::D => {
                        self.registers.d = self.rr(self.registers.d);
                    }
                    ArithmeticTarget::E => {
                        self.registers.e = self.rr(self.registers.e);
                    }
                    ArithmeticTarget::H => {
                        self.registers.h = self.rr(self.registers.h);
                    }
                    ArithmeticTarget::L => {
                        self.registers.l = self.rr(self.registers.l);
                    }
                    _ => panic!("Invalid target"),
                }
            }
            Instruction::RL(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = self.rl(self.registers.a);
                    }
                    ArithmeticTarget::B => {
                        self.registers.b = self.rl(self.registers.b);
                    }
                    ArithmeticTarget::C => {
                        self.registers.c = self.rl(self.registers.c);
                    }
                    ArithmeticTarget::D => {
                        self.registers.d = self.rl(self.registers.d);
                    }
                    ArithmeticTarget::E => {
                        self.registers.e = self.rl(self.registers.e);
                    }
                    ArithmeticTarget::H => {
                        self.registers.h = self.rl(self.registers.h);
                    }
                    ArithmeticTarget::L => {
                        self.registers.l = self.rl(self.registers.l);
                    }
                    _ => panic!("Invalid target"),
                }
            }
            Instruction::RRC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = self.rrc(self.registers.a);
                    }
                    ArithmeticTarget::B => {
                        self.registers.b = self.rrc(self.registers.b);
                    }
                    ArithmeticTarget::C => {
                        self.registers.c = self.rrc(self.registers.c);
                    }
                    ArithmeticTarget::D => {
                        self.registers.d = self.rrc(self.registers.d);
                    }
                    ArithmeticTarget::E => {
                        self.registers.e = self.rrc(self.registers.e);
                    }
                    ArithmeticTarget::H => {
                        self.registers.h = self.rrc(self.registers.h);
                    }
                    ArithmeticTarget::L => {
                        self.registers.l = self.rrc(self.registers.l);
                    }
                    _ => panic!("Invalid target"),
                }
            }
            Instruction::RLC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = self.rlc(self.registers.a);
                    }
                    ArithmeticTarget::B => {
                        self.registers.b = self.rlc(self.registers.b);
                    }
                    ArithmeticTarget::C => {
                        self.registers.c = self.rlc(self.registers.c);
                    }
                    ArithmeticTarget::D => {
                        self.registers.d = self.rlc(self.registers.d);
                    }
                    ArithmeticTarget::E => {
                        self.registers.e = self.rlc(self.registers.e);
                    }
                    ArithmeticTarget::H => {
                        self.registers.h = self.rlc(self.registers.h);
                    }
                    ArithmeticTarget::L => {
                        self.registers.l = self.rlc(self.registers.l);
                    }
                    _ => panic!("Invalid target"),
                }
            }
            Instruction::SRA(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = self.sra(self.registers.a);
                    }
                    ArithmeticTarget::B => {
                        self.registers.b = self.sra(self.registers.b);
                    }
                    ArithmeticTarget::C => {
                        self.registers.c = self.sra(self.registers.c);
                    }
                    ArithmeticTarget::D => {
                        self.registers.d = self.sra(self.registers.d);
                    }
                    ArithmeticTarget::E => {
                        self.registers.e = self.sra(self.registers.e);
                    }
                    ArithmeticTarget::H => {
                        self.registers.h = self.sra(self.registers.h);
                    }
                    ArithmeticTarget::L => {
                        self.registers.l = self.sra(self.registers.l);
                    }
                    _ => panic!("Invalid target"),
                }
            }
            Instruction::SLA(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = self.sla(self.registers.a);
                    }
                    ArithmeticTarget::B => {
                        self.registers.b = self.sla(self.registers.b);
                    }
                    ArithmeticTarget::C => {
                        self.registers.c = self.sla(self.registers.c);
                    }
                    ArithmeticTarget::D => {
                        self.registers.d = self.sla(self.registers.d);
                    }
                    ArithmeticTarget::E => {
                        self.registers.e = self.sla(self.registers.e);
                    }
                    ArithmeticTarget::H => {
                        self.registers.h = self.sla(self.registers.h);
                    }
                    ArithmeticTarget::L => {
                        self.registers.l = self.sla(self.registers.l);
                    }
                    _ => panic!("Invalid target"),
                }
            }
            Instruction::SWAP(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = self.swap(self.registers.a);
                    }
                    ArithmeticTarget::B => {
                        self.registers.b = self.swap(self.registers.b);
                    }
                    ArithmeticTarget::C => {
                        self.registers.c = self.swap(self.registers.c);
                    }
                    ArithmeticTarget::D => {
                        self.registers.d = self.swap(self.registers.d);
                    }
                    ArithmeticTarget::E => {
                        self.registers.e = self.swap(self.registers.e);
                    }
                    ArithmeticTarget::H => {
                        self.registers.h = self.swap(self.registers.h);
                    }
                    ArithmeticTarget::L => {
                        self.registers.l = self.swap(self.registers.l);
                    }
                    _ => panic!("Invalid target"),
                }
            }
        }
    }

    fn jump(&mut self, should_jump: bool) {
        if should_jump {
            let least_significant = self.bus.read_byte(self.pc) as u16;
            let most_significant = self.bus.read_byte(self.pc + 1) as u16;
            self.pc = (most_significant << 8) | least_significant
        } else {
            self.pc = self.pc.wrapping_add(2)
        }
    }
    
    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.sub = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.h_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
    fn addhl(&mut self, value: u16) -> u16 {
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
        self.registers.f.sub = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.h_carry = (self.registers.get_hl() & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
    fn adc(&mut self, value:u8) -> u8 {
        let carry = self.registers.f.carry as u8;
        let (v1, o1) = self.registers.a.overflowing_add(value);
        let (v2, o2) = v1.overflowing_add(carry);
        self.registers.f.zero = v2 == 0;
        self.registers.f.sub = false;
        self.registers.f.carry = o1 || o2;
        self.registers.f.h_carry = (self.registers.a & 0xF) + (value & 0xF) + carry > 0xF;
        v2
    }
    fn sub(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.sub = true;
        self.registers.f.carry = did_overflow;
        self.registers.f.h_carry = (self.registers.a & 0xF) < (value & 0xF);
        new_value
    }
    fn sbc(&mut self, value:u8) -> u8 {
        let carry = self.registers.f.carry as u8;
        let (v1, o1) = self.registers.a.overflowing_sub(value);
        let (v2, o2) = v1.overflowing_sub(carry);
        self.registers.f.zero = v2 == 0;
        self.registers.f.sub = true;
        self.registers.f.carry = o1 || o2;
        self.registers.f.h_carry = (self.registers.a & 0xF) < (value & 0xF);
        v2
    }
    fn and(&mut self, value: u8) -> u8 {
        let new_value = value & self.registers.a;
        self.registers.f.zero = new_value == 0;
        self.registers.f.sub = false;
        self.registers.f.carry = false;
        self.registers.f.h_carry = true;
        new_value
    }
    fn or(&mut self, value: u8) -> u8 {
        let new_value = value | self.registers.a;
        self.registers.f.zero = new_value == 0;
        self.registers.f.sub = false;
        self.registers.f.carry = false;
        self.registers.f.h_carry = false;
        new_value
    }
    fn xor(&mut self, value: u8) -> u8 {
        let new_value = value ^ self.registers.a;
        self.registers.f.zero = new_value == 0;
        self.registers.f.sub = false;
        self.registers.f.carry = false;
        self.registers.f.h_carry = false;
        new_value
    }
    fn cp(&mut self, value: u8) {
        let old_a = self.registers.a;
        self.sub(value);
        self.registers.a = old_a;
    }
    fn inc(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.sub = false;
        self.registers.f.h_carry = (value & 0xF) + 1 > 0xF;
        new_value
    }
    fn dec(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.sub = true;
        self.registers.f.h_carry = (value & 0xF) == 0;
        new_value
    }
    fn ccf(&mut self) {
        self.registers.f.sub = false;
        self.registers.f.h_carry = false;
        self.registers.f.carry = !self.registers.f.carry;
    }
    fn scf(&mut self) {
        self.registers.f.sub = false;
        self.registers.f.h_carry = false;
        self.registers.f.carry = true;
    }
    fn rra(&mut self) {
        let old_carry = self.registers.f.carry as u8;
        let new_carry = self.registers.a & 0x1;
        self.registers.a = (self.registers.a >> 1) | (old_carry << 7);
        self.registers.f.zero = false;
        self.registers.f.sub = false;
        self.registers.f.carry = new_carry != 0;
        self.registers.f.h_carry = false;
    }
    fn rla(&mut self) {
        let old_carry = self.registers.f.carry as u8;
        let new_carry = self.registers.a & 0x80;
        self.registers.a = (self.registers.a << 1) | old_carry;
        self.registers.f.zero = false;
        self.registers.f.sub = false;
        self.registers.f.carry = new_carry != 0;
        self.registers.f.h_carry = false;
    }
    fn rrca(&mut self) {
        let bit = self.registers.a & 0x1;
        self.registers.a = (self.registers.a >> 1) | (bit << 7);
        self.registers.f.zero = false;
        self.registers.f.sub = false;
        self.registers.f.carry = bit != 0;
        self.registers.f.h_carry = false;
    }
    fn rlca(&mut self) {
        let bit = (self.registers.a & 0x80) >> 7;
        self.registers.a = (self.registers.a << 1) | bit;
        self.registers.f.zero = false;
        self.registers.f.sub = false;
        self.registers.f.carry = bit != 0;
        self.registers.f.h_carry = false;
    }
    fn cpl(&mut self) {
        self.registers.a = !self.registers.a;
        self.registers.f.sub = true;
        self.registers.f.h_carry = true;
    }
    fn bit(&mut self, bit: u8, value: u8) {
        self.registers.f.zero = (value >> bit) & 0x1 == 0;
        self.registers.f.sub = false;
        self.registers.f.h_carry = true;
    }
    fn reset(&mut self, bit: u8, value: u8) -> u8 {
        value & !(1 << bit)
    }
    fn set(&mut self, bit: u8, value: u8) -> u8 {
        value | (1 << bit)
    }
    fn srl(&mut self, value: u8) -> u8 {
        let new_value = value >> 1;
        self.registers.f.zero = new_value == 0;
        self.registers.f.sub = false;
        self.registers.f.h_carry = false;
        self.registers.f.carry = (value & 0x1) != 0;
        new_value
    }
    fn rr(&mut self, value: u8) -> u8 {
        let old_carry = self.registers.f.carry as u8;
        let new_carry = value & 0x1;
        let new_value = (value >> 1) | (old_carry << 7);
        self.registers.f.zero = new_value == 0;
        self.registers.f.sub = false;
        self.registers.f.h_carry = false;
        self.registers.f.carry = new_carry != 0;
        new_value
    }
    fn rl(&mut self, value: u8) -> u8 {
        let old_carry = self.registers.f.carry as u8;
        let new_carry = value & 0x80;
        let new_value = (value << 1) | old_carry;
        self.registers.f.zero = new_value == 0;
        self.registers.f.sub = false;
        self.registers.f.h_carry = false;
        self.registers.f.carry = new_carry != 0;
        new_value
    }
    fn rrc(&mut self, value: u8) -> u8 {
        let bit = value & 0x1;
        let new_value = (value >> 1) | (bit << 7);
        self.registers.f.zero = new_value == 0;
        self.registers.f.sub = false;
        self.registers.f.h_carry = false;
        self.registers.f.carry = bit != 0;
        new_value
    }
    fn rlc(&mut self, value: u8) -> u8 {
        let bit = value & 0x80;
        let new_value = (value << 1) | bit;
        self.registers.f.zero = new_value == 0;
        self.registers.f.sub = false;
        self.registers.f.h_carry = false;
        self.registers.f.carry = bit != 0;
        new_value
    }
    fn sra(&mut self, value: u8) -> u8 {
        let bit = value & 0x80;
        let new_carry = value & 0x1;
        let new_value = (value >> 1) | bit;
        self.registers.f.zero = new_value == 0;
        self.registers.f.sub = false;
        self.registers.f.h_carry = false;
        self.registers.f.carry = new_carry != 0;
        new_value
    }
    fn sla(&mut self, value: u8) -> u8 {
        let new_carry = value & 0x80;
        let new_value = value << 1;
        self.registers.f.zero = new_value == 0;
        self.registers.f.sub = false;
        self.registers.f.h_carry = false;
        self.registers.f.carry = new_carry != 0;
        new_value
    }
    fn swap(&mut self, value: u8) -> u8 {
        let new_value = (value & 0x0F) << 4 | (value & 0xF0) >> 4;
        self.registers.f.zero = new_value == 0;
        self.registers.f.sub = false;
        self.registers.f.h_carry = false;
        self.registers.f.carry = false;
        new_value
    }
}
