use crate::state::{MouseAction, State};

use enigo;

use enigo::{Enigo, MouseButton, MouseControllable};
use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

pub fn make_mouse_events(state: &'static State) {
    let (tx, rx) = channel::<bool>();
    let filename = "/dev/input/mice";
    let mut f = File::open(filename).ok().unwrap();
    let mut buffer = [0; 3];
    thread::spawn(move || loop {
        f.read(&mut buffer).ok().unwrap();

        if buffer[1..3] != [0, 0] {
            tx.send(false).unwrap();
        }
    });

    let mut enigo = Enigo::new();
    thread::spawn(move || loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Err(_) => match state.value {
                MouseAction::CLICK => enigo.mouse_click(MouseButton::Left),
                MouseAction::DROIT => enigo.mouse_click(MouseButton::Right),
                _ => (),
            },
            _ => (),
        }
    });
}
