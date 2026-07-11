use ratatui::crossterm::event;
use tokio::sync::mpsc;

/// Reads keyboard events in a dedicated blocking thread and forwards them
/// over a channel to the asynchronous application.
pub fn key_events(tx: mpsc::Sender<event::KeyEvent>) -> std::io::Result<()> {
    loop {
        if let Some(key_event) = event::read()?.as_key_press_event()
            && tx.blocking_send(key_event).is_err()
        {
            break;
        }
    }

    Ok(())
}
