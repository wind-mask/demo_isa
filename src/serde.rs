//! 序列化指令集
use crate::Inst;

/// 序列化指令
///
/// 暂使用postcard序列化指令集
pub fn serde_code(code: &Vec<Inst>) -> Result<Vec<u8>, postcard::Error> {
    postcard::to_allocvec(&code)
}
/// 反序列化指令
///
/// 暂使用postcard反序列化指令集
pub fn de_serde_code(code: &[u8]) -> Result<Vec<Inst>, postcard::Error> {
    postcard::from_bytes(code)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::reg::Reg;
    use crate::Inst::{Call, Nop};
    use crate::{Inst, RegType};

    #[test]
    fn test_serde_code() {
        let code = vec![
            Nop,
            Inst::M(Reg::R2, RegType::Usize(10)),
            Inst::M(Reg::R1, RegType::Usize(1)),
            Call(Reg::R2),
            Call(Reg::R2),
            Inst::M(Reg::R4, RegType::Usize(2)),
            Inst::AddU(Reg::R3, Reg::R2, Reg::R4),
            Inst::Jmp(Reg::R3),
            Nop,
            Nop,
            Inst::AddU(Reg::R1, Reg::R1, Reg::R1),
            Inst::Ret,
            Inst::Halt,
        ];
        let out = serde_code(&code).unwrap();
    }
    #[test]
    fn test_de_serde_code() {
        let code = vec![
            Nop,
            Inst::M(Reg::R2, RegType::Usize(10)),
            Inst::M(Reg::R1, RegType::Usize(1)),
            Call(Reg::R2),
            Call(Reg::R2),
            Inst::M(Reg::R4, RegType::Usize(2)),
            Inst::AddU(Reg::R3, Reg::R2, Reg::R4),
            Inst::Jmp(Reg::R3),
            Nop,
            Nop,
            Inst::AddU(Reg::R1, Reg::R1, Reg::R1),
            Inst::Ret,
            Inst::Halt,
        ];
        let out = serde_code(&code).unwrap();
        let out2 = de_serde_code(&out).unwrap();
        assert_eq!(code, out2);
    }
}
