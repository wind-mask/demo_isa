//! 寄存器和标志
//! 函数调用时尽可能使用寄存器传递，调用约定暂未定。
use enumflags2::bitflags;
use serde::{Deserialize, Serialize};

/// 寄存器类型
///
///
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

/*
pub enum Reg{
    Usize(usize),
    F64(F64Reg),
}
*/
/// 通用寄存器类型
///
/// 通用寄存器
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UsizeReg {
    U1,
    U2,
    U3,
    U4,
    U5,
    U6,
    U7,
    U8,
}
/// 浮点寄存器类型
///
/// 64位浮点寄存器
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
//TODO:分离不同类型寄存器，从而提高指令效率
pub enum F64Reg {
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
}
/// 标志
///
/// 用于标志CPU的状态
#[bitflags]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Flags {
    Overflow,  // 溢出
    Interrupt, // 中断
}
