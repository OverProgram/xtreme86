/// This file was generated by the Python script in utils/opcode_generator

use crate::cpu::instruction::opcode::{Opcode, Mnemonic, NumArgs, Placeholder};
use crate::cpu::{Regs, CPU, CPUFlags};
use crate::cpu::instruction::actions::{alu, flags, int, jmp, mem, stack};
use enumflags2::make_bitflags;
use crate::cpu::instruction::opcode::OpcodeFlags;
use std::rc::Rc;

impl Opcode {
	pub fn get_opcode_data() -> [Option<Opcode>; 256] {
		[
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(alu::add), mnemonic: Mnemonic::Static(String::from("add")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			None,
			None,
			None,
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(alu::add), mnemonic: Mnemonic::Static(String::from("add")), shorthand1: Some(Placeholder::Reg(0)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			None,
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::push), mnemonic: Mnemonic::Static(String::from("push")), shorthand1: Some(Placeholder::RegEnum(Regs::ES)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::pop), mnemonic: Mnemonic::Static(String::from("pop")), shorthand1: Some(Placeholder::RegEnum(Regs::ES)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(alu::or), mnemonic: Mnemonic::Static(String::from("or")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			None,
			None,
			None,
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(alu::or), mnemonic: Mnemonic::Static(String::from("or")), shorthand1: Some(Placeholder::Reg(0)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			None,
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::push), mnemonic: Mnemonic::Static(String::from("push")), shorthand1: Some(Placeholder::RegEnum(Regs::CS)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			None,
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(alu::adc), mnemonic: Mnemonic::Static(String::from("adc")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			None,
			None,
			None,
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(alu::adc), mnemonic: Mnemonic::Static(String::from("adc")), shorthand1: Some(Placeholder::Reg8(0)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			None,
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::push), mnemonic: Mnemonic::Static(String::from("push")), shorthand1: Some(Placeholder::RegEnum(Regs::SS)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::pop), mnemonic: Mnemonic::Static(String::from("pop")), shorthand1: Some(Placeholder::RegEnum(Regs::SS)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(alu::sbb), mnemonic: Mnemonic::Static(String::from("sbb")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			None,
			None,
			None,
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(alu::sbb), mnemonic: Mnemonic::Static(String::from("sbb")), shorthand1: Some(Placeholder::Reg(0)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			None,
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::push), mnemonic: Mnemonic::Static(String::from("push")), shorthand1: Some(Placeholder::RegEnum(Regs::DS)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::pop), mnemonic: Mnemonic::Static(String::from("pop")), shorthand1: Some(Placeholder::RegEnum(Regs::DS)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(alu::and), mnemonic: Mnemonic::Static(String::from("and")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			None,
			None,
			None,
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(alu::and), mnemonic: Mnemonic::Static(String::from("and")), shorthand1: Some(Placeholder::Reg(0)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			None,
			None,
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(alu::daa), mnemonic: Mnemonic::Static(String::from("daa")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(alu::sub), mnemonic: Mnemonic::Static(String::from("sub")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			None,
			None,
			None,
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(alu::sub), mnemonic: Mnemonic::Static(String::from("sub")), shorthand1: Some(Placeholder::Reg(0)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			None,
			None,
			None,
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(alu::xor), mnemonic: Mnemonic::Static(String::from("xor")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			None,
			None,
			None,
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(alu::xor), mnemonic: Mnemonic::Static(String::from("xor")), shorthand1: Some(Placeholder::Reg(0)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			None,
			None,
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(alu::aaa), mnemonic: Mnemonic::Static(String::from("aaa")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(flags::cmp), mnemonic: Mnemonic::Static(String::from("cmp")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ SizeMismatch }), segment: Regs::DS }),
			None,
			None,
			None,
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(flags::cmp), mnemonic: Mnemonic::Static(String::from("cmp")), shorthand1: Some(Placeholder::Reg(0)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			None,
			None,
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(alu::aas), mnemonic: Mnemonic::Static(String::from("aas")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(alu::inc), mnemonic: Mnemonic::Static(String::from("inc")), shorthand1: Some(Placeholder::Reg16(0)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(alu::inc), mnemonic: Mnemonic::Static(String::from("inc")), shorthand1: Some(Placeholder::Reg16(1)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(alu::inc), mnemonic: Mnemonic::Static(String::from("inc")), shorthand1: Some(Placeholder::Reg16(2)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(alu::inc), mnemonic: Mnemonic::Static(String::from("inc")), shorthand1: Some(Placeholder::Reg16(3)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(alu::inc), mnemonic: Mnemonic::Static(String::from("inc")), shorthand1: Some(Placeholder::Reg16(4)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(alu::inc), mnemonic: Mnemonic::Static(String::from("inc")), shorthand1: Some(Placeholder::Reg16(5)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(alu::inc), mnemonic: Mnemonic::Static(String::from("inc")), shorthand1: Some(Placeholder::Reg16(6)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(alu::inc), mnemonic: Mnemonic::Static(String::from("inc")), shorthand1: Some(Placeholder::Reg16(7)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(alu::dec), mnemonic: Mnemonic::Static(String::from("dec")), shorthand1: Some(Placeholder::Reg16(0)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(alu::dec), mnemonic: Mnemonic::Static(String::from("dec")), shorthand1: Some(Placeholder::Reg16(1)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(alu::dec), mnemonic: Mnemonic::Static(String::from("dec")), shorthand1: Some(Placeholder::Reg16(2)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(alu::dec), mnemonic: Mnemonic::Static(String::from("dec")), shorthand1: Some(Placeholder::Reg16(3)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(alu::dec), mnemonic: Mnemonic::Static(String::from("dec")), shorthand1: Some(Placeholder::Reg16(4)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(alu::dec), mnemonic: Mnemonic::Static(String::from("dec")), shorthand1: Some(Placeholder::Reg16(5)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(alu::dec), mnemonic: Mnemonic::Static(String::from("dec")), shorthand1: Some(Placeholder::Reg16(6)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(alu::dec), mnemonic: Mnemonic::Static(String::from("dec")), shorthand1: Some(Placeholder::Reg16(7)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::push), mnemonic: Mnemonic::Static(String::from("push")), shorthand1: Some(Placeholder::Reg16(0)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::push), mnemonic: Mnemonic::Static(String::from("push")), shorthand1: Some(Placeholder::Reg16(1)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::push), mnemonic: Mnemonic::Static(String::from("push")), shorthand1: Some(Placeholder::Reg16(2)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::push), mnemonic: Mnemonic::Static(String::from("push")), shorthand1: Some(Placeholder::Reg16(3)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::push), mnemonic: Mnemonic::Static(String::from("push")), shorthand1: Some(Placeholder::Reg16(4)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::push), mnemonic: Mnemonic::Static(String::from("push")), shorthand1: Some(Placeholder::Reg16(5)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::push), mnemonic: Mnemonic::Static(String::from("push")), shorthand1: Some(Placeholder::Reg16(6)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::push), mnemonic: Mnemonic::Static(String::from("push")), shorthand1: Some(Placeholder::Reg16(7)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::pop), mnemonic: Mnemonic::Static(String::from("pop")), shorthand1: Some(Placeholder::Reg16(0)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::pop), mnemonic: Mnemonic::Static(String::from("pop")), shorthand1: Some(Placeholder::Reg16(1)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::pop), mnemonic: Mnemonic::Static(String::from("pop")), shorthand1: Some(Placeholder::Reg16(2)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::pop), mnemonic: Mnemonic::Static(String::from("pop")), shorthand1: Some(Placeholder::Reg16(3)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::pop), mnemonic: Mnemonic::Static(String::from("pop")), shorthand1: Some(Placeholder::Reg16(4)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::pop), mnemonic: Mnemonic::Static(String::from("pop")), shorthand1: Some(Placeholder::Reg16(5)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::pop), mnemonic: Mnemonic::Static(String::from("pop")), shorthand1: Some(Placeholder::Reg16(6)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::pop), mnemonic: Mnemonic::Static(String::from("pop")), shorthand1: Some(Placeholder::Reg16(7)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(stack::pusha), mnemonic: Mnemonic::Static(String::from("pusha")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(stack::popa), mnemonic: Mnemonic::Static(String::from("popa")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(int::bound), mnemonic: Mnemonic::Static(String::from("bound")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ ForceDWord }), segment: Regs::DS }),
			None,
			None,
			None,
			None,
			None,
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::push), mnemonic: Mnemonic::Static(String::from("push")), shorthand1: Some(Placeholder::Imm), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | ForceWord }), segment: Regs::DS }),
			None,
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::push), mnemonic: Mnemonic::Static(String::from("push")), shorthand1: Some(Placeholder::Imm), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | ForceByte }), segment: Regs::DS }),
			None,
			None,
			None,
			None,
			None,
			Some(Opcode{ num_args: NumArgs::One, action: jmp::cond_jmp(Box::new(|this: &CPU| this.check_flag(CPUFlags::OVERFLOW))), mnemonic: Mnemonic::Static(String::from("jo")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | SizeMismatch }), segment: Regs::CS }),
			Some(Opcode{ num_args: NumArgs::One, action: jmp::cond_jmp(Box::new(|this: &CPU| !this.check_flag(CPUFlags::OVERFLOW))), mnemonic: Mnemonic::Static(String::from("jno")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | SizeMismatch }), segment: Regs::CS }),
			Some(Opcode{ num_args: NumArgs::One, action: jmp::cond_jmp(Box::new(|this: &CPU| this.check_flag(CPUFlags::CARRY))), mnemonic: Mnemonic::Static(String::from("jc")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | SizeMismatch }), segment: Regs::CS }),
			Some(Opcode{ num_args: NumArgs::One, action: jmp::cond_jmp(Box::new(|this: &CPU| !this.check_flag(CPUFlags::CARRY))), mnemonic: Mnemonic::Static(String::from("jnc")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | SizeMismatch }), segment: Regs::CS }),
			Some(Opcode{ num_args: NumArgs::One, action: jmp::cond_jmp(Box::new(|this: &CPU| this.check_flag(CPUFlags::ZERO))), mnemonic: Mnemonic::Static(String::from("je")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | SizeMismatch }), segment: Regs::CS }),
			Some(Opcode{ num_args: NumArgs::One, action: jmp::cond_jmp(Box::new(|this: &CPU| !this.check_flag(CPUFlags::ZERO))), mnemonic: Mnemonic::Static(String::from("jne")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | SizeMismatch }), segment: Regs::CS }),
			Some(Opcode{ num_args: NumArgs::One, action: jmp::cond_jmp(Box::new(|this: &CPU| this.check_flag(CPUFlags::CARRY) || this.check_flag(CPUFlags::ZERO))), mnemonic: Mnemonic::Static(String::from("jbe")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | SizeMismatch }), segment: Regs::CS }),
			Some(Opcode{ num_args: NumArgs::One, action: jmp::cond_jmp(Box::new(|this: &CPU| !this.check_flag(CPUFlags::CARRY) && !this.check_flag(CPUFlags::ZERO))), mnemonic: Mnemonic::Static(String::from("ja")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | SizeMismatch }), segment: Regs::CS }),
			Some(Opcode{ num_args: NumArgs::One, action: jmp::cond_jmp(Box::new(|this: &CPU| this.check_flag(CPUFlags::SIGN))), mnemonic: Mnemonic::Static(String::from("js")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | SizeMismatch }), segment: Regs::CS }),
			Some(Opcode{ num_args: NumArgs::One, action: jmp::cond_jmp(Box::new(|this: &CPU| !this.check_flag(CPUFlags::SIGN))), mnemonic: Mnemonic::Static(String::from("jns")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | SizeMismatch }), segment: Regs::CS }),
			Some(Opcode{ num_args: NumArgs::One, action: jmp::cond_jmp(Box::new(|this: &CPU| this.check_flag(CPUFlags::PARITY))), mnemonic: Mnemonic::Static(String::from("jp")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | SizeMismatch }), segment: Regs::CS }),
			Some(Opcode{ num_args: NumArgs::One, action: jmp::cond_jmp(Box::new(|this: &CPU| !this.check_flag(CPUFlags::PARITY))), mnemonic: Mnemonic::Static(String::from("jnp")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | SizeMismatch }), segment: Regs::CS }),
			Some(Opcode{ num_args: NumArgs::One, action: jmp::cond_jmp(Box::new(|this: &CPU| this.check_flags_not_equal(CPUFlags::SIGN, CPUFlags::OVERFLOW))), mnemonic: Mnemonic::Static(String::from("jl")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | SizeMismatch }), segment: Regs::CS }),
			Some(Opcode{ num_args: NumArgs::One, action: jmp::cond_jmp(Box::new(|this: &CPU| !this.check_flags_not_equal(CPUFlags::SIGN, CPUFlags::OVERFLOW))), mnemonic: Mnemonic::Static(String::from("jge")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | SizeMismatch }), segment: Regs::CS }),
			Some(Opcode{ num_args: NumArgs::One, action: jmp::cond_jmp(Box::new(|this: &CPU| this.check_flags_not_equal(CPUFlags::SIGN, CPUFlags::OVERFLOW) || this.check_flag(CPUFlags::ZERO))), mnemonic: Mnemonic::Static(String::from("jle")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | SizeMismatch }), segment: Regs::CS }),
			Some(Opcode{ num_args: NumArgs::One, action: jmp::cond_jmp(Box::new(|this: &CPU| this.check_flag(CPUFlags::SIGN) && !this.check_flags_not_equal(CPUFlags::SIGN, CPUFlags::OVERFLOW))), mnemonic: Mnemonic::Static(String::from("jg")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | SizeMismatch }), segment: Regs::CS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(alu::alu_dispatch_two_args), mnemonic: Mnemonic::Dynamic(Rc::new(alu::alu_dispatch_two_args_mnemonic)), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			None,
			None,
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(alu::alu_dispatch_two_args), mnemonic: Mnemonic::Dynamic(Rc::new(alu::alu_dispatch_two_args_mnemonic)), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | SizeMismatch }), segment: Regs::DS }),
			None,
			None,
			None,
			None,
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(mem::mov), mnemonic: Mnemonic::Static(String::from("mov")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			None,
			None,
			None,
			None,
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(mem::lea), mnemonic: Mnemonic::Static(String::from("lea")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ ForceDirection }), segment: Regs::DS }),
			None,
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::pop), mnemonic: Mnemonic::Static(String::from("pop")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(mem::nop), mnemonic: Mnemonic::Static(String::from("nop")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Nop }), segment: Regs::DS }),
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(mem::cbw), mnemonic: Mnemonic::Static(String::from("cbw")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(mem::cdw), mnemonic: Mnemonic::Static(String::from("cbw")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::far_call), mnemonic: Mnemonic::Static(String::from("call")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ ForceWord | Immediate }), segment: Regs::DS }),
			None,
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(stack::push), mnemonic: Mnemonic::Static(String::from("pushf")), shorthand1: Some(Placeholder::RegEnum(Regs::FLAGS)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(stack::pop), mnemonic: Mnemonic::Static(String::from("popf")), shorthand1: Some(Placeholder::RegEnum(Regs::FLAGS)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(flags::sahf), mnemonic: Mnemonic::Static(String::from("sahf")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(flags::lahf), mnemonic: Mnemonic::Static(String::from("lahf")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(mem::mov), mnemonic: Mnemonic::Static(String::from("mov")), shorthand1: Some(Placeholder::Reg(0)), shorthand2: Some(Placeholder::Ptr), flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			None,
			None,
			None,
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(mem::movs), mnemonic: Mnemonic::Static(String::from("movsb")), shorthand1: Some(Placeholder::Byte(0)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(mem::movs), mnemonic: Mnemonic::Static(String::from("movsw")), shorthand1: Some(Placeholder::Word(0)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(flags::cmps), mnemonic: Mnemonic::Static(String::from("cmps")), shorthand1: Some(Placeholder::Reg16(6)), shorthand2: Some(Placeholder::Reg16(7)), flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			None,
			None,
			None,
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(mem::stos), mnemonic: Mnemonic::Static(String::from("stosb")), shorthand1: Some(Placeholder::Byte(0)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::ES }),
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(mem::stos), mnemonic: Mnemonic::Static(String::from("stosw")), shorthand1: Some(Placeholder::Word(0)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::ES }),
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(mem::lods), mnemonic: Mnemonic::Static(String::from("lodsb")), shorthand1: Some(Placeholder::Byte(0)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(mem::lods), mnemonic: Mnemonic::Static(String::from("lodsw")), shorthand1: Some(Placeholder::Word(0)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(flags::scas), mnemonic: Mnemonic::Static(String::from("scasb")), shorthand1: None, shorthand2: Some(Placeholder::Reg8(0)), flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::ES }),
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(flags::scas), mnemonic: Mnemonic::Static(String::from("scasw")), shorthand1: None, shorthand2: Some(Placeholder::Reg16(0)), flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::ES }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(mem::mov), mnemonic: Mnemonic::Static(String::from("mov")), shorthand1: Some(Placeholder::Reg8(0)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(mem::mov), mnemonic: Mnemonic::Static(String::from("mov")), shorthand1: Some(Placeholder::Reg8(1)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(mem::mov), mnemonic: Mnemonic::Static(String::from("mov")), shorthand1: Some(Placeholder::Reg8(2)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(mem::mov), mnemonic: Mnemonic::Static(String::from("mov")), shorthand1: Some(Placeholder::Reg8(3)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(mem::mov), mnemonic: Mnemonic::Static(String::from("mov")), shorthand1: Some(Placeholder::Reg8(4)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(mem::mov), mnemonic: Mnemonic::Static(String::from("mov")), shorthand1: Some(Placeholder::Reg8(5)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(mem::mov), mnemonic: Mnemonic::Static(String::from("mov")), shorthand1: Some(Placeholder::Reg8(6)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(mem::mov), mnemonic: Mnemonic::Static(String::from("mov")), shorthand1: Some(Placeholder::Reg8(7)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(mem::mov), mnemonic: Mnemonic::Static(String::from("mov")), shorthand1: Some(Placeholder::Reg16(0)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(mem::mov), mnemonic: Mnemonic::Static(String::from("mov")), shorthand1: Some(Placeholder::Reg16(1)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(mem::mov), mnemonic: Mnemonic::Static(String::from("mov")), shorthand1: Some(Placeholder::Reg16(2)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(mem::mov), mnemonic: Mnemonic::Static(String::from("mov")), shorthand1: Some(Placeholder::Reg16(3)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(mem::mov), mnemonic: Mnemonic::Static(String::from("mov")), shorthand1: Some(Placeholder::Reg16(4)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(mem::mov), mnemonic: Mnemonic::Static(String::from("mov")), shorthand1: Some(Placeholder::Reg16(5)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(mem::mov), mnemonic: Mnemonic::Static(String::from("mov")), shorthand1: Some(Placeholder::Reg16(6)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(mem::mov), mnemonic: Mnemonic::Static(String::from("mov")), shorthand1: Some(Placeholder::Reg16(7)), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(alu::rotate_dispatch), mnemonic: Mnemonic::Dynamic(Rc::new(alu::rotate_dispatch_mnemonic)), shorthand1: None, shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate | ForceByte | SizeMismatch }), segment: Regs::DS }),
			None,
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::near_ret), mnemonic: Mnemonic::Static(String::from("ret")), shorthand1: Some(Placeholder::Imm), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | ForceWord }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(stack::near_ret), mnemonic: Mnemonic::Static(String::from("ret")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(mem::ldw), mnemonic: Mnemonic::Static(String::from("les")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ ForceDWord }), segment: Regs::ES }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(mem::ldw), mnemonic: Mnemonic::Static(String::from("lds")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ ForceDWord }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(mem::mov), mnemonic: Mnemonic::Static(String::from("mov")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::DS }),
			None,
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(stack::enter), mnemonic: Mnemonic::Static(String::from("enter")), shorthand1: Some(Placeholder::Imm), shorthand2: Some(Placeholder::Imm), flags: make_bitflags!(OpcodeFlags::{ Immediate | SizeMismatch }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(stack::leave), mnemonic: Mnemonic::Static(String::from("leave")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::far_ret), mnemonic: Mnemonic::Static(String::from("ret")), shorthand1: Some(Placeholder::Imm), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | ForceWord }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(stack::far_ret), mnemonic: Mnemonic::Static(String::from("ret")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(int::int_req), mnemonic: Mnemonic::Static(String::from("int")), shorthand1: Some(Placeholder::Byte(3)), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | ForceByte }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(int::int_req), mnemonic: Mnemonic::Static(String::from("int")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | ForceByte }), segment: Regs::DS }),
			None,
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(int::iret), mnemonic: Mnemonic::Static(String::from("int")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(alu::rotate_dispatch), mnemonic: Mnemonic::Dynamic(Rc::new(alu::rotate_dispatch_mnemonic)), shorthand1: None, shorthand2: Some(Placeholder::Byte(1)), flags: make_bitflags!(OpcodeFlags::{ SizeMismatch }), segment: Regs::DS }),
			None,
			None,
			Some(Opcode{ num_args: NumArgs::Two, action: Rc::new(alu::rotate_dispatch), mnemonic: Mnemonic::Dynamic(Rc::new(alu::rotate_dispatch_mnemonic)), shorthand1: None, shorthand2: Some(Placeholder::Reg8(2)), flags: make_bitflags!(OpcodeFlags::{ SizeMismatch }), segment: Regs::DS }),
			None,
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(alu::aad), mnemonic: Mnemonic::Static(String::from("aad")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | ForceByte }), segment: Regs::DS }),
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			Some(Opcode{ num_args: NumArgs::One, action: jmp::lop(Box::new(|this: &CPU| !this.check_flag(CPUFlags::ZERO))), mnemonic: Mnemonic::Static(String::from("loopne")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | SizeMismatch }), segment: Regs::CS }),
			Some(Opcode{ num_args: NumArgs::One, action: jmp::lop(Box::new(|this: &CPU| this.check_flag(CPUFlags::ZERO))), mnemonic: Mnemonic::Static(String::from("loope")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | SizeMismatch }), segment: Regs::CS }),
			Some(Opcode{ num_args: NumArgs::One, action: jmp::lop(Box::new(|_: &CPU| true)), mnemonic: Mnemonic::Static(String::from("loop")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | SizeMismatch }), segment: Regs::CS }),
			Some(Opcode{ num_args: NumArgs::One, action: jmp::cond_jmp(Box::new(|this: &CPU| this.regs.get(&Regs::CX).unwrap().value == 0)), mnemonic: Mnemonic::Static(String::from("jcxz")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate | SizeMismatch }), segment: Regs::CS }),
			None,
			None,
			None,
			None,
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(stack::near_call), mnemonic: Mnemonic::Static(String::from("call")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ ForceWord | Immediate }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(jmp::jmp), mnemonic: Mnemonic::Static(String::from("jmp")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{ Immediate }), segment: Regs::CS }),
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(flags::repne), mnemonic: Mnemonic::Static(String::from("repne")), shorthand1: Some(Placeholder::Opcode), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(flags::rep), mnemonic: Mnemonic::Dynamic(Rc::new(flags::rep_mnemonic)), shorthand1: Some(Placeholder::Opcode), shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			None,
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(flags::cmc), mnemonic: Mnemonic::Static(String::from("cmc")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(alu::mul_dispatch), mnemonic: Mnemonic::Dynamic(Rc::new(alu::mul_dispatch_mnemonic)), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			None,
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(flags::clc), mnemonic: Mnemonic::Static(String::from("clc")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(flags::stc), mnemonic: Mnemonic::Static(String::from("stc")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(flags::cli), mnemonic: Mnemonic::Static(String::from("cli")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(flags::sti), mnemonic: Mnemonic::Static(String::from("sti")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(flags::cld), mnemonic: Mnemonic::Static(String::from("cld")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::Zero, action: Rc::new(flags::std), mnemonic: Mnemonic::Static(String::from("std")), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			Some(Opcode{ num_args: NumArgs::One, action: Rc::new(alu::alu_dispatch_one_arg), mnemonic: Mnemonic::Dynamic(Rc::new(alu::alu_dispatch_one_arg_mnemonic)), shorthand1: None, shorthand2: None, flags: make_bitflags!(OpcodeFlags::{  }), segment: Regs::DS }),
			None,
		]
	}
}

