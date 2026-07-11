use crate::protocol::{CommandMessage, StatusMessage};

/// Represents a single MIL-STD-1553 transaction consisting of a command
/// message and its corresponding status response.
pub struct Transaction {
    pub command: CommandMessage,
    pub status: StatusMessage,
}
