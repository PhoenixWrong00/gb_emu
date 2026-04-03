use crate::inst::{Instruction, ArithmeticTarget};
use crate::registers::{Registers};


pub struct CPU {
    registers: Registers,
    pc: u16,
    sp: u16,
    bus: MemoryBus,
}

impl CPU {
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
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
    fn rrla(&mut self) {
        let bit = self.registers.a & 0x80;
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
}
