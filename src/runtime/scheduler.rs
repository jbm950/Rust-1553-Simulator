//! Schedules and issues periodic commands to the bus controller.

use std::time::Duration;

use tokio::{
    sync::mpsc,
    time::{MissedTickBehavior, interval},
};
use tracing::error;

use crate::protocol::CommandMessage;

/// Defines a command to send at an interval
pub struct PeriodicCommand {
    pub interval: Duration,
    pub command: CommandMessage,
}

/// Starts a configured periodic task.
///
/// Each scheduled command runs in its own asynchronous task and sends
/// commands through a shared channel to the bus controller. This keeps
/// scheduling independent from transport and response handling.
pub async fn run(schedule: PeriodicCommand, tx: mpsc::Sender<CommandMessage>) {
    let mut timer = interval(schedule.interval);

    // Don't attempt to catch up after delays.
    // Resume periodic execution from the current time.
    timer.set_missed_tick_behavior(MissedTickBehavior::Delay);

    timer.tick().await; // Consume initial tick

    loop {
        timer.tick().await;

        if tx.send(schedule.command.clone()).await.is_err() {
            error!("Bus controller channel closed. Scheduler exiting");
            break;
        };
    }
}
