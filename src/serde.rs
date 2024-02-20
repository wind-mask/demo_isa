//! 序列化指令集
use crate::Inst;

/// 序列化指令
///
/// 暂使用postcard序列化指令集
pub fn serde_code(code: &Vec<Inst>) -> Result<Vec<u8>, postcard::Error> {
    postcard::to_allocvec(&code)
}
#[cfg(test)]
#[test]
fn test_serde_code() {
    use crate::reg::F64Reg::*;
    use crate::reg::UsizeReg::*;
    use log::debug;
    use Inst::*;
    env_logger::init();
    const NUM_INST: usize = 10000;
    let mut code = vec![Inst::Nop; NUM_INST];
    let mut len = serde_code(&code).unwrap().len();
    let codeenum = [
        Nop,
        MU(U1, 0),
        MD(F1, 0.0),
        MovU(U1, U2),
        MovD(F1, F2),
        Mod(U1, U2, U3),
        AddU(U1, U2, U3),
        AddUI(U1, 0),
        AddD(F1, F2, F3),
        AddDI(F1, 0.0),
        SubU(U1, U2, U3),
        SubUI(U1, 0),
        SubD(F1, F2, F3),
        SubDI(F1, 0.0),
        MulU(U1, U2, U3),
        MulD(F1, F2, F3),
        DivU(U1, U2, U3),
        DivD(F1, F2, F3),
        And(U1, U2, U3),
        Or(U1, U2, U3),
        Xor(U1, U2, U3),
        Not(U1, U2),
        NegU(U1, U2),
        NegD(F1, F2),
        Shl(U1, U2),
        Shr(U1, U2),
        LoadUH(U1, U2),
        LoadDH(F1, U2),
        LoadUS(U1, U2),
        LoadDS(F1, U2),
        StoreUH(U1, U2),
        StoreDH(F1, U2),
        StoreUS(U1, U2),
        StoreDS(F1, U2),
        InU(U1, U2),
        InD(F1, U1),
        OutD(F1, U1),
        OutU(U1, U2),
        Je(U1, U2, U3),
        Jmp(U1),
        Jne(U1, U2, U3),
        Jno(U1),
        Jnz(U1, U2),
        Jnz(U1, U2),
        Jo(U1),
        Jz(U1, U2),
        PushU(U1),
        PushD(F1),
        PopU(U1),
        PopD(F1),
        Jmp(U1),
        Jz(U1, U2),
        Jnz(U1, U2),
        Call(U1),
        Call(U1),
        SysCall(U1),
        Ret,
        Halt,
    ];
    for i in 0..codeenum.len() {
        code = vec![codeenum[i]; NUM_INST];
        len = serde_code(&code).unwrap().len();
        debug!("{:?}len:{}", codeenum[i], len / NUM_INST);
    }
}
/// 反序列化指令
///
/// 暂使用postcard反序列化指令集
pub fn de_serde_code(code: &[u8]) -> Result<Vec<Inst>, postcard::Error> {
    postcard::from_bytes(code)
}