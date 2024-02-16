use serde::{Deserialize, Serialize};

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ISAErr {
    TypeMismatch,
    InvalidReg,
    InvalidHeapAddr,
    InvalidStackAddr,
    DivByZero,
    Halt,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RegType {
    Usize(usize),
    F64(f64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Reg {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    R16,
}
