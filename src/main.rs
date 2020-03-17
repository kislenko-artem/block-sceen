extern crate gtk;
extern crate glib;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::sync::mpsc::channel;
use std::sync::mpsc;

use gtk::{Application, ApplicationWindow, Label, GtkWindowExt, WidgetExt, true_};

mod style;


#[derive(Copy, Clone)]
struct Counter {
    i: i8,
}

impl Counter {
    fn get(&self) -> i8 {
        return self.i;
    }

}

fn run(app: &gtk::Application, duration: i8) {
    let window = ApplicationWindow::new(app);
    window.set_title("First GTK+ Program");
    window.set_default_size(350, 70);

    GtkWindowExt::fullscreen(&window);


    let label = Label::new(Option::Some("Click me!"));
    WidgetExt::set_widget_name(&label, "label");
    label.set_text("Take a break!");

    window.add(&label);

    window.show_all();

    let counter = Counter{
        i: duration
    };
    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    thread::spawn(move || {
        for v in (1..=counter.get()).rev() {
             thread::sleep(Duration::from_secs(1));
             let _ = tx.send(Some(v));
        }
        let _ = tx.send(None);
    });

    rx.attach(None, move |value| match value {
        Some(value) => {
            label.set_text(&format!("{}", value));
            glib::Continue(true)
        }
        None => {
            GtkWindowExt::close(&window);
            glib::Continue(false)
        }
    });
}

fn show_window(duration: i8) {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(style::STYLE.as_bytes())
            .expect("Failed to load CSS");
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        run(app, 10);
    });

    application.run(&[]);
}

fn main() {
    let duration = Arc::new(Mutex::new(10 as i8));
    let (tx, rx) = mpsc::channel();

    println!("start");
    {

        thread::spawn(move || {
            while (true) {
                let (duration, tx) = (duration.clone(), tx.clone());
                thread::spawn(move || {
                    let mut duration = duration.lock().unwrap();
                    *duration += 10;
                    println!("duration add");
                    thread::sleep(Duration::from_secs(5));
                    tx.send(*duration);
                });
            }
        });
    }

    {
        //let duration = duration.clone();
        while (true) {
            // show_window(duration);
            println!("wait for");
            let duration = rx.recv().unwrap();
            println!("wait after");
            println!("dur {}", duration);
        }
    }
}
