use crate::cpu::{CPU, SrcArg, DstArg, Regs, exceptions};

pub fn int_req(comp: &mut CPU) -> usize {
    let num = get_int_num(comp);
    comp.irq = Some(num);
    0
}

pub fn int_mnemonic(_: u8) -> Option<String> {
                                           Some(String::from("INT"))
                                                                     }

fn get_int_num(comp: &mut CPU) -> u8 {
    match comp.get_src_arg_mut(comp.dst.clone().unwrap()).unwrap() {
        SrcArg::Byte(val) => Some(val),
        _ => None
    }.unwrap()
}

pub fn int(comp: &mut CPU) -> usize {
    let tmp_es = comp.read_reg(Regs::ES).unwrap();

    comp.sub_command(0xFF, None, Some(DstArg::Reg(Regs::FLAGS)), 0b110);
    comp.sub_command(0xFF, None, Some(DstArg::Reg(Regs::CS)), 0b110);
    comp.sub_command(0xFF, None, Some(DstArg::Reg(Regs::IP)), 0b110);

    let num = comp.irq.unwrap();
    comp.irq = None;

    let new_cs = comp.read_mem_word_seg((num as u16) * 4 + 2, Regs::ES).unwrap();
    let new_ip = comp.read_mem_word_seg((num as u16) * 4, Regs::ES).unwrap();
    comp.write_to_arg(DstArg::Reg(Regs::CS), SrcArg::Word(new_cs)).unwrap();
    comp.write_to_arg(DstArg::Reg(Regs::IP), SrcArg::Word(new_ip)).unwrap();

    comp.set_reg(Regs::ES, tmp_es);
    0
}

pub fn iret(comp: &mut CPU) -> usize {
    comp.sub_command(0x8F, None, Some(DstArg::Reg(Regs::IP)), 0b110);
    comp.sub_command(0x8F, None, Some(DstArg::Reg(Regs::CS)), 0b110);
    comp.sub_command(0x8F, None, Some(DstArg::Reg(Regs::FLAGS)), 0b110);
    0
}

pub fn iret_mnemonic(_: u8) -> Option<String> {
                                            Some(String::from("IRET"))
                                                                       }

pub fn bound(comp: &mut CPU) -> usize {
    if let Some(SrcArg::DWord(bounds)) = comp.src.clone() {
        match comp.dst.clone().unwrap() {
            DstArg::Reg16(_) | DstArg::Reg(_) => (),
            _ => comp.except(exceptions::INVALID_OPCODE).unwrap()
        }
        let lower_bound = (bounds & 0xFFFF) as u16;
        let upper_bound = (bounds >> 16) as u16;
        let arg = comp.get_src_arg_mut(comp.dst.clone().unwrap());
        if let Some(SrcArg::Word(val)) = arg {
            if val > upper_bound || val < lower_bound {
                comp.except(exceptions::BOUND).unwrap();
            }
        }
    }
    0
}

pub fn bound_mnemonic(_: u8) -> Option<String> {
    Some(String::from("BOUND"))
}