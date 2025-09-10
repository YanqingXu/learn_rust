use serde::{Serialize, Deserialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    enabled: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    // --- Serialization ---
    let config = ServerConfig {
        host: "localhost".to_string(),
        port: 8080,
        enabled: true,
    };

    let json_string = serde_json::to_string_pretty(&config)?;
    println!("Serialized JSON:\n{}", json_string);

    // --- Deserialization ---
    let json_data = r#"
    {
        "host": "127.0.0.1",
        "port": 3000,
        "enabled": false
    }
    "#;

    let deserialized_config: ServerConfig = serde_json::from_str(json_data)?;
    println!("\nDeserialized Config:\n{:#?}", deserialized_config);

    Ok(())
}