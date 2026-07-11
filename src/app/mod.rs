//! Application-level orchestration.
//!
//! This module wires together the simulator's runtime, networking,
//! devices, and terminal user interface.

mod config;
mod run;
mod state;
mod tui;

pub use run::run;
