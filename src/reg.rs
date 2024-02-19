//! 寄存器和标志
//! 函数调用时尽可能使用寄存器传递，调用约定暂未定。
use enumflags2::bitflags;
use serde::{Deserialize, Serialize};

pub type UsizeRegType = usize;
pub type F64RegType = f64;
/// 寄存器类型
///
/// 通用寄存器和浮点寄存器
pub enum Reg {
    Usize(UsizeReg),
    F64(F64Reg),
}
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
