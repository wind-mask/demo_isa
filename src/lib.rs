use crate::reg::Flags;
use ::serde::{Deserialize, Serialize};
use enumflags2::BitFlags;
use err::{CpuErr, ISAErr};
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

/// 用于表示内存地址
pub type CodeAddr = UsizeRegType;
/// 用于表示栈地址
pub type StackAddr = UsizeRegType;

/// 堆上对象应实现的trait
///
/// 暂时用于将堆上对象转换为寄存器类型
pub trait HeapObjRuner {
    fn get_reg_type(&self) -> Result<RegType, ISAErr>;
    fn get_u8_vec(&self) -> &[u8];
}

/// 内存应实现的trait
///
/// 用于模拟内存的操作
pub trait MemoryRuner {
    fn clear_heap(&mut self);
    fn clear_stack(&mut self);
    fn clear_code(&mut self);
    fn get_heap_u_type(&mut self, addr: UsizeRegType) -> Result<UsizeRegType, ISAErr>;
    fn get_heap_f_type(&mut self, addr: UsizeRegType) -> Result<F64RegType, ISAErr>;
    fn set_heap(&mut self, addr: UsizeRegType, val: RegType);
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

/// ISA执行器应实现的trait
///
/// 实现了该trait的结构体可以执行指令集
pub trait ISARuner {
    type M: MemoryRuner;
    fn run_inst(&mut self, inst: Inst, mem: &mut Self::M) -> Result<(), ISAErr>;
    fn get_u_reg(&self, reg: UsizeReg) -> UsizeRegType;
    fn get_f_reg(&self, reg: F64Reg) -> F64RegType;
    fn get_mut_u_reg(&mut self, reg: UsizeReg) -> &mut UsizeRegType;
    fn get_mut_f_reg(&mut self, reg: F64Reg) -> &mut F64RegType;
    fn set_u_reg(&mut self, reg: UsizeReg, val: UsizeRegType);
    fn set_f_reg(&mut self, reg: F64Reg, val: F64RegType);
    fn get_pc(&self) -> CodeAddr;
    fn set_pc(&mut self, pc: CodeAddr);
    fn get_bp(&self) -> StackAddr;
    fn set_bp(&mut self, bp: StackAddr);
    fn get_flags(&self) -> BitFlags<Flags>;
    fn set_flags(&mut self, flags: BitFlags<Flags>);
}
