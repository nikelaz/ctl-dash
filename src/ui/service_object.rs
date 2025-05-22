use gtk4::glib;
use gtk4::subclass::prelude::*;
use gtk4::glib::prelude::*;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct ServiceObject {
        pub name: std::cell::RefCell<String>,
        pub status: std::cell::RefCell<String>,
        pub description: std::cell::RefCell<String>, 
        pub load_state: std::cell::RefCell<String>,
        pub sub_state: std::cell::RefCell<String>, 
        pub enabled_state: std::cell::RefCell<String>, // New field
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ServiceObject {
        const NAME: &'static str = "ServiceObject";
        type Type = super::ServiceObject;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for ServiceObject {
        fn properties() -> &'static [glib::ParamSpec] {
            use once_cell::sync::Lazy;
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![
                    glib::ParamSpecString::new(
                        "name",
                        "Name",
                        "Service Name",
                        None,
                        glib::ParamFlags::READWRITE,
                    ),
                    glib::ParamSpecString::new(
                        "status",
                        "Status",
                        "Service Status",
                        None,
                        glib::ParamFlags::READWRITE,
                    ),
                    glib::ParamSpecString::new(
                        "description",
                        "Description",
                        "Service Description",
                        None,
                        glib::ParamFlags::READWRITE,
                    ),
                    glib::ParamSpecString::new(
                        "load-state", 
                        "LoadState",
                        "Service Load State",
                        None,
                        glib::ParamFlags::READWRITE,
                    ),
                    glib::ParamSpecString::new(
                        "sub-state", 
                        "SubState",
                        "Service Sub State",
                        None,
                        glib::ParamFlags::READWRITE,
                    ),
                    // New property for enabled_state
                    glib::ParamSpecString::new(
                        "enabled-state",
                        "EnabledState",
                        "Service Enabled State",
                        None,
                        glib::ParamFlags::READWRITE,
                    ),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(
            &self,
            _obj: &Self::Type,
            _id: usize,
            value: &glib::Value,
            pspec: &glib::ParamSpec,
        ) {
            match pspec.name() {
                "name" => {
                    let name: String = value.get().expect("Type conformity checked by `Object::set_property`.");
                    self.name.borrow_mut().replace_range(.., &name);
                }
                "status" => {
                    let status: String = value.get().expect("Type conformity checked by `Object::set_property`.");
                    self.status.borrow_mut().replace_range(.., &status);
                }
                "description" => {
                    let description: String = value.get().expect("Type conformity checked by `Object::set_property`.");
                    self.description.borrow_mut().replace_range(.., &description);
                }
                "load-state" => { 
                    let load_state: String = value.get().expect("Type conformity checked by `Object::set_property`.");
                    self.load_state.borrow_mut().replace_range(.., &load_state);
                }
                "sub-state" => { 
                    let sub_state: String = value.get().expect("Type conformity checked by `Object::set_property`.");
                    self.sub_state.borrow_mut().replace_range(.., &sub_state);
                }
                // Handle setting enabled_state
                "enabled-state" => {
                    let enabled_state: String = value.get().expect("Type conformity checked by `Object::set_property`.");
                    self.enabled_state.borrow_mut().replace_range(.., &enabled_state);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                "name" => self.name.borrow().clone().to_value(),
                "status" => self.status.borrow().clone().to_value(),
                "description" => self.description.borrow().clone().to_value(),
                "load-state" => self.load_state.borrow().clone().to_value(), 
                "sub-state" => self.sub_state.borrow().clone().to_value(),
                // Handle getting enabled_state
                "enabled-state" => self.enabled_state.borrow().clone().to_value(),
                _ => unimplemented!(),
            }
        }
    }
}

glib::wrapper! {
    pub struct ServiceObject(ObjectSubclass<imp::ServiceObject>);
}

impl ServiceObject {
    // Update new function to accept enabled_state
    pub fn new(name: &str, status: &str, description: &str, load_state: &str, sub_state: &str, enabled_state: &str) -> Self { 
        glib::Object::builder::<Self>()
            .property("name", name)
            .property("status", status)
            .property("description", description)
            .property("load-state", load_state) 
            .property("sub-state", sub_state) 
            .property("enabled-state", enabled_state) // Set enabled_state property
            .build()
            .expect("Failed to create ServiceObject")
    }
}
