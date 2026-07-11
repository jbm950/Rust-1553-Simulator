use super::WORD_SIZE;

/// Trait for MIL-STD-1553 Command and Status Words
pub trait ProtocolWord: Copy {
    fn to_be_bytes(self) -> [u8; WORD_SIZE];
}
