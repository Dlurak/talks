fn temperature() -> f32 {
    let env_var = std::env::var("temperature").ok();
    let provided_f32 = env_var.and_then(|v| v.parse().ok());
    provided_f32.unwrap_or(12.0)
}

fn main() {
    let temp = temperature();
    let msg = if temp < 0.0 {
        "Kalt"
    } else if temp < 50.0 {
        "Warm"
    } else {
        "Heiß"
    };

    println!("{msg}");
}
