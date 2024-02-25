use crate::Args;
use std::process::exit;

#[derive(Debug, Clone, Copy)]
pub enum Scale {
    Farenheit,
    Celsius,
    Kelvin,
    Rankine,
    Reaumur,
}

pub fn convert_temperature(args: Args) -> f32 {
    fn parse_temperature_args(args: Args) -> (Scale, f32, Scale) {
        let value = match args.value.trim().parse::<f32>() {
            Ok(v) => v,
            Err(_) => {
                println!("Invalid temperature value");
                exit(1);
            }
        };
        let from_scale = match scale_from_str(args.from) {
            Ok(v) => v,
            Err(_) => {
                println!("Invalid temperature scale (from)");
                exit(1);
            }
        };
        let to_scale = match scale_from_str(args.to) {
            Ok(v) => v,
            Err(_) => {
                println!("Invalid temperature scale (to)");
                exit(1);
            }
        };

        (from_scale, value, to_scale)
    }

    let verbose = args.verbose;

    let (from_scale, value, to_scale) = parse_temperature_args(args);
    let result = convert(from_scale, value, to_scale);

    if verbose {
        println!("Convering from {from_scale:?} to {to_scale:?}");
        println!(
            "{value} {} = {result} {}",
            scale_to_str(from_scale),
            scale_to_str(to_scale)
        );
    } else {
        println!("{result}");
    }
    result
}

fn convert(scale: Scale, value: f32, to: Scale) -> f32 {
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

fn scale_from_str(s: String) -> Result<Scale, ()> {
    match s.to_lowercase().as_str() {
        "c" | "celsius" => Ok(Scale::Celsius),
        "f" | "farenheit" => Ok(Scale::Farenheit),
        "k" | "kelvin" => Ok(Scale::Kelvin),
        "r" | "rankine" => Ok(Scale::Rankine),
        "re" | "ré" | "reaumur" | "réaumur" => Ok(Scale::Reaumur),
        _ => Err(()),
    }
}

fn scale_to_str(s: Scale) -> &'static str {
    match s {
        Scale::Celsius => "°C",
        Scale::Farenheit => "°F",
        Scale::Kelvin => "K",
        Scale::Rankine => "°R",
        Scale::Reaumur => "°Ré",
    }
}
