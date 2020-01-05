use crate::state::{MouseAction, State};

use enigo;

use enigo::{Enigo, MouseButton, MouseControllable};
use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Duration;

pub fn make_mouse_events(state: &'static State) {
    let (tx, rx) = channel::<bool>();
    let filename = "/dev/input/mice";
    let mut f = File::open(filename).ok().unwrap();
    let mut buffer = [0; 3];

    let event_thread = thread::spawn(move || {
        let enigo = &mut Enigo::new();
        loop {
            trigger_mouse_event(state, &rx, enigo)
        }
    });
    thread::spawn(move || loop {
        f.read(&mut buffer).ok().unwrap();

        if buffer[1..3] != [0, 0] {
            tx.send(false).unwrap();
            event_thread.thread().unpark();
        }
    });
}

fn trigger_mouse_event(state: &'static State, rx: &Receiver<bool>, enigo: &mut Enigo) {
    match rx.recv_timeout(Duration::from_secs(1)) {
        Err(_) => {
            match state.value {
                MouseAction::CLICK => enigo.mouse_click(MouseButton::Left),
                MouseAction::DROIT => enigo.mouse_click(MouseButton::Right),
                _ => (),
            };
            thread::park()
        }
        _ => (),
    }
}
