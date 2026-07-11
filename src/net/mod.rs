//! Networking layer for transporting MIL-STD-1553 messages over TCP
//!
//! This module intentionally separates transport concerns from protocol
//! definitions to allow alternate transports in the future.

mod bus;
mod bus_controller;
mod rt;

pub use bus::tcp_bus;
pub use bus_controller::TcpBusController;
pub use rt::TcpRt;
