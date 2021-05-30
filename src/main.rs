mod image_handler;

use crate::image_handler::{Controller, ImageSet};
use gdk_pixbuf::{PixbufLoader, PixbufLoaderExt};
use gtk;
use gtk::prelude::{BuilderExtManual, GtkWindowExt};
use gtk::WidgetExt;
use std::env;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

/*
https://gtk-rs.org/docs-src/tutorial/glade
https://valadoc.org/gtk+-3.0/index.htm
https://docs.rs/gtk/0.9.2/gtk/
https://www.reddit.com/r/rust/comments/hr0wmc/gtkrs_gladis_easily_import_gladegenerated_ui/

https://stackoverflow.com/questions/10355779/how-to-catch-gtk-scroll-event-on-menu-item
https://stackoverflow.com/questions/9852359/how-to-drag-images-with-pygtk
http://www.kcjengr.com/programing/2017/10/16/dragable-gtk-widgets.html
https://www.youtube.com/watch?v=u4YoV-hHu-k
*/

const SUPPORTED_FORMATS: [&str; 7] = ["bmp", "png", "gif", "jpg", "jpeg", "tif", "tiff"];

// fn _main() {
//     if gtk::init().is_err() {
//         println!("Failed to initialize GTK.");
//         return;
//     }
//     let glade_src = read_to_string("./basic.glade").unwrap();
//     let builder = gtk::Builder::from_string(glade_src.as_str());
//
//     let window: gtk::Window = builder.get_object("window1").unwrap();
//     let button: gtk::Button = builder.get_object("button1").unwrap();
//     let dialog: gtk::MessageDialog = builder.get_object("messagedialog1").unwrap();
//
//     button.connect_clicked(move |_| {
//         dialog.run();
//         dialog.hide();
//     });
//
//     window.connect_destroy(|_| {
//         gtk::main_quit();
//     });
//
//     window.show_all();
//
//     gtk::main();
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    let image_path = match args.get(1) {
        Some(path) => String::from(path),
        None => String::from(args.get(0).unwrap()),
    };
    let image_path = Path::new(&image_path);
    let full_path = image_path.canonicalize().unwrap();
    let cur_dir = full_path.parent().unwrap();
    println!("full_path: {:?}", full_path);
    println!("cur_dir: {:?}", cur_dir);
    let images = std::fs::read_dir(cur_dir)
        .unwrap()
        .map(|res| res.unwrap().path())
        .filter(|res| {
            let ext = match res.extension() {
                Some(ext) => String::from(ext.to_str().unwrap()),
                None => return false,
            };
            let supported_ext = SUPPORTED_FORMATS.contains(&ext.as_str());
            res.is_file() && supported_ext
        })
        .collect::<Vec<PathBuf>>();
    println!("folder content: {:?}", images);

    if gtk::init().is_err() {
        println!("Failed to initialize GTK");
        std::process::exit(1);
    }
    let glade_src = read_to_string("./src/iw.glade").unwrap();
    // let glade_src = include_str!("./iw.glade");
    let builder = gtk::Builder::from_string(glade_src.as_str());

    let window: gtk::Window = builder.get_object("window").unwrap();
    let icon_data = include_bytes!("./favicon.ico");
    let pixbuf_loader = PixbufLoader::new();
    if pixbuf_loader.write(icon_data).is_ok() {
        if let Some(icon) = pixbuf_loader.get_pixbuf() {
            window.set_icon(Some(&icon));
        }
    }
    pixbuf_loader.close().is_ok();

    let image: gtk::Image = builder.get_object("image").unwrap();
    let layout: gtk::Layout = builder.get_object("layout").unwrap();

    // Controller
    let image_set = ImageSet::new(images, &full_path);
    let mut controller = Controller::new(window, image, image_set, layout);

    println!("{:?}", controller.image_set);

    controller.set_from_file(&full_path);

    controller.window.set_title(full_path.to_str().unwrap());

    controller.init_events();

    controller.window.connect_destroy(|_| gtk::main_quit());

    controller.window.show_all();

    gtk::main();

    // let screen = gdk::Screen::get_default().unwrap();
    // let display = gdk::Display::get_default().unwrap();
    // let screen = display.get_screen(1);
    // println!("{}x{}", screen.get_width(), screen.get_height());
    // println!("{}", gdk::Screen::width());
    // println!("{}", gdk::Screen::height());
}
