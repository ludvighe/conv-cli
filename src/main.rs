use crate::temperature::{convert, scale_from_str, scale_to_str};
use clap::{ArgAction, Parser};
use std::process::exit;
use temperature::Scale;

mod temperature;

/// -------------------------------------------------
/// Conv - a converter to convert between unit scales
/// -------------------------------------------------
/// Temperature scales:
///   - °C  Celsius     "c" | "celsius"
///   - °F  Farenheit   "f" | "farenheit"
///   - K   Kelvin      "k" | "kelvin"
///   - °R  Rankine     "r" | "rankine"
///   - °Ré Réaumur     "re" | "ré" | "reaumur" | "réaumur"

#[derive(Parser, Debug, Clone)]
#[command(version, about, verbatim_doc_comment)]
struct Args {
    /// The numerical value to be converted.
    #[arg(allow_hyphen_values = true)]
    value: f32,

    /// The scale from which to convert.
    from: String,
    /// The scale to which the value should be converted.
    to: String,

    #[clap(long, short, action=ArgAction::SetTrue)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    convert_temperature(args);
}

fn convert_temperature(args: Args) -> f32 {
    fn parse_temperature_args(args: Args) -> (Scale, f32, Scale) {
        let value = args.value;
        let from_scale = match scale_from_str(args.from) {
            Some(v) => v,
            None => {
                println!("Invalid temperature scale (from)");
                exit(1);
            }
        };
        let to_scale = match scale_from_str(args.to) {
            Some(v) => v,
            None => {
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
