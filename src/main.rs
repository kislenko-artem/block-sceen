extern crate gtk;
extern crate glib;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use std::thread;
use std::time::Duration;

use gtk::{Application, ApplicationWindow, Button, GtkWindowExt};

fn main() {

    let application = Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size(350, 70);

        let button = Button::new_with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Clicked!");
        });
        window.add(&button);

        window.show_all();


        GtkWindowExt::fullscreen(&window);


        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            tx.send("#{} Text from another thread.")
                .expect("Couldn't send data to channel");
        });

        rx.attach(None, move |text| {
            GtkWindowExt::close(&window);

            glib::Continue(true)
        });


    });

    application.run(&[]);
}
