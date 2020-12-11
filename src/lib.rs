use std::fs::File;
use std::fs;
use std::io::Read;

pub mod cpu;

fn new_cpu_from_file(filename: &str) -> cpu::CPU {
    let mut computer = cpu::CPU::new(0x7FFFFF);
    computer.set_reg(cpu::Regs::SS, 0x003F);
    computer.set_reg(cpu::Regs::CS, 0x103F);
    computer.set_reg(cpu::Regs::DS, 0x203F);
    computer.set_reg(cpu::Regs::SP, 0xFFFF);
    computer.set_reg(cpu::Regs::IP, 0x0000);

    let mut f = File::open(&filename).expect("No file found!");
    let metadata = fs::metadata(&filename).expect("No file found!");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("Couldn't read file!");
    computer.load(buffer, 0x103F0);

    computer
}

fn new_cpu_vec(code: Vec<u8>) -> cpu::CPU {
    let mut computer = cpu::CPU::new(code.len());
    computer.load(code, 0);
    computer
}

#[cfg(test)]
mod mov_test {
    use super::cpu;

    #[test]
    fn test_mov_reg_shorthand() {
        let code = vec![0xB8, 0x06, 0x00];    // mov ax, 0x6
        let mut computer = cpu::CPU::new(code.len());
        computer.load(code, 0);
        computer.execute_next();
        assert_eq!(computer.read_reg(cpu::Regs::AX).unwrap(), 6);
    }

    #[test]
    fn test_mov_reg() {
        let code = vec![0x66, 0xc6, 0x6, 0x0, 0x0, 0x0];
        let mut computer = cpu::CPU::new(code.len());
        computer.load(code, 0);
        computer.execute_next_from(1);
        assert_eq!(computer.read_mem(0), 0x0);
    }

    #[test]
    fn test_mov_ptr() {
        let code = vec![0x00, 0xc6, 0x06, 0x00, 0x00, 0x55];
        let mut computer = cpu::CPU::new(code.len());
        computer.load(code, 0);
        computer.execute_next_from(1);
        assert_eq!(computer.read_mem(0), 0x55);
    }
}

#[cfg(test)]
mod test_alu {
    use super::cpu;
    use crate::cpu::Regs;
    use crate::new_cpu_vec;

    #[test]
    fn test_add() {
        let code = vec![0x83, 0xc0, 0x5];
        let mut computer = cpu::CPU::new(code.len());
        computer.load(code, 0);
        computer.execute_next();
        assert_eq!(computer.read_reg(cpu::Regs::AX).unwrap(), 5);
    }

    #[test]
    fn test_sub() {
        let code = vec![0x30, 0x80, 0x2e, 0x0, 0x0, 0x20];
        let mut computer = cpu::CPU::new(code.len());
        computer.load(code, 0);
        computer.execute_next_from(1);
        assert_eq!(computer.read_mem(0), 0x10);
    }

    #[test]
    fn test_and() {
        let code = vec![0xb8, 0xFF, 0x0, 0xbb, 0xaa, 0x0, 0x21, 0xc3];
        let mut computer = cpu::CPU::new(code.len());
        computer.load(code, 0);
        computer.execute_next();
        computer.execute_next();
        computer.execute_next();
        assert_eq!(computer.read_reg(Regs::BX).unwrap(), 0xAA);
    }

    #[test]
    fn test_or() {
        let code = vec![0xaa, 0x00, 0xb9, 0x55, 0x0, 0xb, 0xe, 0x0, 0x0];
        let mut computer = cpu::CPU::new(code.len());
        computer.load(code, 0);
        computer.execute_next_from(2);
        computer.execute_next();
        assert_eq!(computer.read_reg(Regs::CX).unwrap(), 0xFF);
    }

    #[test]
    fn test_inc_reg() {
        let code = vec![0x40];
        let mut computer = cpu::CPU::new(code.len());
        computer.load(code, 0);
        computer.execute_next();
        assert_eq!(computer.read_reg(Regs::AX).unwrap(), 0x01);
    }

    #[test]
    fn test_dec_mem() {
        let code = vec![0xff, 0xfe, 0xe, 0x0, 0x0];
        let mut computer = cpu::CPU::new(code.len());
        computer.load(code, 0);
        computer.execute_next_from(1);
        assert_eq!(computer.read_mem(0), 0xFE)
    }

    #[test]
    fn test_carry() {
        let mut computer = new_cpu_vec(vec![0xff, 0xfe, 0x6, 0x0, 0x0]);
        computer.execute_next_from(1);
        assert_eq!(computer.read_reg(Regs::FLAGS).unwrap() & (cpu::CPUFlags::CARRY | cpu::CPUFlags::ZERO), (cpu::CPUFlags::CARRY | cpu::CPUFlags::ZERO));
    }

    #[test]
    fn test_mul() {
        let mut computer = new_cpu_vec(vec![0xb8, 0x55, 0x0, 0xbb, 0xaa, 0x0, 0xf7, 0xe3]);
        computer.execute_next();
        computer.execute_next();
        computer.execute_next();
        assert_eq!(computer.read_reg(Regs::AX).unwrap(), 0x3872);
        assert_eq!(computer.read_reg(Regs::DX).unwrap(), 0x00);
    }

    #[test]
    fn test_div() {
        let mut computer = new_cpu_vec(vec![0xba, 0xaa, 0x00, 0xb8, 0x55, 0x55, 0xbb, 0xff, 0x00, 0xf7, 0xfb]);
        computer.execute_next();
        computer.execute_next();
        computer.execute_next();
        computer.execute_next();
        assert_eq!(computer.read_reg(Regs::AX).unwrap(), 0xab00);
        assert_eq!(computer.read_reg(Regs::DX).unwrap(), 0x0055);
    }
}

#[cfg(test)]
mod stack_test {
    use crate::cpu::Regs;
    use crate::{new_cpu_from_file};

    #[test]
    fn test_push() {
        let mut computer = new_cpu_from_file("tests/objects/push.out");
        computer.execute_next();
        assert_eq!(computer.read_reg(Regs::AX).unwrap(), 0x05);
        computer.execute_next();
        assert_eq!(computer.read_reg(Regs::SP).unwrap(), 0xFFFD);
        assert_eq!(computer.get_mem_seg(Regs::SS, computer.read_reg(Regs::SP).unwrap() + 1), 0x05);
    }

    #[test]
    fn test_pop() {
        let mut computer = new_cpu_from_file("tests/objects/pop.out");
        computer.execute_next();
        computer.execute_next();
        computer.execute_next();
        assert_eq!(computer.read_reg(Regs::BX).unwrap(), 0x05);
    }
}
