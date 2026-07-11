//! Types representing MIL-STD-1553 protocol words and messages
//!
//! This module is transport-agnostic and is responsible only for
//! serialization and deserialization of protocol data structures.

mod cmd_word;
mod data_word;
mod message;
mod protocol_word;
mod status_word;
mod support;
mod transaction;

pub use cmd_word::{CmdWord, Subaddress, TxRx};
pub use data_word::DataWord;
pub use message::{CommandMessage, StatusMessage};
pub use protocol_word::ProtocolWord;
pub use status_word::StatusWord;
pub use transaction::Transaction;

/// Represents MIL-STD-1553 word size (2 bytes)
pub const WORD_SIZE: usize = 2;
