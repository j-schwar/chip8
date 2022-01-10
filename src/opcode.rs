use std::fmt::{self, Display};

use crate::data::{Addr, Nibble, Register};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opcode {
    // http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.1

    // nnn - 12-bit memory address (Addr)
    // n - 4-bit value: lowest 4 bits of the instruction (Nibble)
    // x - 4-bit value: lower 4 bits of the high byte of the instruction (Nibble)
    // y - 4-bit value: high 4 bits of the low byte of the instruction (Nibble)
    Sys(Addr),                       // 0nnn - Jump to machine code routine at nnn (unused)
    Cls,                             // 00E0 - Clear the display
    Ret,                             // 00EE - Return from subroutine
    Jp(Addr),                        // 1nnn - Jump to address
    Call(Addr),                      // 2nnn - Call subroutine at nnn
    Se(Register, u8),                // 3xkk - Skip next instr. if Vx equals kk
    Sne(Register, u8),               // 4xkk - Skip next instr. if Vx not equals kk
    Sev(Register, Register),         // 5xy0 - Skip next instr. if Vx equals Vy
    LdImm(Register, u8),             // 6xkk - Set Vx to kk
    AddImm(Register, u8),            // 7xkk - Set Vx to Vx + kk
    Ld(Register, Register),          // 8xy0 - Set Vx to Vy
    Or(Register, Register),          // 8xy1 - Set Vx to Vx | Vy
    And(Register, Register),         // 8xy2 - Set Vx to Vx & Vy
    Xor(Register, Register),         // 8xy3 - Set Vx to Vx ^ Vy
    Add(Register, Register),         // 8xy4 - Set Vx to Vx + Vy and set VF = carry
    Sub(Register, Register),         // 8xy5 - Set Vx to Vx - Vy and set VF = NOT borrow
    Shr(Register),                   // 8xy6 - Set Vx to Vx >> 1 and set VF = Vx & 0x01
    Subn(Register, Register),        // 8xy7 - Set Vx to Vy - Vx and set FV = NOT borrow
    Shl(Register),                   // 8xyE - Set Vx to Vx << 1 and set VF = Vx & 0x80
    Snev(Register, Register),        // 9xy0 - Skip next instr. if Vx not equals Vy
    Ldi(Addr),                       // Annn - Set I to nnn
    JpV0(Addr),                      // Bnnn - Jump to V0 + nnn
    Rnd(Register, u8),               // Cxkk - Set Vx to RANDOM BYTE & kk
    Drw(Register, Register, Nibble), // Dxyn - Display n-byte sprite at address I at position (Vx, Vy) set VF = collision
    Skp(Register),                   // Ex9E - Skip next instr. if key with value Vx is pressed
    Sknp(Register),                  // ExA1 - Skip next instr. if key with value Vx is not pressed
    LdVDt(Register),                 // Fx07 - Load the delay timer value into Vx
    LdK(Register),                   // Fx0A - Wait for a key press, store the value in Vx
    LdDtV(Register),                 // Fx15 - Set the delay timer to Vx
    LdStV(Register),                 // Fx18 - Set the sound timer to Vx
    AddI(Register),                  // Fx1E - Set I to I + Vx
    LdF(Register),                   // Fx29 - Set I to the location of the sprite for digit Vx
    LdB(Register),                   // Fx33 - Store the BCD rep. of Vx in locations I, I+1, and I+2
    Dump(Register),                  // Fx55 - Store V0 to Vx in memory starting at loc. I
    Restore(Register),               // Fx65 - Read V0 to Vx from memory starting at loc. I
}

impl Opcode {
    /// Decodes a pair of bytes into an [Opcode]. Returns [None] if the given bytes do
    /// not correspond to a valid opcode.
    ///
    /// # Panics
    ///
    /// This function will panic if `bytes.len()` is not equal to 2.
    pub fn decode(bytes: &[u8]) -> Option<Self> {
        assert_eq!(bytes.len(), 2);

        let n0 = Nibble::from_high(bytes[0]);
        match n0.as_u8() {
            0x0 => match (bytes[0], bytes[1]) {
                (0x00, 0xE0) => Some(Opcode::Cls),
                (0x00, 0xEE) => Some(Opcode::Ret),
                (high, low) => {
                    let addr = addr_from_bytes(high, low);
                    Some(Opcode::Sys(addr))
                }
            },

            0x1 => {
                let addr = addr_from_bytes(bytes[0], bytes[1]);
                Some(Opcode::Jp(addr))
            }

            0x2 => {
                let addr = addr_from_bytes(bytes[0], bytes[1]);
                Some(Opcode::Call(addr))
            }

            0x3 => {
                let r = Register(Nibble::from_low(bytes[0]));
                Some(Opcode::Se(r, bytes[1]))
            }

            0x4 => {
                let r = Nibble::from_low(bytes[0]);
                Some(Opcode::Sne(Register(r), bytes[1]))
            }

            0x5 => {
                if Nibble::from_low(bytes[1]).as_u8() == 0 {
                    let r1 = Register(Nibble::from_low(bytes[0]));
                    let r2 = Register(Nibble::from_high(bytes[1]));
                    Some(Opcode::Sev(r1, r2))
                } else {
                    None
                }
            }

            0x6 => {
                let r = Nibble::from_low(bytes[0]);
                Some(Opcode::LdImm(Register(r), bytes[1]))
            }

            0x7 => {
                let r = Nibble::from_low(bytes[0]);
                Some(Opcode::AddImm(Register(r), bytes[1]))
            }

            0x8 => {
                let r1 = Register(Nibble::from_low(bytes[0]));
                let r2 = Register(Nibble::from_high(bytes[1]));
                match Nibble::from_low(bytes[1]).as_u8() {
                    0x0 => Some(Opcode::Ld(r1, r2)),
                    0x1 => Some(Opcode::Or(r1, r2)),
                    0x2 => Some(Opcode::And(r1, r2)),
                    0x3 => Some(Opcode::Xor(r1, r2)),
                    0x4 => Some(Opcode::Add(r1, r2)),
                    0x5 => Some(Opcode::Sub(r1, r2)),
                    0x6 => Some(Opcode::Shr(r1)),
                    0x7 => Some(Opcode::Subn(r1, r2)),
                    0xE => Some(Opcode::Shl(r1)),
                    _ => None,
                }
            }

            0x9 => {
                if Nibble::from_low(bytes[1]).as_u8() == 0 {
                    let r1 = Register(Nibble::from_low(bytes[0]));
                    let r2 = Register(Nibble::from_high(bytes[1]));
                    Some(Opcode::Snev(r1, r2))
                } else {
                    None
                }
            }

            0xA => {
                let addr = addr_from_bytes(bytes[0], bytes[1]);
                Some(Opcode::Ldi(addr))
            }

            0xB => {
                let addr = addr_from_bytes(bytes[0], bytes[1]);
                Some(Opcode::JpV0(addr))
            }

            0xC => {
                let r = Register(Nibble::from_low(bytes[0]));
                Some(Opcode::Rnd(r, bytes[1]))
            }

            0xD => {
                let r1 = Register(Nibble::from_low(bytes[0]));
                let r2 = Register(Nibble::from_high(bytes[1]));
                let n = Nibble::from_low(bytes[1]);
                Some(Opcode::Drw(r1, r2, n))
            }

            0xE => {
                let r = Register(Nibble::from_low(bytes[0]));
                match bytes[1] {
                    0x9E => Some(Opcode::Skp(r)),
                    0xA1 => Some(Opcode::Sknp(r)),
                    _ => None,
                }
            }

            0xF => {
                let r = Register(Nibble::from_low(bytes[0]));
                match bytes[1] {
                    0x07 => Some(Opcode::LdVDt(r)),
                    0x0A => Some(Opcode::LdK(r)),
                    0x15 => Some(Opcode::LdDtV(r)),
                    0x18 => Some(Opcode::LdStV(r)),
                    0x1E => Some(Opcode::AddI(r)),
                    0x29 => Some(Opcode::LdF(r)),
                    0x33 => Some(Opcode::LdB(r)),
                    0x55 => Some(Opcode::Dump(r)),
                    0x65 => Some(Opcode::Restore(r)),
                    _ => None,
                }
            }

            _ => None,
        }
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Opcode::*;

        match self {
            Sys(addr) => write!(f, "SYS  0x{:03X}", addr),
            Cls => write!(f, "CLS"),
            Ret => write!(f, "RET"),
            Jp(addr) => write!(f, "JP   0x{:03X}", addr),
            Call(addr) => write!(f, "CALL 0x{:03X}", addr),
            Se(r, x) => write!(f, "SE   V{}, 0x{:02X}", r.0, x),
            Sne(r, x) => write!(f, "SNE  V{}, 0x{:02X}", r.0, x),
            Sev(r1, r2) => write!(f, "SE   V{}, V{}", r1.0, r2.0),
            LdImm(r, x) => write!(f, "LD   V{}, 0x{:02X}", r.0, x),
            AddImm(r, x) => write!(f, "ADD  V{}, 0x{:02X}", r.0, x),
            Ld(r1, r2) => write!(f, "LD   V{}, V{}", r1.0, r2.0),
            Or(r1, r2) => write!(f, "OR   V{}, V{}", r1.0, r2.0),
            And(r1, r2) => write!(f, "AND  V{}, V{}", r1.0, r2.0),
            Xor(r1, r2) => write!(f, "XOR  V{}, V{}", r1.0, r2.0),
            Add(r1, r2) => write!(f, "ADD  V{}, V{}", r1.0, r2.0),
            Sub(r1, r2) => write!(f, "SUB  V{}, V{}", r1.0, r2.0),
            Shr(r1) => write!(f, "SHR  V{}", r1.0),
            Subn(r1, r2) => write!(f, "SUBN V{}, V{}", r1.0, r2.0),
            Shl(r1) => write!(f, "SHL   V{}", r1.0),
            Snev(r1, r2) => write!(f, "SNE  V{}, V{}", r1.0, r2.0),
            Ldi(addr) => write!(f, "LD   I, 0x{:03X}", addr),
            JpV0(addr) => write!(f, "JP   V0, 0x{:03X}", addr),
            Rnd(r, x) => write!(f, "RND  V{}, 0x{:02X}", r.0, x),
            Drw(r1, r2, n) => write!(f, "DRW  V{}, V{}, 0x{:X}", r1.0, r2.0, n.as_u8()),
            Skp(r) => write!(f, "SKP  V{}", r.0),
            Sknp(r) => write!(f, "SKNP V{}", r.0),
            LdVDt(r) => write!(f, "LD   V{}, DT", r.0),
            LdK(r) => write!(f, "LD   V{}, K", r.0),
            LdDtV(r) => write!(f, "LD   DT, V{}", r.0),
            LdStV(r) => write!(f, "LD   ST, V{}", r.0),
            AddI(r) => write!(f, "ADD  I, V{}", r.0),
            LdF(r) => write!(f, "LD   F, V{}", r.0),
            LdB(r) => write!(f, "LD   B, V{}", r.0),
            Dump(r) => write!(f, "LD   [I], V{}", r.0),
            Restore(r) => write!(f, "LD   V{}, [I]", r.0),
        }
    }
}

fn addr_from_bytes(high: u8, low: u8) -> Addr {
    u16::from_be_bytes([high & 0x0F, low])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decode_sys() {
        let opcode = Opcode::decode(&[0x01, 0x23]);
        assert_eq!(opcode, Some(Opcode::Sys(0x0123)));
    }

    #[test]
    fn decode_cls() {
        let opcode = Opcode::decode(&[0x00, 0xE0]);
        assert_eq!(opcode, Some(Opcode::Cls));
    }

    #[test]
    fn decode_ret() {
        let opcode = Opcode::decode(&[0x00, 0xEE]);
        assert_eq!(opcode, Some(Opcode::Ret));
    }
}
