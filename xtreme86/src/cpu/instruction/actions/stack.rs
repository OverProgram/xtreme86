use crate::cpu::{CPU, Regs};
use crate::cpu::instruction::args::{SrcArg, DstArg, Size};
use crate::cpu::instruction::Instruction;


pub fn push(comp: &mut CPU, instruction: Instruction) -> usize {
    let arg = instruction.dst.clone().unwrap().to_src_arg(comp).unwrap();
    comp.instruction.as_mut().map(|mut s| { s.segment = Regs::SS });
    comp.write_to_arg(DstArg::Ptr(comp.read_reg(Regs::SP).unwrap() - 1, Size::Word), arg).expect("Err");
    comp.regs.get_mut(&Regs::SP).unwrap().value -= 2;
    1
}

pub fn push_mnemonic(_: u8) -> Option<String> {
                                            Some(String::from("PUSH"))
                                                                       }

pub fn pop(comp: &mut CPU, instruction: Instruction) -> usize {
    let val = SrcArg::Word(comp.read_mem_word_seg(comp.read_reg(Regs::SP).unwrap() + 1, Regs::SS).unwrap());
    comp.write_to_arg(instruction.dst.clone().unwrap(), val).unwrap();
    comp.regs.get_mut(&Regs::SP).unwrap().value += 2;
    1
}

pub fn pop_mnemonic(_: u8) -> Option<String> {
                                           Some(String::from("POP"))
                                                                     }

pub fn far_call(comp: &mut CPU, instruction: Instruction) -> usize {
    comp.sub_command(0xFF, None, Some(DstArg::Reg(Regs::CS)), 0b110);
    comp.sub_command(0xFF, None, Some(DstArg::Reg(Regs::IP)), 0b110);
    let arg = instruction.dst;
    comp.sub_command(0xFF, None, arg, 0b101);
    0
}

pub fn near_call(comp: &mut CPU, instruction: Instruction) -> usize {
    comp.sub_command(0xFF, None, Some(DstArg::Reg(Regs::IP)), 0b110);
    match instruction.dst.clone().unwrap() {
        DstArg::Imm16(val) => {
            comp.sub_command(0xE9, None, Some(DstArg::Imm16(val)), 0);
        },
        _ => {
            let dst = instruction.dst.clone().unwrap();
            let val_src = dst.to_src_arg(comp);
            if let Some(src) = val_src {
                comp.write_to_arg(DstArg::Reg(Regs::IP), src).unwrap();
            }
        }
    }
    0
}

pub fn call_mnemonic(_: u8) -> Option<String> {
    Some(String::from("CALL"))
                                                                       }

pub fn ret(comp: &mut CPU, instruction: Instruction) -> usize {
    comp.sub_command(0x8F, None, Some(DstArg::Reg(Regs::IP)), 0b000);
    comp.sub_command(0xE9, None, Some(DstArg::Reg(Regs::IP)), 0b000);
    0
}

pub fn ret_mnemonic(_: u8) -> Option<String> {
                                           Some(String::from("RET"))
                                                                     }

pub fn enter(comp: &mut CPU, instruction: Instruction) -> usize {
    let dst_arg = instruction.dst.unwrap();
    let dst = match dst_arg.to_src_arg(comp) {
        Some(SrcArg::Word(val)) => val,
        _ => panic!("First operand for ENTER must be a word")
    };
    let level = match instruction.src.clone().unwrap().to_src_arg(comp) {
        Some(SrcArg::Byte(val)) => val % 13,
        _ => panic!("Second operand for ENTER must be a byte")
    };
    comp.sub_command(0xFE, None, Some(DstArg::Reg(Regs::BP)), 0b110);
    let frame_ptr = comp.regs.get(&Regs::SP).unwrap().value;
    if level > 0 {
        for _ in 1..level {
            let new_bp = comp.regs.get(&Regs::BP).unwrap().value - 2;
            comp.regs.get_mut(&Regs::BP).unwrap().value = new_bp;
            comp.sub_command(0xFE, None, Some(DstArg::Ptr(new_bp, Size::Word)), 0b110);
        }
        comp.sub_command(0xFE, None, Some(DstArg::Imm16(frame_ptr)), 0b110);
    }
    comp.regs.get_mut(&Regs::BP).unwrap().value = frame_ptr;
    let new_sp = comp.regs.get(&Regs::SP).unwrap().value - dst;
    comp.regs.get_mut(&Regs::SP).unwrap().value = new_sp;
    println!("in enter");
    0
}

pub fn enter_mnemonic(_: u8) -> Option<String> {
    Some(String::from("ENTER"))
}

pub fn leave(comp: &mut CPU, instruction: Instruction) -> usize {
    let new_sp = comp.regs.get(&Regs::BP).unwrap().value;
    comp.write_to_arg(DstArg::Reg(Regs::SP), SrcArg::Word(new_sp)).unwrap();
    comp.sub_command(0x8F, None, Some(DstArg::Reg(Regs::BP)), 0);
    0
}

pub fn leave_mnemonic(_: u8) -> Option<String> {
    Some(String::from("LEAVE"))
}
