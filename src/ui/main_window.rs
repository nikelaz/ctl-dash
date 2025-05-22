use crate::system::systemctl;
use crate::ui::service_object::ServiceObject;
use gtk4::{
    self,
    gio,
    prelude::*,
    Box as GtkBox,
    Button, // Added Button
    Label,
    SignalListItemFactory,
    GridView,
    ScrolledWindow,
    PolicyType,
    pango,
};
use libadwaita::HeaderBar;
use libadwaita::prelude::*;
use std::ops::{Deref, DerefMut};

pub struct MainWindow {
    window: libadwaita::ApplicationWindow,
}

impl Deref for MainWindow {
    type Target = libadwaita::ApplicationWindow; 

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
        let window = libadwaita::ApplicationWindow::builder() 
            .application(app)
            .title("CTL Dash")
            .default_width(1280) 
            .default_height(720)
            .build();

        let header_bar = HeaderBar::new();

        let main_box = GtkBox::new(gtk4::Orientation::Vertical, 0);
        window.set_content(Some(&main_box));

        main_box.append(&header_bar);

        let loading_label = Label::new(Some("Loading services..."));
        main_box.append(&loading_label);
        
        let headers_box = GtkBox::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .margin_start(10)
            .margin_end(10)
            .margin_top(10)
            .margin_bottom(5)
            .homogeneous(true) 
            .build();
            
        let name_header = Label::builder()
            .label("Service Name")
            .xalign(0.0) 
            .hexpand(true)
            .use_markup(true)
            .build();
        name_header.set_markup("<b>Service Name</b>");
            
        let status_header = Label::builder()
            .label("Status")
            .xalign(0.0)
            .hexpand(true)
            .use_markup(true)
            .build();
        status_header.set_markup("<b>Status</b>");
            
        let description_header = Label::builder()
            .label("Description")
            .xalign(0.0)
            .hexpand(true)
            .use_markup(true)
            .build();
        description_header.set_markup("<b>Description</b>");
            
        let load_state_header = Label::builder()
            .label("Load State")
            .xalign(0.0)
            .hexpand(true)
            .use_markup(true)
            .build();
        load_state_header.set_markup("<b>Load State</b>");
            
        let sub_state_header = Label::builder()
            .label("Sub-State")
            .xalign(0.0)
            .hexpand(true)
            .use_markup(true)
            .build();
        sub_state_header.set_markup("<b>Sub-State</b>");

        let start_stop_actions_header = Label::builder()
            .label("<b>Start/Stop</b>")
            .xalign(0.0)
            .hexpand(true)
            .use_markup(true)
            .build();

        let enable_disable_actions_header = Label::builder()
            .label("<b>Enable/Disable</b>")
            .xalign(0.0)
            .hexpand(true)
            .use_markup(true)
            .build();
            
        // Add headers to the header box
        headers_box.append(&name_header);
        headers_box.append(&status_header);
        headers_box.append(&description_header);
        headers_box.append(&load_state_header);
        headers_box.append(&sub_state_header);
        headers_box.append(&start_stop_actions_header);
        headers_box.append(&enable_disable_actions_header);
        
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
                .margin_start(10)
                .margin_end(10)
                .margin_top(5)
                .margin_bottom(5)
                .homogeneous(true) 
                .build();

            // Add columns: Name, Status, Description, Load State, Sub State
            let name_label = Label::builder()
                .xalign(0.0) // Left-aligned text
                .hexpand(true) // Allow horizontal expansion
                .ellipsize(pango::EllipsizeMode::End) // Add ellipsis if text is too long
                .build();
                
            let status_label = Label::builder()
                .xalign(0.0)
                .hexpand(true)
                .ellipsize(gtk4::pango::EllipsizeMode::End)
                .build();
                
            let description_label = Label::builder()
                .xalign(0.0)
                .hexpand(true)
                .ellipsize(gtk4::pango::EllipsizeMode::End)
                .build();
                
            let load_state_label = Label::builder()
                .xalign(0.0)
                .hexpand(true)
                .ellipsize(gtk4::pango::EllipsizeMode::End)
                .build();
                
            let sub_state_label = Label::builder()
                .xalign(0.0)
                .hexpand(true)
                .ellipsize(gtk4::pango::EllipsizeMode::End)
                .build();
            
            // Create a box for action buttons
            let actions_box = GtkBox::builder()
                .orientation(gtk4::Orientation::Horizontal)
                .spacing(6)
                .hexpand(true) // Allow horizontal expansion for the actions box
                .halign(gtk4::Align::End) // Align buttons to the right
                .build();

            let start_stop_button = Button::builder()
                .label("Start/Stop") // Placeholder label
                .build();

            let enable_disable_button = Button::builder()
                .label("Enable/Disable") // Placeholder label
                .build();

            actions_box.append(&start_stop_button);
            actions_box.append(&enable_disable_button);
                
            // Add all columns to the row
            item_box.append(&name_label);
            item_box.append(&status_label);
            item_box.append(&description_label);
            item_box.append(&load_state_label);
            item_box.append(&sub_state_label);
            item_box.append(&actions_box); // Add actions_box to the item_box
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
            
            // The actions_box is the 6th child (index 5)
            let actions_box = children.get(5).and_then(|w| w.downcast_ref::<GtkBox>()).unwrap();
            
            let mut action_children = Vec::new();
            if let Some(first_action_child) = actions_box.first_child() {
                action_children.push(first_action_child.clone());
                let mut current_action_child = first_action_child;
                while let Some(next_action_child) = current_action_child.next_sibling() {
                    action_children.push(next_action_child.clone());
                    current_action_child = next_action_child;
                }
            }

            let start_stop_button = action_children.get(0).and_then(|w| w.downcast_ref::<Button>()).unwrap();
            let enable_disable_button = action_children.get(1).and_then(|w| w.downcast_ref::<Button>()).unwrap();

            name_label.set_text(&service_object.property::<String>("name"));
            status_label.set_text(&service_object.property::<String>("status"));
            description_label.set_text(&service_object.property::<String>("description"));
            load_state_label.set_text(&service_object.property::<String>("load-state"));
            sub_state_label.set_text(&service_object.property::<String>("sub-state"));

            let active_state = service_object.property::<String>("status");
            let enabled_state = service_object.property::<String>("enabled-state");

            if active_state == "active" {
                start_stop_button.set_label("Stop");
            } else {
                start_stop_button.set_label("Start");
            }

            if enabled_state == "enabled" {
                enable_disable_button.set_label("Disable");
            } else {
                enable_disable_button.set_label("Enable");
                // TODO: Optionally make button insensitive if enabled_state is "static"
                // if enabled_state == "static" {
                // enable_disable_button.set_sensitive(false);
                // } else {
                // enable_disable_button.set_sensitive(true);
                // }
            }
            
            let model_clone_for_handlers = model.clone();
            let loading_label_clone_for_handlers = loading_label.clone();

            let service_name = service_object.property::<String>("name");
            // No need to clone service_object for property changes if we refresh fully
            // let start_stop_button_clone = start_stop_button.clone(); 

            let model_refresh_start_stop = model_clone_for_handlers.clone();
            let loading_label_refresh_start_stop = loading_label_clone_for_handlers.clone();
            let service_name_clone_start_stop = service_name.clone();

            start_stop_button.connect_clicked(move |_| {
                let current_active_state = service_object.property::<String>("status"); // Read current state directly
                if current_active_state == "active" {
                    match systemctl::stop_service(&service_name_clone_start_stop) {
                        Ok(_) => {
                            println!("Successfully stopped {}", service_name_clone_start_stop);
                            loading_label_refresh_start_stop.set_visible(true);
                            model_refresh_start_stop.clear();
                            let model_clone_inner = model_refresh_start_stop.clone();
                            let loading_label_clone_inner = loading_label_refresh_start_stop.clone();
                            systemctl::get_services_async(move |services| {
                                for (name, description, load_state, active_state, sub_state, enabled_state) in services {
                                    let service = ServiceObject::new(&name, &active_state, &description, &load_state, &sub_state, &enabled_state);
                                    model_clone_inner.append(&service);
                                }
                                loading_label_clone_inner.set_visible(false);
                            });
                        }
                        Err(e) => eprintln!("Error stopping service {}: {:?}", service_name_clone_start_stop, e),
                    }
                } else {
                    match systemctl::start_service(&service_name_clone_start_stop) {
                        Ok(_) => {
                            println!("Successfully started {}", service_name_clone_start_stop);
                            loading_label_refresh_start_stop.set_visible(true);
                            model_refresh_start_stop.clear();
                            let model_clone_inner = model_refresh_start_stop.clone();
                            let loading_label_clone_inner = loading_label_refresh_start_stop.clone();
                            systemctl::get_services_async(move |services| {
                                for (name, description, load_state, active_state, sub_state, enabled_state) in services {
                                    let service = ServiceObject::new(&name, &active_state, &description, &load_state, &sub_state, &enabled_state);
                                    model_clone_inner.append(&service);
                                }
                                loading_label_clone_inner.set_visible(false);
                            });
                        }
                        Err(e) => eprintln!("Error starting service {}: {:?}", service_name_clone_start_stop, e),
                    }
                }
            });

            let model_refresh_enable_disable = model_clone_for_handlers.clone();
            let loading_label_refresh_enable_disable = loading_label_clone_for_handlers.clone();
            let service_name_clone_enable_disable = service_name.clone();
            // let enable_disable_button_clone = enable_disable_button.clone(); 

            enable_disable_button.connect_clicked(move |_| {
                let current_enabled_state = service_object.property::<String>("enabled-state"); // Read current state
                if current_enabled_state == "enabled" {
                    match systemctl::disable_service(&service_name_clone_enable_disable) {
                        Ok(_) => {
                            println!("Successfully disabled {}", service_name_clone_enable_disable);
                            loading_label_refresh_enable_disable.set_visible(true);
                            model_refresh_enable_disable.clear();
                            let model_clone_inner = model_refresh_enable_disable.clone();
                            let loading_label_clone_inner = loading_label_refresh_enable_disable.clone();
                            systemctl::get_services_async(move |services| {
                                for (name, description, load_state, active_state, sub_state, enabled_state) in services {
                                    let service = ServiceObject::new(&name, &active_state, &description, &load_state, &sub_state, &enabled_state);
                                    model_clone_inner.append(&service);
                                }
                                loading_label_clone_inner.set_visible(false);
                            });
                        }
                        Err(e) => eprintln!("Error disabling service {}: {:?}", service_name_clone_enable_disable, e),
                    }
                } else {
                     match systemctl::enable_service(&service_name_clone_enable_disable) {
                        Ok(_) => {
                            println!("Successfully enabled {}", service_name_clone_enable_disable);
                            loading_label_refresh_enable_disable.set_visible(true);
                            model_refresh_enable_disable.clear();
                            let model_clone_inner = model_refresh_enable_disable.clone();
                            let loading_label_clone_inner = loading_label_refresh_enable_disable.clone();
                            systemctl::get_services_async(move |services| {
                                for (name, description, load_state, active_state, sub_state, enabled_state) in services {
                                    let service = ServiceObject::new(&name, &active_state, &description, &load_state, &sub_state, &enabled_state);
                                    model_clone_inner.append(&service);
                                }
                                loading_label_clone_inner.set_visible(false);
                            });
                        }
                        Err(e) => eprintln!("Error enabling service {}: {:?}", service_name_clone_enable_disable, e),
                    }
                }
            });
        });
        
        // Need to clone model and loading_label for the initial load as well.
        let model_initial_load = model.clone();
        let loading_label_initial_load = loading_label.clone();

        let selection_model = gtk4::SingleSelection::new(Some(&model_initial_load)); // Use cloned model
        let grid_view = GridView::builder()
            .model(&selection_model)
            .factory(&factory)
            .max_columns(1) 
            .vexpand(true)
            .build();

        let scrolled_window = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Automatic)
            .vscrollbar_policy(PolicyType::Automatic)
            .vexpand(true) 
            .hexpand(true)
            .propagate_natural_width(true)
            .propagate_natural_height(true)
            .child(&grid_view)
            .build();
        
        main_box.append(&scrolled_window);
        
        systemctl::get_services_async(move |services| {
            for (name, description, load_state, active_state, sub_state, enabled_state) in services {
                let service = ServiceObject::new(
                    &name,
                    &active_state,
                    &description,
                    &load_state,
                    &sub_state,
                    &enabled_state, 
                );
                model_initial_load.append(&service); // Use cloned model for initial load
            }

            // Use cloned loading_label for initial load
            let loading_label_clone_for_initial_hide = loading_label_initial_load.clone();
            gtk4::glib::MainContext::default().spawn_local(async move {
                loading_label_clone_for_initial_hide.set_visible(false);
            });
        });
        
        window.present();
        Self { window }
    }
}
