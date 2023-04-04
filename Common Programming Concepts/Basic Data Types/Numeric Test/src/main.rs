fn main() {
    let fahrenheit_degree: f64 = 451f64/*TODO*/;
    let celsius_degree: f64 = (fahrenheit_degree - 32f64) * 0.5556;
    println!("{}°F is {:.1}°C", fahrenheit_degree, celsius_degree);
}
