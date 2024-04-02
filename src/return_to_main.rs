use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::time::Duration;

// very hacky way to make these values public
fn return_event() -> KeyEvent {
    KeyEvent {
        code: KeyCode::Char('q'),
        modifiers: KeyModifiers::ALT,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    }
}

fn continue_event() -> KeyEvent {
    KeyEvent {
        code: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    }
}

pub fn return_to_main() -> Result<&'static str, Box<dyn std::error::Error>> {
    enable_raw_mode()?;

    loop {
        if event::poll(Duration::from_millis(1000))? {
            if let Event::Key(event) = event::read()? {
                if event == return_event() {
                    return Ok("userReturned");
                }
                if event == continue_event() {
                    return Ok("userContinued");
                }
            }
        }

        disable_raw_mode()?;
    }
}
