// This file contains utility functions for the meshtastic client

pub fn get_config_path() -> String {
    if let Ok(xdg_config_home) = std::env::var("XDG_CONFIG_HOME") {
        format!("{}/meshtacean", xdg_config_home)
    } else {
        let home = std::env::var("HOME").unwrap();
        format!("{}/.config/meshtacean", home)
    }
}

pub fn snake_to_camel(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(f) => f.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        })
        .collect()
}

pub fn camel_to_snake(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                format!("_{}", c.to_lowercase())
            } else {
                c.to_string()
            }
        })
        .collect()
}

pub fn todo() -> Result<(), Box<dyn std::error::Error>> {
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Not implemented",
    )))
}
