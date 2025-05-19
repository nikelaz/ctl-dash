use crate::system::systemctl;
use crate::ui::service_object::ServiceObject;
use gtk4::{
    self,
    gio,
    prelude::*,
    Box as GtkBox,
    Label,
    SignalListItemFactory,
    GridView,
    ScrolledWindow,
    PolicyType,
    StyleContext,
};
use libadwaita::HeaderBar;
use libadwaita::prelude::*;
use std::ops::{Deref, DerefMut};

pub struct MainWindow {
    window: libadwaita::ApplicationWindow, // Specify libadwaita::ApplicationWindow
}

impl Deref for MainWindow {
    type Target = libadwaita::ApplicationWindow; // Specify libadwaita::ApplicationWindow

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl DerefMut for MainWindow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window
    }
}

impl MainWindow {
    pub fn new(app: &libadwaita::Application) -> Self {
        let window = libadwaita::ApplicationWindow::builder() // Specify libadwaita::ApplicationWindow
            .application(app)
            .title("CTL Dash")
            .default_width(800)
            .default_height(600)
            .build();

        let header_bar = HeaderBar::new();

        let main_box = GtkBox::new(gtk4::Orientation::Vertical, 0);
        window.set_content(Some(&main_box));

        main_box.append(&header_bar);

        let loading_label = Label::new(Some("Loading services..."));
        main_box.append(&loading_label);
        
        // Create headers for the table using a standard box
        let headers_box = GtkBox::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .margin_start(5)
            .margin_end(5)
            .margin_top(10)
            .margin_bottom(5)
            .build();
            
        // Create header labels with the same width as the data columns
        let name_header = Label::builder()
            .label("Service Name")
            .width_chars(24)
            .xalign(0.0) // Left-aligned text
            .use_markup(true)
            .build();
        name_header.set_markup("<b>Service Name</b>");
            
        let status_header = Label::builder()
            .label("Status")
            .width_chars(12)
            .xalign(0.0)
            .use_markup(true)
            .build();
        status_header.set_markup("<b>Status</b>");
            
        let description_header = Label::builder()
            .label("Description")
            .width_chars(32)
            .xalign(0.0)
            .use_markup(true)
            .build();
        description_header.set_markup("<b>Description</b>");
            
        let load_state_header = Label::builder()
            .label("Load State")
            .width_chars(12)
            .xalign(0.0)
            .use_markup(true)
            .build();
        load_state_header.set_markup("<b>Load State</b>");
            
        let sub_state_header = Label::builder()
            .label("Sub-State")
            .width_chars(12)
            .xalign(0.0)
            .use_markup(true)
            .build();
        sub_state_header.set_markup("<b>Sub-State</b>");
            
        // Add headers to the header box
        headers_box.append(&name_header);
        headers_box.append(&status_header);
        headers_box.append(&description_header);
        headers_box.append(&load_state_header);
        headers_box.append(&sub_state_header);
        
        // Add a separator under the headers
        let separator = gtk4::Separator::new(gtk4::Orientation::Horizontal);
        
        // Add the headers to the main box before the grid view
        main_box.append(&headers_box);
        main_box.append(&separator);
        
        let model = gio::ListStore::new(ServiceObject::static_type());

        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_factory, list_item| {
            let item_box = GtkBox::builder()
                .orientation(gtk4::Orientation::Horizontal)
                .spacing(12)
                .build();
            // Add columns: Name, Status, Description, Load State, Sub State
            let name_label = Label::builder()
                .width_chars(24)
                .xalign(0.0) // Left-aligned text
                .build();
                
            let status_label = Label::builder()
                .width_chars(12)
                .xalign(0.0)
                .build();
                
            let description_label = Label::builder()
                .width_chars(32)
                .xalign(0.0)
                .build();
                
            let load_state_label = Label::builder()
                .width_chars(12)
                .xalign(0.0)
                .build();
                
            let sub_state_label = Label::builder()
                .width_chars(12)
                .xalign(0.0)
                .build();
                
            // Add all columns to the row
            item_box.append(&name_label);
            item_box.append(&status_label);
            item_box.append(&description_label);
            item_box.append(&load_state_label);
            item_box.append(&sub_state_label);
            list_item.set_child(Some(&item_box));
        });

        factory.connect_bind(move |_factory, list_item| {
            let service_object = list_item
                .downcast_ref::<gtk4::ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .expect("Item should exist")
                .downcast::<ServiceObject>()
                .expect("Item should be ServiceObject");

            let item_box = list_item
                .downcast_ref::<gtk4::ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .expect("Child should exist")
                .downcast::<GtkBox>()
                .expect("Child should be GtkBox");

            // Collect child widgets into a vector
            let mut children = Vec::new();
            
            // Use first_child and next_sibling to iterate over children
            if let Some(first) = item_box.first_child() {
                children.push(first.clone());
                
                let mut current = first;
                while let Some(next) = current.next_sibling() {
                    children.push(next.clone());
                    current = next;
                }
            }
            
            let name_label = children.get(0).and_then(|w| w.downcast_ref::<Label>()).unwrap();
            let status_label = children.get(1).and_then(|w| w.downcast_ref::<Label>()).unwrap();
            let description_label = children.get(2).and_then(|w| w.downcast_ref::<Label>()).unwrap();
            let load_state_label = children.get(3).and_then(|w| w.downcast_ref::<Label>()).unwrap();
            let sub_state_label = children.get(4).and_then(|w| w.downcast_ref::<Label>()).unwrap();

            name_label.set_text(&service_object.property::<String>("name"));
            status_label.set_text(&service_object.property::<String>("status"));
            description_label.set_text(&service_object.property::<String>("description"));
            load_state_label.set_text(&service_object.property::<String>("load-state"));
            sub_state_label.set_text(&service_object.property::<String>("sub-state"));
        });

        let selection_model = gtk4::SingleSelection::new(Some(&model));
        let grid_view = GridView::new(Some(&selection_model), Some(&factory));

        let scrolled_window = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
            .min_content_width(360)
            .child(&grid_view)
            .build();
        
        main_box.append(&scrolled_window);


        // Asynchronously fetch services
        let model_clone = model.clone();
        let loading_label_clone = loading_label.clone();
        systemctl::get_services_async(move |services| {
            println!("Callback received {} services", services.len());
            for (name, description, load_state, active_state, sub_state) in services { // Destructure all five
                println!(
                    "Adding to model: Name: {}, Desc: {}, Load: {}, Active: {}, Sub: {}",
                    name, description, load_state, active_state, sub_state
                );
                // Ensure ServiceObject::new is called with the correct order and all five properties
                // ServiceObject::new(name: &str, status: &str, description: &str, load_state: &str, sub_state: &str)
                let service = ServiceObject::new(
                    &name,
                    &active_state, // 'status' in ServiceObject corresponds to 'active_state' from D-Bus
                    &description,
                    &load_state,
                    &sub_state,
                );
                model_clone.append(&service);
            }
            loading_label_clone.set_visible(false); // Hide loading label
            println!("Finished populating model.");
        });
        
        window.present();
        Self { window } // list_store field removed
    }
}
