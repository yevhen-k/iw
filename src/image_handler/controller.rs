use crate::image_handler::ImageSet;
use gtk;
use gtk::prelude::{BuilderExtManual, GtkWindowExt, Inhibit, WidgetExtManual, LayoutExt};
use gtk::{ButtonExt, DialogExt, ImageExt, WidgetExt, ContainerExt};
use gdk::{ScrollDirection, EventType};
use std::path::{PathBuf};
use gdk_pixbuf::InterpType;
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;

const LEFT_KEY: u16 = 113;
const RIGHT_KEY: u16 = 114;
const ESC: u16 = 9;

pub struct Controller {
    pub window: gtk::Window,
    pub image: Rc<gtk::Image>,
    pub orig_image: Rc<gtk::Image>,
    pub image_set: Rc<RefCell<ImageSet>>,
    layout: Rc<gtk::Layout>,
    curr_scale: Rc<RefCell<f32>>,
    scale_factor: Rc<RefCell<f32>>,
}

impl Controller {
    pub fn new(window: gtk::Window, image: gtk::Image, image_set: ImageSet, layout: gtk::Layout) -> Self {
        let orig_image = Rc::new(image);
        let image = orig_image.clone();
        let image_set = Rc::new(RefCell::new(image_set));
        let layout = Rc::new(layout);
        Self {window, image, orig_image, image_set, layout,
            curr_scale: Rc::new(RefCell::new(1.0)),
            scale_factor: Rc::new(RefCell::new(0.10)),
        }
    }

    pub fn set_from_file(&mut self, full_path: &PathBuf) {
        let image = Rc::clone(&self.image);

        println!("empty image path: {:?}", full_path);

        image.set_from_file(full_path);
        let pixbuff = match image.get_pixbuf() {
            Some(pb) => pb,
            None => return,
        };

        let width = pixbuff.get_width();
        let height = pixbuff.get_height();
        self.window.resize(width, height);
        let pb = &image.get_pixbuf().unwrap().copy();
        self.orig_image = Rc::new(gtk::Image::from_pixbuf(pb.as_ref()));
    }

    pub fn init_events(&mut self) {
        // events on image
        let image_set = self.image_set.clone();
        let image_for_key_event = Rc::clone(&self.image);
        let orig_image_for_key_event = Rc::clone(&self.orig_image);
        let curr_scale_for_key_event = self.curr_scale.clone();
        let scale_factor_for_key_event = self.scale_factor.clone();
        let layout = Rc::clone(&self.layout);

        // handle events
        self.window.connect_key_press_event(move |window, event_key| {
            match event_key.get_keycode() {
                Some(LEFT_KEY) => {
                    let mut prev_image = image_set.borrow_mut();
                    let prev_image: PathBuf = match prev_image.prev() {
                        Some(pi) => pi,
                        None => return Inhibit::default(),
                    };
                    let image = &image_for_key_event;
                    image.set_from_file(&prev_image);
                    let pixbuff = image.get_pixbuf().unwrap();
                    let width = pixbuff.get_width();
                    let height = pixbuff.get_height();

                    let orig_image = &orig_image_for_key_event;
                    orig_image.set_from_pixbuf(pixbuff.copy().as_ref());
                    *curr_scale_for_key_event.borrow_mut() = 1.0;
                    *scale_factor_for_key_event.borrow_mut() = 0.10;

                    window.set_title(&prev_image.to_str().unwrap());
                    // window.set_size_request(width, height);
                    window.resize(width, height);
                    layout.set_child_x(image_for_key_event.clone().as_ref(), 0);
                    layout.set_child_y(image_for_key_event.clone().as_ref(), 0);
                    layout.set_size(0, 0);
                }
                Some(RIGHT_KEY) => {
                    let mut next_image = image_set.borrow_mut();
                    let next_image = match next_image.next() {
                        Some(ni) => ni,
                        None => return Inhibit::default(),
                    };
                    let image = &image_for_key_event;
                    image.set_from_file(&next_image);
                    let pixbuff = image.get_pixbuf().unwrap();
                    let width = pixbuff.get_width();
                    let height = pixbuff.get_height();

                    let orig_image = &orig_image_for_key_event;
                    orig_image.set_from_pixbuf(pixbuff.copy().as_ref());
                    *curr_scale_for_key_event.borrow_mut() = 1.0;
                    *scale_factor_for_key_event.borrow_mut() = 0.10;

                    window.set_title(&next_image.to_str().unwrap());
                    window.resize(width, height);
                    layout.set_child_x(image_for_key_event.clone().as_ref(), 0);
                    layout.set_child_y(image_for_key_event.clone().as_ref(), 0);
                    layout.set_size(0, 0);
                }
                Some(ESC) => gtk::main_quit(),
                _ => (),
            };
            Inhibit::default()
        });

        let image_for_scroll_event = Rc::clone(&self.image);
        let orig_image_for_scroll_event = Rc::clone(&self.orig_image);
        let curr_scale_for_scroll_event = self.curr_scale.clone();
        let scale_factor_for_scroll_event = self.scale_factor.clone();
        let layout = Rc::clone(&self.layout);

        // scale image on scroll event
        // and position it properly in the center of the layout
        self.window.add_events(gdk::EventMask::SCROLL_MASK);
        self.window.connect_scroll_event(move |window, scroll_event| {
            match scroll_event.get_direction() {
                ScrollDirection::Up => {
                    let image = &orig_image_for_scroll_event;
                    let mut curr_scale = curr_scale_for_scroll_event.borrow_mut();
                    *curr_scale = *curr_scale + *scale_factor_for_scroll_event.borrow();

                    if *curr_scale >= 4.0 {
                        *curr_scale = *curr_scale - *scale_factor_for_scroll_event.borrow();
                        return Inhibit::default();
                    }

                    let pixbuff = match (&*image).get_pixbuf() {
                        Some(pb) => pb,
                        None => return Inhibit::default(),
                    };
                    let width = pixbuff.get_width();
                    let height = pixbuff.get_height();

                    let dest_width = (width as f64 * *curr_scale as f64).round() as i32;
                    let dest_height = (height as f64 * *curr_scale as f64).round() as i32;
                    let rescaled_pixbuff = &pixbuff.scale_simple(dest_width, dest_height, InterpType::Bilinear);
                    let image = &image_for_scroll_event;
                    image.set_from_pixbuf(rescaled_pixbuff.as_ref());
                    // window.set_size_request(width, height);

                    // translate image to the center of the view
                    let w_width = window.get_allocated_width();
                    let w_height = window.get_allocated_height();
                    let xwc = (w_width as f32 / 2.0) as i32;
                    let ywc = (w_height as f32 / 2.0) as i32;
                    let xic = (dest_width as f32 / 2.0) as i32;
                    let yic = (dest_height as f32 / 2.0) as i32;
                    layout.set_child_x(image.clone().as_ref(), xwc-xic);
                    layout.set_child_y(image.clone().as_ref(), ywc-yic);
                },
                ScrollDirection::Down => {
                    let image = &orig_image_for_scroll_event;
                    let mut curr_scale = curr_scale_for_scroll_event.borrow_mut();
                    *curr_scale = *curr_scale - *scale_factor_for_scroll_event.borrow();

                    let pixbuff = match (&*image).get_pixbuf() {
                        Some(pb) => pb,
                        None => return Inhibit::default(),
                    };
                    let width = pixbuff.get_width();
                    let height = pixbuff.get_height();

                    let dest_width = (width as f64 * *curr_scale as f64).round() as i32;
                    let dest_height = (height as f64 * *curr_scale as f64).round() as i32;

                    if dest_height <= 20 || dest_width <= 20 {
                        *curr_scale = *curr_scale + *scale_factor_for_scroll_event.borrow();
                        return Inhibit::default();
                    }

                    let rescaled_pixbuff = &pixbuff.scale_simple(dest_width, dest_height, InterpType::Bilinear);
                    let image = &image_for_scroll_event;
                    image.set_from_pixbuf(rescaled_pixbuff.as_ref());
                    // translate image to the center of the view
                    let w_width = window.get_allocated_width();
                    let w_height = window.get_allocated_height();
                    let xwc = (w_width as f32 / 2.0) as i32;
                    let ywc = (w_height as f32 / 2.0) as i32;
                    let xic = (dest_width as f32 / 2.0) as i32;
                    let yic = (dest_height as f32 / 2.0) as i32;
                    layout.set_child_x(image.clone().as_ref(), xwc-xic);
                    layout.set_child_y(image.clone().as_ref(), ywc-yic);
                },
                _ => ()
            };
            Inhibit::default()
        });

        self.window.add_events(gdk::EventMask::POINTER_MOTION_MASK
            | gdk::EventMask::BUTTON_PRESS_MASK
            | gdk::EventMask::BUTTON_RELEASE_MASK
        );
        self.window.connect_button_press_event(|window, press_event| {
            // println!("get_position {:?}", press_event.get_position());
            // println!("get_coords {:?}", press_event.get_coords());
            // println!("get_click_count {:?}", press_event.get_click_count());
            Inhibit::default()
        });
        self.window.connect_button_release_event(|window, release_event| {
            // println!("release_event {:?}", release_event);
            Inhibit::default()
        });

        let image = Rc::clone(&self.image);
        let layout = Rc::clone(&self.layout);
        self.window.connect_motion_notify_event(move |window, motion_event| {
            let (x_event, y_event)  = motion_event.get_position();
            let (x_root, y_root)  = motion_event.get_root();
            // println!("event {:?}", (x_event, y_event));
            // println!("root {:?}", (x_root, y_root));

            let x = &layout.get_child_x(&image.clone().as_ref().to_owned());
            let y = &layout.get_child_y(&image.clone().as_ref().to_owned());
            // println!("layout {:?}", (x, y));
            Inhibit::default()
        });

        // set position of the image on the center of the window while window resize
        let image = Rc::clone(&self.image);
        let layout = Rc::clone(&self.layout);
        // self.window.connect_size_allocate(move |window, rect| {
        self.window.connect_check_resize(move |window| {
            let pixbuff = match (&*image).get_pixbuf() {
                Some(pb) => pb,
                None => return,
            };
            let width = pixbuff.get_width();
            let height = pixbuff.get_height();

            let w_width = window.get_allocated_width();
            let w_height = window.get_allocated_height();

            let xwc = (w_width as f32 / 2.0) as i32;
            let ywc = (w_height as f32 / 2.0) as i32;
            let xic = (width as f32 / 2.0) as i32;
            let yic = (height as f32 / 2.0) as i32;

            layout.set_child_x(image.clone().as_ref(), xwc-xic);
            layout.set_child_y(image.clone().as_ref(), ywc-yic);
            window.resize(w_width, w_height);
        });
    }

}
