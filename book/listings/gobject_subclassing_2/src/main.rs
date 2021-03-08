use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use std::cell::RefCell;

// ANCHOR: impl
// Implementation of our custom GObject
mod imp {
    // Import parent scope
    use super::*;
    // Import necessary traits for subclassing
    use gtk::subclass::prelude::*;

    // Object holding the state
    #[derive(Default)]
    pub struct CustomButton {
        number: RefCell<i32>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for CustomButton {
        const NAME: &'static str = "CustomButton";
        type Type = super::CustomButton;
        type ParentType = gtk::Button;
    }

    // Trait shared by all GObjects
    impl ObjectImpl for CustomButton {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
            obj.set_label(&self.number.borrow().to_string());
        }
    }

    // Trait shared by all widgets
    impl WidgetImpl for CustomButton {}

    // Trait shared by all buttons
    impl ButtonImpl for CustomButton {
        fn clicked(&self, button: &Self::Type) {
            *self.number.borrow_mut() += 1;
            button.set_label(&self.number.borrow().to_string())
        }
    }
}
// ANCHOR_END: impl
glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends gtk::Button, gtk::Widget;
}

impl CustomButton {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create Button")
    }
    pub fn with_label(label: &str) -> Self {
        let button = Self::new();
        button.set_label(label);
        button
    }
}

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example.Devel"), Default::default())
        .expect("Initialization failed...");
    app.connect_activate(|app| on_activate(app));

    // Run the application
    app.run(&std::env::args().collect::<Vec<_>>());
}
// ANCHOR: activate
// When the application is launched…
fn on_activate(application: &Application) {
    // … create a new window …
    let window = ApplicationWindow::new(application);

    // Create a button
    let button = CustomButton::new();

    // Add button
    window.set_child(Some(&button));
    window.present();
}
// ANCHOR_END: activate
