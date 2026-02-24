use std::error::Error;

pub async fn send_alert(message: &str) -> Result<(), Box<dyn Error>> {
    println!(">>> [TELEGRAM] Alerta: {}", message);
    // Aici Axon va folosi configuratia din config.toml pentru bot_token
    Ok(())
}
