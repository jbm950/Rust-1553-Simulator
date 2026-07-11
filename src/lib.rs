//! A Rust simulation of a MIL-STD-1553 data bus.
//!
//! The project demonstrates protocol design, asynchronous networking,
//! layered architecture, and terminal user interface development.
//!
//! Major modules:
//!
//! - `protocol`: MIL-STD-1553 protocol types
//! - `net`: TCP transport
//! - `devices`: Simulated remote terminals
//! - `runtime`: Runtime orchestration and scheduling
//! - `app`: Application orchestration
//! - `app/tui`: Terminal user interface

pub mod app;
pub mod devices;
pub mod net;
pub mod protocol;
pub mod runtime;
