use ::serde::{Deserialize, Serialize};
use reg::{F64Reg, F64RegType, UsizeReg, UsizeRegType};

pub mod err;
pub mod reg;
#[allow(unused)]
pub mod serde;
/// 指令集
///
/// 约定指令的目标寄存器是第一个参数，源寄存器是后面的参数
/// 各指令后缀的U和D分别表示通用寄存器和浮点寄存器
/// 再有I后缀代表立即数操作
/// 例如：MovU(UsizeReg::U1, UsizeReg::U2) 表示将U2的值赋给U1
///
/// 例如：AddU(UsizeReg::U1, UsizeReg::U2, UsizeReg::U3) 表示将U2和U3的值相加，结果赋给U1
///
/// 例如：Not(UsizeReg::U1, UsizeReg::U2) 表示将U2的值取反，结果赋给U1
///
/// 对于Load，Store，In，Out等指令，第一个参数是目标寄存器，第二个参数是地址寄存器
///
/// 对于Jmp，Call等指令，第一个参数是地址寄存器,后面的参数是条件寄存器
///
/// 对于Push，Pop等指令，只有一个参数
///
/// 对于Halt，Ret等指令，没有参数
///
/// 对于SysCall，参数是内置函数的标号
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Inst {
    Nop,
    MU(UsizeReg, UsizeRegType),
    MD(F64Reg, F64RegType),
    MovU(UsizeReg, UsizeReg),
    MovD(F64Reg, F64Reg),
    Mod(UsizeReg, UsizeReg, UsizeReg),
    AddU(UsizeReg, UsizeReg, UsizeReg),
    AddUI(UsizeReg, UsizeRegType),
    AddD(F64Reg, F64Reg, F64Reg),
    AddDI(F64Reg, F64RegType),
    SubU(UsizeReg, UsizeReg, UsizeReg),
    SubUI(UsizeReg, UsizeRegType),
    SubD(F64Reg, F64Reg, F64Reg),
    SubDI(F64Reg, F64RegType),
    MulU(UsizeReg, UsizeReg, UsizeReg),
    MulD(F64Reg, F64Reg, F64Reg),
    DivU(UsizeReg, UsizeReg, UsizeReg),
    DivD(F64Reg, F64Reg, F64Reg),
    And(UsizeReg, UsizeReg, UsizeReg),
    Or(UsizeReg, UsizeReg, UsizeReg),
    Xor(UsizeReg, UsizeReg, UsizeReg),
    Not(UsizeReg, UsizeReg),
    NegU(UsizeReg, UsizeReg),
    NegD(F64Reg, F64Reg),
    Shl(UsizeReg, UsizeReg),
    Shr(UsizeReg, UsizeReg),
    LoadUH(UsizeReg, UsizeReg),
    LoadDH(F64Reg, UsizeReg),
    LoadUS(UsizeReg, UsizeReg),
    LoadDS(F64Reg, UsizeReg),
    StoreUH(UsizeReg, UsizeReg),
    StoreDH(F64Reg, UsizeReg),
    StoreUS(UsizeReg, UsizeReg),
    StoreDS(F64Reg, UsizeReg),
    Jo(UsizeReg),
    Jno(UsizeReg),
    Je(UsizeReg, UsizeReg, UsizeReg),
    Jne(UsizeReg, UsizeReg, UsizeReg),
    Jz(UsizeReg, UsizeReg),
    Jnz(UsizeReg, UsizeReg),
    Jmp(UsizeReg),
    PushU(UsizeReg),
    PushD(F64Reg),
    PopU(UsizeReg),
    PopD(F64Reg),
    Call(UsizeReg),
    SysCall(UsizeReg),
    InU(UsizeReg, UsizeReg),
    InD(F64Reg, UsizeReg),
    OutU(UsizeReg, UsizeReg),
    OutD(F64Reg, UsizeReg),
    Ret,
    Halt,
}

/// 寄存器类型
///
/// 用于表示寄存器的值
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RegType {
    Usize(UsizeRegType),
    F64(F64RegType),
}



pub trait VmRunner {
    type VmErr;
    fn run(&mut self,code: &[Inst])->Result<(),Self::VmErr>;
}