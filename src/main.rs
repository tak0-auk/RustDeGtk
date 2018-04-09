extern crate gdk;
extern crate gdk_pixbuf;
extern crate gtk;

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use gtk::prelude::*;
use gtk::{Window, WindowType};

use futures::{Future, Stream};
use hyper::Client;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);

    window.set_title("title");
    window.set_default_size(1280, 640);
    window.set_position(gtk::WindowPosition::Center);
    // window.set_default_icon(Pixbuf::new_from_resource("").unwrap());

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    let search_txt = gtk::Entry::new();
    hbox.pack_start(&search_txt, true, true, 5);

    let button = gtk::Button::new_with_label("search");


    hbox.pack_start(&button, false, false, 5);
    vbox.pack_start(&hbox, false, false, 2);

    let scr_win = gtk::ScrolledWindow::new(None, None);

    let txt_view = gtk::Label::new(None);
    scr_win.add(&txt_view);
    vbox.pack_start(&scr_win, true, true, 2);

    button.connect_clicked(move |_| {


        match search_txt.get_text() {
            Some(s) => {
                let text =  s.to_string();
                if text.len() > 0 {
                    let mut core = Core::new().unwrap();
                    let handle = core.handle();
                    let client = Client::configure()
                        .connector(HttpsConnector::new(4, &handle).unwrap())
                        .build(&handle);

                    let uri = text.parse().unwrap();

                    let get = client.get(uri).and_then(|res| res.body().concat2());
                    let got = core.run(get);

                    match got {
                        Ok(res) =>
                        {
                            txt_view.set_text(std::str::from_utf8(&res).unwrap());
                        },
                        Err(_) => {
                            txt_view.set_text("");
                        },
                    }
                } else {
                    txt_view.set_text("");
                }
            },
            None => {},
        };

    });


    window.connect_delete_event(|_, _| {
        println!("exit");
        // Stop the main loop.
        gtk::main_quit();
        // Let the default handler destroy the window.
        Inhibit(false)
    });

    window.add(&vbox);
    window.show_all();
    gtk::main();
}