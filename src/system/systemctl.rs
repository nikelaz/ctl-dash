use std::process::Command;

pub fn get_services() -> Vec<(String, String)> {
    let output = Command::new("systemctl")
        .arg("list-units")
        .arg("--type=service")
        .arg("--no-pager")
        .output()
        .expect("Failed to execute systemctl");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut services = Vec::new();

    for line in stdout.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() > 4 {
            let name = parts[0].to_string();
            let status = parts[3].to_string();
            services.push((name, status));
        }
    }

    services
}
