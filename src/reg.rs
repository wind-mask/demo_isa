use enumflags2::bitflags;
use serde::{Deserialize, Serialize};

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

#[bitflags]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Flags {
    Overflow,
}
