extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow, Builder, Button, Grid, LayoutBuilder};

use std::env::args;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// upgrade weak reference or return
#[macro_export]
macro_rules! upgrade_weak {
    ($x:ident, $r:expr) => {{
        match $x.upgrade() {
            Some(o) => o,
            None => return $r,
        }
    }};
    ($x:ident) => {
        upgrade_weak!($x, ())
    };
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("gui.xml");
    let builder = Builder::new_from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    let grid: Grid = builder.get_object("grid").expect("Couldn't get grid");
    let button6: Button = builder.get_object("button6").expect("Couldn't get button6");
    let weak_grid = grid.downgrade();
    button6.connect_clicked(move |button| {
        let grid = upgrade_weak!(weak_grid);
        let height = grid.get_cell_height(button);
        let new_height = if height == 2 { 1 } else { 2 };
        grid.set_cell_height(button, new_height);
    });
    let button7: Button = builder.get_object("button7").expect("Couldn't get button7");
    let weak_grid = grid.downgrade();
    button7.connect_clicked(move |button| {
        let grid = upgrade_weak!(weak_grid);
        let left_attach = grid.get_cell_left_attach(button);
        let new_left_attach = if left_attach == 2 { 0 } else { left_attach + 1 };
        grid.set_cell_left_attach(button, new_left_attach);
    });
    button7.connect_enter_notify_event(|button, _| {
        button.set_label("test");
        Inhibit(false)
    });

    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.grid"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
