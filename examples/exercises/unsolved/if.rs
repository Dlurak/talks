// Diese Funktion ignorieren
fn temperature() -> f32 {
    let env_var = std::env::var("temperature").ok();
    let provided_f32 = env_var.and_then(|v| v.parse().ok());
    provided_f32.unwrap_or(12.0)
}

fn main() {
    // Unter 0: Kalt
    // Unter 20: Warm
    // Sonst: Heiß
    let temp = temperature();
    let msg = "Heiß";
    println!("{msg}");
}
