use crate::reg::Flags;
use ::serde::{Deserialize, Serialize};
use enumflags2::BitFlags;
use err::{CpuErr, ISAErr};
use reg::Reg;

pub mod err;
pub mod reg;
#[allow(unused)]
pub mod serde;
/// 指令集
///
/// 约定指令的目标寄存器是第一个参数，源寄存器是后面的参数
///
/// 例如：Mov(Reg::R1, Reg::R2) 表示将R2的值赋给R1
///
/// 例如：AddU(Reg::R1, Reg::R2, Reg::R3) 表示将R2和R3的值相加，结果赋给R1
///
/// 例如：Not(Reg::R1, Reg::R2) 表示将R2的值取反，结果赋给R1
///
/// 对于Load，Store，In，Out等指令，第一个参数是目标寄存器，第二个参数是地址寄存器
///
/// 对于Jmp，Call等指令，第一个参数是地址寄存器,后面的参数是条件寄存器
///
/// 对于Push，Pop等指令，只有一个参数
///
/// 对于Halt，Ret等指令，没有参数
///
/// 对于CallIF，第一个参数是内置函数的标号
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
    SysCall(Reg),
    In(Reg, Reg),
    Out(Reg, Reg),
    Ret,
    Halt,
}

/// 寄存器类型
///
/// 用于表示寄存器的值
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RegType {
    Usize(usize),
    F64(f64),
}

/// 用于表示内存地址
pub type CodeAddr = usize;
/// 用于表示栈地址
pub type StackAddr = usize;

/// 堆上对象应实现的trait
///
/// 暂时用于将堆上对象转换为寄存器类型
pub trait HeapObjRuner {
    fn get_reg_type(&self) -> RegType;
    fn set_reg_type(&mut self, val: RegType);
}

/// 内存应实现的trait
///
/// 用于模拟内存的操作
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

/// ISA执行器应实现的trait
///
/// 实现了该trait的结构体可以执行指令集
pub trait ISARuner {
    type M: MemoryRuner;
    fn run_inst(&mut self, inst: Inst, mem: &mut Self::M) -> Result<(), ISAErr>;
    fn get_reg(&self, reg: Reg) -> RegType;
    fn set_reg(&mut self, reg: Reg, val: RegType);
    fn get_pc(&self) -> CodeAddr;
    fn set_pc(&mut self, pc: CodeAddr);
    fn get_bp(&self) -> StackAddr;
    fn set_bp(&mut self, bp: StackAddr);
    fn get_flags(&self) -> BitFlags<Flags>;
    fn set_flags(&mut self, flags: BitFlags<Flags>);
}
