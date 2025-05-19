use dbus::blocking::Connection;
use gtk4::glib;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

// This is the original synchronous function - will block the UI thread!
pub fn get_services() -> Vec<(String, String, String, String, String)> { // Updated return type
    let mut services = Vec::new();
    
    // Connect to the system bus
    match Connection::new_system() {
        Ok(conn) => {
            println!("Debug: Connected to system D-Bus");
            
            // Create a proxy for systemd's manager interface
            let proxy = conn.with_proxy(
                "org.freedesktop.systemd1", 
                "/org/freedesktop/systemd1", 
                Duration::from_secs(5)
            );
            
            // Call ListUnits method
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
                    // Extract service units and their states
                    for unit in units {
                        let name = unit.0;
                        let description = unit.1; // New
                        let load_state = unit.2;  // New
                        let active_state = unit.3; // Was status
                        let sub_state = unit.4;   // New
                        
                        if name.ends_with(".service") {
                            services.push((name, description, load_state, active_state, sub_state)); // Add new fields
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
            // Just return empty list on error
            println!("Debug: Failed to connect to system D-Bus: {:?}", e);
        }
    }
    
    services
}

// Improved version - non-blocking but keeps the GTK context requirements in mind
pub fn get_services_async<F: Fn(Vec<(String, String, String, String, String)>) + 'static>(callback: F) { // Updated callback signature
    // Create channel to communicate between threads
    let (sender, receiver) = mpsc::channel();
    
    // Create a separate thread to do the heavy D-Bus work
    thread::spawn(move || {
        let services = get_services();
        let _ = sender.send(services);  // Ignore error if receiver is gone
    });
    
    // Create a source that will check for results
    glib::source::idle_add_local(move || {
        match receiver.try_recv() {
            Ok(services) => {
                // We got the services, call the callback
                println!("Debug: Received {} services in async callback", services.len());
                for (name, description, load_state, active_state, sub_state) in &services {
                    println!("Debug: Service: {} - Desc: {} - Load: {} - Active: {} - Sub: {}", name, description, load_state, active_state, sub_state);
                }
                callback(services);
                glib::Continue(false) // Work done, remove source
            },
            Err(mpsc::TryRecvError::Empty) => {
                // No result yet, try again later
                glib::Continue(true)
            },
            Err(mpsc::TryRecvError::Disconnected) => {
                // Thread died without sending a result
                println!("Debug: Thread disconnected without sending results");
                callback(Vec::new());
                glib::Continue(false)
            }
        }
    });
}
