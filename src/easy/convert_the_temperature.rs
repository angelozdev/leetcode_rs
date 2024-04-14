pub fn convert_temperature(celsius: f64) -> Vec<f64> {
    vec![to_kelvin(celsius), to_fahrenheit(celsius)]
}

fn to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.80 + 32.00
}

fn to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}
