use crate::registers::Register;
use crate::{CoreState, registers};
use std::any::Any;
use std::ops::{BitAnd, BitOr, BitXor};

pub trait Instruction: Any {
    fn run(&self, state: &mut CoreState);
}

impl<F: Fn(&mut CoreState) + 'static + Clone> Instruction for F {
    fn run(&self, state: &mut CoreState) {
        self(state)
    }
}
macro_rules! op_reg_reg {
    ($name:ident, $op:expr) => {
        #[allow(unused)]
        pub fn $name<Rd, Rs1, Rs2>(_: Rd, _: Rs1, _: Rs2) -> impl Instruction
        where
            Rd: Register,
            Rs1: Register,
            Rs2: Register,
        {
            move |state: &mut CoreState| {
                let val = $op(Rs1::load(state), Rs2::load(state));
                Rd::store(state, val)
            }
        }
    };
}
macro_rules! op_reg_imm {
    ($name:ident, $op:expr) => {
        #[allow(unused)]
        pub fn $name<Rd, Rs1>(_: Rd, _: Rs1, imm: i32) -> impl Instruction
        where
            Rd: Register,
            Rs1: Register,
        {
            move |state: &mut CoreState| {
                let val = $op(Rs1::load(state), imm);
                Rd::store(state, val)
            }
        }
    };
}

op_reg_imm!(addi, i32::wrapping_add);
op_reg_reg!(add, i32::wrapping_add);
op_reg_reg!(sub, i32::wrapping_sub);

pub fn ebreak() -> impl Instruction {
    |state: &mut CoreState| {
        state.stopped = true;
    }
}

#[allow(unused)]
pub fn li<Rd>(rd: Rd, imm: i32) -> impl Instruction
where
    Rd: Register,
{
    addi(rd, registers::x0, imm)
}
#[allow(unused)]
pub fn mv<Rd, Rs1>(rd: Rd, rs1: Rs1) -> impl Instruction
where
    Rd: Register,
    Rs1: Register,
{
    addi(rd, rs1, 0)
}

op_reg_reg!(and, i32::bitand);
op_reg_reg!(or, i32::bitor);
op_reg_reg!(xor, i32::bitxor);
op_reg_imm!(andi, i32::bitand);
op_reg_imm!(ori, i32::bitor);
op_reg_imm!(xori, i32::bitxor);

op_reg_reg!(slt, |rs1, rs2| { if rs1 < rs2 { 1 } else { 0 } });
op_reg_reg!(sltu, |rs1, rs2| {
    if (rs1 as u32) < (rs2 as u32) { 1 } else { 0 }
});
op_reg_imm!(slti, |rs1, rs2| { if rs1 < rs2 { 1 } else { 0 } });
op_reg_imm!(sltiu, |rs1, rs2| {
    if (rs1 as u32) < (rs2 as u32) { 1 } else { 0 }
});

op_reg_reg!(sra, |a, b| a >> (b as u32 % 32));
op_reg_imm!(srai, |a, b| a >> b);
op_reg_reg!(srl, |a, b| ((a as u32) >> (b as u32 % 32)) as i32);
op_reg_imm!(srli, |a, b| ((a as u32) >> b) as i32);
op_reg_reg!(sll, |a, b| a << b);
op_reg_imm!(slli, |a, b| a << b);

const _: () = {
    // Verify Rust's shift rules match the arithmetic/logical rules we want
    let x = -1;
    let y = ((x as u32) >> 2) as i32;
    let z = (x >> 2);
    assert!(y == 0b00111111_11111111_11111111_11111111);
    assert!(z == -1);
};

/// Load upper immediate.
/// Often combined with `hi`, `lo` and `addi`
pub fn lui<Rd>(rd: Rd, imm: i32) -> impl Instruction
where
    Rd: Register,
{
    assert!((0..=1048575).contains(&imm));
    addi(rd, registers::x0, imm << 12)
}

pub struct Label {
    pub name: &'static str,
}
impl Instruction for Label {
    fn run(&self, _: &mut CoreState) {
        /* No-op */
    }
}
pub fn label(name: &'static str) -> impl Instruction {
    Label { name }
}

macro_rules! op_jmp_reg_reg {
    ($name:ident, $op:expr) => {
        #[allow(unused)]
        pub fn $name<Rs1, Rs2>(_: Rs1, _: Rs2, dst: &'static str) -> impl Instruction
        where
            Rs1: Register,
            Rs2: Register,
        {
            move |state: &mut CoreState| {
                if $op(&Rs1::load(state), &Rs2::load(state)) {
                    state.jump(dst);
                }
            }
        }
    };
}

op_jmp_reg_reg!(beq, i32::eq);
op_jmp_reg_reg!(bne, i32::ne);
op_jmp_reg_reg!(blt, i32::lt);
op_jmp_reg_reg!(bge, i32::ge);
op_jmp_reg_reg!(bltu, |a: &i32, b: &i32| u32::lt(&(*a as u32), &(*b as u32)));
op_jmp_reg_reg!(bgeu, |a: &i32, b: &i32| u32::ge(&(*a as u32), &(*b as u32)));

fn jal<Rd>(_: Rd, label: &'static str) -> impl Instruction
where
    Rd: Register,
{
    move |state: &mut CoreState| {
        let next = state.pc;
        Rd::store(state, next + 1);
        state.jump(label);
    }
}
fn jalr<Rd>(_: Rd, imm: i32, label: &'static str) -> impl Instruction
where
    Rd: Register,
{
    move |state: &mut CoreState| {
        let next = state.pc;
        Rd::store(state, next + 1);
        state.jump(label);
    }
}

fn j(label: &'static str) -> impl Instruction {
    jal(registers::x0, label)
}
