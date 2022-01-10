use std::io;

use crate::opcode::Opcode;

const DEFAULT_START_ADDR: u16 = 0x200;

/// [Disassembler] provides facilities for disassembling Chip8 machine code into assembly
/// instructions.
pub struct Disassembler {
    include_addresses: bool,
    start_address: u16,
    include_binary: bool,
}

impl Disassembler {
    /// Constructs a default disassembler.
    pub fn new() -> Self {
        Disassembler {
            include_addresses: false,
            start_address: DEFAULT_START_ADDR,
            include_binary: false,
        }
    }

    /// Enables/disables address printing for this disassembler.
    pub fn with_addresses(self, include_addresses: bool) -> Self {
        Disassembler {
            include_addresses,
            start_address: self.start_address,
            include_binary: self.include_binary,
        }
    }

    /// Sets the address to start at when printing addresses.
    pub fn with_start_address(self, start_address: u16) -> Self {
        Disassembler {
            include_addresses: self.include_addresses,
            start_address,
            include_binary: self.include_binary,
        }
    }

    /// Enables/disables printing binary along with instructions.
    pub fn with_binary(self, include_binary: bool) -> Self {
        Disassembler {
            include_addresses: self.include_addresses,
            start_address: self.start_address,
            include_binary,
        }
    }

    /// Disassembles a given program writing assembly instructions to a given writer.
    ///
    /// # Panics
    ///
    /// This method panics if the length of `program` is not even.
    pub fn disassemble<W: io::Write>(&self, program: &[u8], w: &mut W) -> io::Result<()> {
        if program.len() % 2 != 0 {
            panic!("program length must be equal");
        }

        for i in (0..program.len() - 1).step_by(2) {
            let opcode_bytes = &program[i..i + 2];
            let opcode = Opcode::decode(opcode_bytes);
            self.write_instruction(&opcode, i, opcode_bytes, w)?;
        }

        Ok(())
    }

    fn write_instruction<W: io::Write>(
        &self,
        opcode: &Option<Opcode>,
        index: usize,
        bytes: &[u8],
        w: &mut W,
    ) -> io::Result<()> {
        debug_assert!(bytes.len() == 2);

        let opcode_text = match opcode {
            Some(opcode) => format!("{}", opcode),
            None => String::from("--"),
        };

        let addr = index as u16 + self.start_address;

        match (self.include_addresses, self.include_binary) {
            (true, true) => {
                writeln!(
                    w,
                    "{:03X}   {:02X} {:02X}    {}",
                    addr, bytes[0], bytes[1], opcode_text
                )?;
            }

            (true, false) => {
                writeln!(w, "{:03X}    {}", addr, opcode_text)?;
            }

            (false, true) => {
                writeln!(w, "{:02X} {:02X}    {}", bytes[0], bytes[1], opcode_text)?;
            }

            (false, false) => {
                writeln!(w, "{}", opcode_text)?;
            }
        }

        Ok(())
    }
}

impl Default for Disassembler {
    /// Constructs a default disassembler.
    fn default() -> Self {
        Self::new()
    }
}
