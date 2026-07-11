//! Configuration used to initialize the simulator

use std::time::Duration;

use crate::{
    protocol::{CmdWord, CommandMessage, Subaddress, TxRx},
    runtime::scheduler::PeriodicCommand,
};

const POWER_RT: u8 = 5;
const GPS_RT: u8 = 13;

pub fn periodic_commands() -> Vec<PeriodicCommand> {
    vec![
        PeriodicCommand {
            interval: Duration::from_secs(1),
            command: CommandMessage {
                word: CmdWord::new(
                    POWER_RT,
                    Subaddress {
                        address: 7,
                        tr: TxRx::T,
                    },
                    3,
                ),
                data: Vec::new(),
            },
        },
        PeriodicCommand {
            interval: Duration::from_secs(2),
            command: CommandMessage {
                word: CmdWord::new(
                    GPS_RT,
                    Subaddress {
                        address: 13,
                        tr: TxRx::T,
                    },
                    15,
                ),
                data: Vec::new(),
            },
        },
    ]
}
