use dbus::blocking::Connection;
use gtk4::glib;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

pub fn get_services() -> Vec<(String, String, String, String, String, String)> {
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
                            // Call GetUnitFileState for each service
                            let enabled_state_result: Result<(String,), _> = proxy.method_call(
                                "org.freedesktop.systemd1.Manager", 
                                "GetUnitFileState", 
                                (name.clone(),)
                            );
                            let enabled_state = match enabled_state_result {
                                Ok((state,)) => state,
                                Err(e) => {
                                    println!("Debug: Error calling GetUnitFileState for {}: {:?}", name, e);
                                    "unknown".to_string() // Default or error state
                                }
                            };
                            services.push((name, description, load_state, active_state, sub_state, enabled_state));
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

pub fn get_services_async<F: Fn(Vec<(String, String, String, String, String, String)>) + 'static>(callback: F) { 
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

pub fn start_service(service_name: &str) -> Result<(), dbus::Error> {
    println!("Attempting to start service: {}", service_name);
    let conn = Connection::new_system()?;
    let proxy = conn.with_proxy(
        "org.freedesktop.systemd1",
        "/org/freedesktop/systemd1",
        Duration::from_secs(5),
    );
    match proxy.method_call(
        "org.freedesktop.systemd1.Manager",
        "StartUnit",
        (service_name, "replace"),
    ) {
        Ok(output @ (_job_path,)) => {
            println!("Successfully called StartUnit for {}: {:?}", service_name, output);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error calling StartUnit for {}: {:?}", service_name, e);
            Err(e)
        }
    }
}

pub fn stop_service(service_name: &str) -> Result<(), dbus::Error> {
    println!("Attempting to stop service: {}", service_name);
    let conn = Connection::new_system()?;
    let proxy = conn.with_proxy(
        "org.freedesktop.systemd1",
        "/org/freedesktop/systemd1",
        Duration::from_secs(5),
    );
    match proxy.method_call(
        "org.freedesktop.systemd1.Manager",
        "StopUnit",
        (service_name, "replace"),
    ) {
        Ok(output @ (_job_path,)) => {
            println!("Successfully called StopUnit for {}: {:?}", service_name, output);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error calling StopUnit for {}: {:?}", service_name, e);
            Err(e)
        }
    }
}

pub fn enable_service(service_name: &str) -> Result<(), dbus::Error> {
    println!("Attempting to enable service: {}", service_name);
    let conn = Connection::new_system()?;
    let proxy = conn.with_proxy(
        "org.freedesktop.systemd1",
        "/org/freedesktop/systemd1",
        Duration::from_secs(5),
    );
    // Parameters: Vec<String> (unit files), bool (runtime), bool (force)
    match proxy.method_call(
        "org.freedesktop.systemd1.Manager",
        "EnableUnitFiles",
        (vec![service_name.to_string()], false, true),
    ) {
        Ok(output @ (_carries_install_info, _changes)) => {
             println!("Successfully called EnableUnitFiles for {}: {:?}", service_name, output);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error calling EnableUnitFiles for {}: {:?}", service_name, e);
            Err(e)
        }
    }
}

pub fn disable_service(service_name: &str) -> Result<(), dbus::Error> {
    println!("Attempting to disable service: {}", service_name);
    let conn = Connection::new_system()?;
    let proxy = conn.with_proxy(
        "org.freedesktop.systemd1",
        "/org/freedesktop/systemd1",
        Duration::from_secs(5),
    );
    // Parameters: Vec<String> (unit files), bool (runtime)
    match proxy.method_call(
        "org.freedesktop.systemd1.Manager",
        "DisableUnitFiles",
        (vec![service_name.to_string()], false),
    ) {
       Ok(output @ (_changes,)) => {
            println!("Successfully called DisableUnitFiles for {}: {:?}", service_name, output);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error calling DisableUnitFiles for {}: {:?}", service_name, e);
            Err(e)
        }
    }
}
