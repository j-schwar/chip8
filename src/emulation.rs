use crate::data::Register;

/// Size of emulator RAM in number of bytes.
const MEMORY_SIZE: usize = 4096;

/// Size of the stack in number of addresses (u16).
const STACK_SIZE: usize = 16;

#[derive(Debug)]
pub enum EmulationError {
    StackOverflow,
    StackUnderflow,
    OutOfMemory,
    InvalidInstruction(u16),
}

/// [Memory] is a 4KiB array of bytes used as RAM for the Chip-8 emulator.
struct Memory([u8; MEMORY_SIZE]);

impl Memory {
    /// Loads a chunk of data into memory at a given offset. Returns an out-of-memory
    /// error if the given data chunk is too large.
    fn load(&mut self, offset: usize, data: &[u8]) -> Result<(), EmulationError> {
        if data.len() > MEMORY_SIZE - offset {
            return Err(EmulationError::OutOfMemory);
        }

        for i in 0..data.len() {
            self.0[i + offset] = data[i];
        }

        Ok(())
    }

    /// Fetches a 2-byte instruction at a given address.
    fn fetch_instruction(&self, address: u16) -> &[u8] {
        assert_eq!(address % 2, 0, "instructions must be 16-bit aligned");
        assert!((address as usize) < MEMORY_SIZE - 2);

        let index = address as usize;
        &self.0[index..index + 2]
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory([0; MEMORY_SIZE])
    }
}

/// [Stack] is the program stack for the Chip-8 emulator. The stack is used to store
/// return addresses for subroutine calls. 16 such addresses can be stored on the stack.
/// Chip-8 only allows for up to 16 levels of nested subroutine calls.
#[derive(Default)]
struct Stack {
    stack_index: usize,
    memory: [u16; STACK_SIZE],
}

impl Stack {
    /// Pushes an address onto the stack. Returns a stack overflow error if the stack is
    /// full and no more addresses can be pushed.
    fn push(&mut self, addr: u16) -> Result<(), EmulationError> {
        if self.stack_index >= STACK_SIZE - 1 {
            return Err(EmulationError::StackOverflow);
        }

        self.stack_index += 1;
        self.memory[self.stack_index] = addr;
        Ok(())
    }

    /// Pops an address off of the top of the stack. Returns a stack underflow error if
    /// the stack is empty and not addresses can be popped off.
    fn pop(&mut self) -> Result<u16, EmulationError> {
        if self.stack_index == 0 {
            return Err(EmulationError::StackUnderflow);
        }

        let addr = self.memory[self.stack_index];
        self.stack_index -= 1;
        Ok(addr)
    }
}

/// [Registers] is a collection of 16 general purpose registers.
#[derive(Default)]
struct Registers([u8; 16]);

impl Registers {
    /// Retrieves the value stored in a specific register.
    #[inline]
    fn get(&self, r: Register) -> u8 {
        self.0[r.0.as_usize()]
    }

    /// Sets the value of a specific register.
    #[inline]
    fn set(&mut self, r: Register, value: u8) {
        self.0[r.0.as_usize()] = value;
    }
}

#[derive(Default)]
struct EmulatorState {
    registers: Registers,
    address_register: u16, // aka. I
    program_counter: u16,  // aka. PC
    delay_register: u8,
    sound_register: u8,
    stack: Stack,
    memory: Memory,
}

pub struct Emulator {
    start_address: u16,
    state: EmulatorState,
}

impl Emulator {
    /// Constructs a new emulator with default options.
    pub fn new() -> Self {
        Emulator {
            start_address: 0x200,
            state: EmulatorState::default(),
        }
    }

    /// Executes a program written in Chip-8 machine code.
    pub fn run(&mut self, program: &[u8]) -> Result<(), EmulationError> {
        self.state = Default::default();
        self.state
            .memory
            .load(self.start_address as usize, program)?;

        self.state.program_counter = self.start_address;
        self.emulation_loop()?;
        Ok(())
    }

    fn emulation_loop(&mut self) -> Result<(), EmulationError> {
        loop {}
    }
}

impl Default for Emulator {
    /// Constructs a new emulator with default options.
    fn default() -> Self {
        Self::new()
    }
}
