//! Simulated GPS device
//!
//! Subaddresses:
//!   - 13T - Sends GPS telemetry.

mod model;
mod rt;

pub use model::{GpsTelemetry, GpsTime, Position, Velocity};
pub use rt::run;
