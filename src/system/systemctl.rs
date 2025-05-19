use dbus::blocking::Connection;
use gtk4::glib;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

pub fn get_services() -> Vec<(String, String, String, String, String)> {
    let mut services = Vec::new();
    
    match Connection::new_system() {
        Ok(conn) => {
            println!("Debug: Connected to system D-Bus");
            
            let proxy = conn.with_proxy(
                "org.freedesktop.systemd1", 
                "/org/freedesktop/systemd1", 
                Duration::from_secs(5)
            );
            
            println!("Debug: Calling ListUnits method");
            let result: Result<(Vec<(String, String, String, String, String, String, dbus::Path<'static>, u32, String, dbus::Path<'static>)>,), _> = 
                proxy.method_call(
                    "org.freedesktop.systemd1.Manager", 
                    "ListUnits", 
                    ()
                );
                
            match result {
                Ok((units,)) => {
                    println!("Debug: Received {} units from D-Bus", units.len());
                    
                    for unit in units {
                        let name = unit.0;
                        let description = unit.1;
                        let load_state = unit.2;  
                        let active_state = unit.3;
                        let sub_state = unit.4;   
                        
                        if name.ends_with(".service") {
                            services.push((name, description, load_state, active_state, sub_state));
                        }
                    }
                    println!("Debug: Filtered to {} service units", services.len());
                },
                Err(e) => {
                    println!("Debug: Error calling ListUnits: {:?}", e);
                }
            }
        },
        Err(e) => {
            println!("Debug: Failed to connect to system D-Bus: {:?}", e);
        }
    }
    
    services
}

pub fn get_services_async<F: Fn(Vec<(String, String, String, String, String)>) + 'static>(callback: F) { 
    let (sender, receiver) = mpsc::channel();
    
    thread::spawn(move || {
        let services = get_services();
        let _ = sender.send(services);  
    });
    
    glib::source::idle_add_local(move || {
        match receiver.try_recv() {
            Ok(services) => {
                println!("Debug: Received {} services in async callback", services.len());
                callback(services);
                glib::Continue(false) 
            },
            Err(mpsc::TryRecvError::Empty) => {
                glib::Continue(true)
            },
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("Debug: Thread disconnected without sending results");
                callback(Vec::new());
                glib::Continue(false)
            }
        }
    });
}
