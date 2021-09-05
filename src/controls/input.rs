use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::{Duration, Instant};
use crossterm::{
    event::{self, Event as CEvent, KeyEvent},
};

pub enum Event<I> {
    Input(I),
    Tick,
}

pub fn listen_for_keys() -> Receiver<Event<KeyEvent>> {
    // Setup event listener for user input
    let (tx, rx) = channel();
    let tick_rate = Duration::from_millis(200);
    // Listen for events in a separate thread so we don't block rendering
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    rx
}

#[derive(Clone)]
pub enum PlayerAction {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,

    LookLeft,
    LookRight,
    LookDown,
    LookUp,
}