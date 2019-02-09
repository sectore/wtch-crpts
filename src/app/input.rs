extern crate termion;
extern crate tui;

use std::io;
use std::sync::mpsc;
use std::thread;
use termion::event::Key;
use termion::input::TermRead;

pub enum InputEvent {
    Exit,
    InputKey(Key),
}

pub struct InputChannel {
    pub rx: mpsc::Receiver<InputEvent>,
    #[allow(dead_code)]
    handle: thread::JoinHandle<()>,
}

impl InputChannel {
    pub fn new() -> InputChannel {
        let (tx, rx) = mpsc::channel();
        let handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.keys() {
                    if let Ok(key) = evt {
                            let inp_event = if key == Key::Char('q') {
                                InputEvent::Exit
                            } else {
                                InputEvent::InputKey(key)
                            };
                            if tx.send(inp_event).is_err() {
                                return;
                            }
                        }
                    }
                }
            )
        };
        InputChannel { rx, handle }
    }
}
