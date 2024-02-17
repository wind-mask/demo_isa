use crate::reg::Flags;
use ::serde::{Deserialize, Serialize};
use enumflags2::BitFlags;
use err::{CpuErr, ISAErr};
use reg::Reg;

pub mod err;
pub mod reg;
#[allow(unused)]
pub mod serde;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Inst {
    Nop,
    M(Reg, RegType),
    Mov(Reg, Reg),
    Mod(Reg, Reg, Reg),
    AddU(Reg, Reg, Reg),
    AddD(Reg, Reg, Reg),
    SubU(Reg, Reg, Reg),
    SubD(Reg, Reg, Reg),
    MulU(Reg, Reg, Reg),
    MulD(Reg, Reg, Reg),
    DivU(Reg, Reg, Reg),
    DivD(Reg, Reg, Reg),
    And(Reg, Reg, Reg),
    Or(Reg, Reg, Reg),
    Xor(Reg, Reg, Reg),
    Not(Reg, Reg),
    Neg(Reg, Reg),
    Shl(Reg, Reg),
    Shr(Reg, Reg),
    LoadH(Reg, Reg),
    LoadS(Reg, Reg),
    StoreS(Reg, Reg),
    StoreH(Reg, Reg),
    Jo(Reg),
    Jno(Reg),
    Je(Reg, Reg, Reg),
    Jne(Reg, Reg, Reg),
    Jz(Reg, Reg),
    Jnz(Reg, Reg),
    Jmp(Reg),
    Push(Reg),
    Pop(Reg),
    Call(Reg),
    Ret,
    Halt,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RegType {
    Usize(usize),
    F64(f64),
}

pub type CodeAddr = usize;
pub type StackAddr = usize;

pub trait HeapObjRuner {
    fn get_reg_type(&self) -> RegType;
    fn set_reg_type(&mut self, val: RegType);
}

pub trait MemoryRuner {
    fn clear_heap(&mut self);
    fn clear_stack(&mut self);
    fn clear_code(&mut self);
    fn get_heap(&mut self, addr: usize) -> RegType;
    fn set_heap(&mut self, addr: usize, val: RegType);
    fn get_stack(&self, bp: StackAddr, addr: StackAddr) -> Result<RegType, ISAErr>;
    fn set_stack(&mut self, bp: StackAddr, addr: StackAddr, val: RegType) -> Result<(), ISAErr>;
    fn push_stack(&mut self, val: RegType);
    fn pop_stack(&mut self) -> Result<RegType, ISAErr>;
    fn get_stack_top_addr(&self) -> StackAddr;
    fn drop_stack_bp(&mut self, bp: StackAddr);
    fn fetch_code(&self, addr: CodeAddr) -> Result<Inst, CpuErr>;
    fn push_code_vec(&mut self, code: Vec<Inst>);
    fn push_stack_vec(&mut self, stack: Vec<RegType>);
}

pub trait ISARuner {
    fn run_inst(&mut self, inst: Inst, mem: &mut impl MemoryRuner) -> Result<(), ISAErr>;
    fn get_reg(&self, reg: Reg) -> RegType;
    fn set_reg(&mut self, reg: Reg, val: RegType);
    fn get_pc(&self) -> CodeAddr;
    fn set_pc(&mut self, pc: CodeAddr);
    fn get_bp(&self) -> StackAddr;
    fn set_bp(&mut self, bp: StackAddr);
    fn get_flags(&self) -> BitFlags<Flags>;
    fn set_flags(&mut self, flags: BitFlags<Flags>);
}
