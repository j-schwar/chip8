use std::fmt::{self, Debug, Display, Formatter};

/// 4-bit unsigned integer.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Nibble(u8);

impl Nibble {
    // /// Constructs a [Nibble] from a raw value.
    // ///
    // /// # Panics
    // ///
    // /// This function panics if `value` is larger that the largest 4-bit value (`0x0F`).
    // pub fn new(value: u8) -> Self {
    //     assert!(value <= 0xF);
    //     Nibble(value)
    // }

    /// Constructs a [Nibble] from the high 4 bits in a [u8].
    pub fn from_high(value: u8) -> Self {
        Nibble((value >> 4) & 0x0F)
    }

    /// Constructs a [Nibble] from the low 4 bits in a [u8].
    pub fn from_low(value: u8) -> Self {
        Nibble(value & 0x0F)
    }

    /// Converts this 4-bit value into an [u8].
    #[inline]
    pub fn as_u8(&self) -> u8 {
        self.0
    }

    /// Converts this 4-bit value into a [usize].
    #[inline]
    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }
}

impl Debug for Nibble {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl Display for Nibble {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Register(pub Nibble);

/// 12-bit unsigned integer representing a memory address.
pub type Addr = u16;
