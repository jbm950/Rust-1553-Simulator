//! Simulated power device.
//!
//! Device models some simulated physics as well as different modes/faults.
//!
//! Subaddresses:
//!   - 5R - Receives power commands.
//!   - 7T - Sends power telemetry.

mod model;
mod rt;

pub use model::{Fault, Power, PowerCommand, PowerMode, PowerParseError, PowerTelemetry};
pub use rt::run;
