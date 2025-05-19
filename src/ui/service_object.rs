use gtk4::glib;
use gtk4::subclass::prelude::*;
use gtk4::glib::prelude::*;

// Define a single mod imp
mod imp {
    use super::*;

    #[derive(Default)]
    pub struct ServiceObject {
        pub name: std::cell::RefCell<String>,
        pub status: std::cell::RefCell<String>,
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
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                "name" => self.name.borrow().clone().to_value(),
                "status" => self.status.borrow().clone().to_value(),
                _ => unimplemented!(),
            }
        }
    }
}

// Define the ServiceObject wrapper only once
glib::wrapper! {
    pub struct ServiceObject(ObjectSubclass<imp::ServiceObject>);
}

impl ServiceObject {
    pub fn new(name: &str, status: &str) -> Self {
        // Fixed property setting format
        glib::Object::builder::<Self>()
            .property("name", name)
            .property("status", status)
            .build()
            .expect("Failed to create ServiceObject")
    }
}
