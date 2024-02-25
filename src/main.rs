mod length;
mod temperature;

use std::process::exit;

use clap::{ArgAction, Parser};
use length::convert_length;
use temperature::convert_temperature;

/// -----------------------------------------------------------
/// Conv - a converter to convert between unit scales and units
/// -----------------------------------------------------------

/// Temperature scales:
///   - °C  Celsius     "c" | "celsius"
///   - °F  Farenheit   "f" | "farenheit"
///   - K   Kelvin      "k" | "kelvin"
///   - °R  Rankine     "r" | "rankine"
///   - °Ré Réaumur     "re" | "ré" | "reaumur" | "réaumur"

/// Length units:
///   - Planck              "lp" | "planck"
///   - Angstrom            "å" | "a" | "ångström" | "angstrom"
///   - Nanometer           "nm" | "nanometer"
///   - Micron              "μ" | "micron"
///   - Millimeter          "mm" | "millimeter"
///   - Centimeter          "cm" | "centimeter"
///   - Meter               "m" | "meter"
///   - Kilometer           "km" | "kilometer"
///   - LightSecond         "ls" | "lightsecond"
///   - LightMinute         "lm" | "lightminute"
///   - LightHour           "lh" | "lighthour"
///   - LightDay            "ld" | "lightday"
///   - LightYear           "ly" | "lightyear"
///   - AstronomicalUnit    "au" | "astronomical"
///   - Parsec              "ps" | "parsec"
///   - Inch                "in" | "inch"
///   - Foot                "ft" | "foot"
///   - Yard                "yd" | "yard"
///   - Mile                "mi" | "mile"
///   - Capefoot            "cf" | "capefoot"
///   - Rod                 "rd" | "rod"

#[derive(Parser, Debug, Clone)]
#[command(version, about, verbatim_doc_comment)]
struct Args {
    /// The numerical value to be converted.
    #[arg(allow_hyphen_values = true)]
    value: String,

    /// The scale/unit from which to convert.
    from: String,
    /// The scale/unit to which the value should be converted.
    to: String,

    #[clap(long, short, action=ArgAction::SetTrue)]
    verbose: bool,

    /// Determines what to convert - "t" (temperature), "l" (length)
    #[clap(long, short, default_value = "t")]
    mode: String,
}

fn main() {
    let args = Args::parse();

    let parsed_mode = args.mode.trim().to_lowercase();
    let parsed_mode = parsed_mode.as_str();

    match parsed_mode {
        "t" | "temp" | "temperature" => {
            convert_temperature(args);
        }
        "l" | "len" | "length" => {
            convert_length(args);
        }
        _ => {
            println!("Invalid mode");
            exit(1);
        }
    }
}
