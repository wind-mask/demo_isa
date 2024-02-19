//! CPU和ISA错误

/// ISA errors
///
/// 当指令执行出错时，会返回一个`ISAErr`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ISAErr {
    TypeMismatch,
    StackUnderflow,
    InvalidReg,
    InvalidHeapAddr,
    InvalidStackAddr,
    DivByZero,
    InvalidSysCall,
    InvalidSysCallArg,
    SysCallErr,
    Halt,
}

/// CPU errors
///
/// 当CPU未能执行指令时，会返回一个`CpuErr`。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CpuErr {
    InvalidCodeAddr,
    ISAErr(ISAErr),
}

impl From<ISAErr> for CpuErr {
    fn from(e: ISAErr) -> CpuErr {
        CpuErr::ISAErr(e)
    }
}
