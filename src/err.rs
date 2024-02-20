//! CPU和ISA错误

/// ISA errors
///
/// 当指令执行出错时，会返回一个`ISAErr`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ISAErr {
    TypeMismatch,
    StackUnderflow,
    InvalidReg,
    InvalidHeapType,
    InvalidStackAddr,
    DivByZero,
    InvalidSysCall,
    InvalidSysCallArg,
    SysCallErr,
    Halt,
}


