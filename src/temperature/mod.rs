#[derive(Debug, Clone, Copy)]
pub enum Scale {
    Farenheit,
    Celsius,
    Kelvin,
    Rankine,
    Reaumur,
}

pub fn convert(scale: Scale, value: f32, to: Scale) -> f32 {
    match scale {
        Scale::Celsius => match to {
            Scale::Celsius => value,
            Scale::Farenheit => (value * (9.0 / 5.0)) + 32.0, // °F = (°C × (9/5)) + 32
            Scale::Kelvin => value + 273.15,                  // K = °C + 273.15
            Scale::Rankine => (value * (9.0 / 5.0)) + 491.67, // °R = (°C × (9/5)) + 491.67
            Scale::Reaumur => value * (4.0 / 5.0),            // °Ré = °C × (4/5)
        },

        Scale::Farenheit => match to {
            Scale::Farenheit => value,
            Scale::Kelvin => (value + 459.67) * (5.0 / 9.0), // K = (°F + 459.67) × (5/9)
            Scale::Celsius => (value - 32.0) * (5.0 / 9.0),  // °C = (°F − 32) × (5/9)
            Scale::Rankine => value + 459.67,                // °R = °F + 459.67
            Scale::Reaumur => (value - 32.0) * (4.0 / 9.0),  // °Ré = (°F − 32) × (4/9)
        },

        Scale::Kelvin => match to {
            Scale::Kelvin => value,
            Scale::Farenheit => (value * (9.0 / 5.0)) - 459.67, // °F = (K × (9/5)) - 459.67
            Scale::Celsius => value - 273.15,                   // °C = K - 273.15
            Scale::Rankine => value * (9.0 / 5.0),              // °R = K × (9/5)
            Scale::Reaumur => (value - 273.15) * (4.0 / 5.0),   // °Ré = (K − 273.15) × (4/5)
        },

        Scale::Rankine => match to {
            Scale::Rankine => value,
            Scale::Celsius => (value - 491.67) * (5.0 / 9.0), // °C = (°R - 491.67) × (5/9)
            Scale::Farenheit => value - 459.67,               // °F = °R - 459.67
            Scale::Kelvin => value * (5.0 / 9.0),             // K = °R × (5/9)
            Scale::Reaumur => (value - 491.67) * (4.0 / 9.0), // °Ré = (°R − 491.67) × (4/9)
        },

        Scale::Reaumur => match to {
            Scale::Reaumur => value,
            Scale::Farenheit => value * (9.0 / 4.0) + 32.0, // °F = °Ré × 9/4 + 32
            Scale::Celsius => value * (5.0 / 4.0),          // °C = °Ré × 5/4
            Scale::Kelvin => value * (5.0 / 4.0) + 273.15,  // K = °Ré × 5/4 + 273.15
            Scale::Rankine => value * (9.0 / 4.0) + 491.67, // °R = °Ré × 9/4 + 491.67
        },
    }
}

pub fn scale_from_str(s: String) -> Option<Scale> {
    match s.to_lowercase().as_str() {
        "c" | "celsius" => Some(Scale::Celsius),
        "f" | "farenheit" => Some(Scale::Farenheit),
        "k" | "kelvin" => Some(Scale::Kelvin),
        "r" | "rankine" => Some(Scale::Rankine),
        "re" | "ré" | "reaumur" | "réaumur" => Some(Scale::Reaumur),
        _ => None,
    }
}

pub fn scale_to_str(s: Scale) -> &'static str {
    match s {
        Scale::Celsius => "°C",
        Scale::Farenheit => "°F",
        Scale::Kelvin => "K",
        Scale::Rankine => "°R",
        Scale::Reaumur => "°Ré",
    }
}
