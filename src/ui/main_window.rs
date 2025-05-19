use libadwaita::prelude::*;
use libadwaita::ApplicationWindow;
use gtk4::{glib, Button, Label, Orientation, PolicyType, ScrolledWindow, SignalListItemFactory, SingleSelection};
use crate::ui::service_object::ServiceObject;
use gtk4::gio::ListStore;
use crate::system::systemctl;

pub struct MainWindow {
    window: ApplicationWindow,
    list_store: ListStore,
}

impl MainWindow {
    pub fn new(app: &libadwaita::Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Systemctl Services")
            .default_width(600)
            .default_height(400)
            .build();

        // Create a ListStore with two columns: service name and status
        let list_store: ListStore = ListStore::new(ServiceObject::static_type());
        assert!(list_store.item_type() == ServiceObject::static_type(), "ListStore type mismatch");

        // Create a ListView to display the data
        let factory = SignalListItemFactory::new();
        factory.connect_setup(|_, list_item| {
            let label = Label::new(None);
            list_item.set_child(Some(&label));
        });
        factory.connect_bind(|_, list_item| {
            let label = list_item.child().unwrap().downcast::<Label>().unwrap();
            let item = list_item.item().unwrap().downcast::<glib::Object>().unwrap();
            let name = item.property::<String>("name");
            let status = item.property::<String>("status");
            label.set_text(&format!("{} - {}", name, status));
        });

        let selection_model = SingleSelection::new(Some(&list_store));
        let list_view = gtk4::ListView::new(Some(&selection_model), Some(&factory));

        // Add a scrollable container for the list view
        let scrolled_window = ScrolledWindow::builder()
            .vexpand(true)
            .hscrollbar_policy(PolicyType::Never)
            .build();
        scrolled_window.set_child(Some(&list_view));

        // Fetch and populate the list during initialization
        let services = systemctl::get_services();
        for (name, status) in services {
            let item = ServiceObject::new(&name, &status);
            assert!(item.type_() == ServiceObject::static_type(), "ServiceObject type mismatch");
            list_store.append(&item);
        }

        // Remove the button as it's no longer needed

        // Layout
        let content = gtk4::Box::new(Orientation::Vertical, 10);
        content.append(&scrolled_window);

        window.set_content(Some(&content));

        Self { window, list_store }
    }

    pub fn show(&self) {
        self.window.show();
    }
}
