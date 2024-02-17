#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ISAErr {
    TypeMismatch,
    InvalidReg,
    InvalidHeapAddr,
    InvalidStackAddr,
    DivByZero,
    Halt,
}

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
