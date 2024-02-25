use std::process::exit;

use crate::Args;

#[derive(Debug, Clone, Copy)]
enum Unit {
    Angstrom = 0,
    Nanometer = 1,
    Micron = 2,
    Millimeter = 3,
    Centimeter = 4,
    Meter = 5,
    Kilometer = 6,
    LightSecond = 7,
    LightMinute = 8,
    LightHour = 9,
    LightDay = 10,
    LightYear = 11,
    AstronomicalUnit = 12,
    Parsec = 13,
    Inch = 14,
    Foot = 15,
    Yard = 16,
    Mile = 17,
    Capefoot = 18,
    Rod = 19,
}

impl Unit {
    fn from_str(s: String) -> Result<Unit, ()> {
        let result = match s.to_lowercase().as_str() {
            "å" | "a" | "ångström" | "angstrom" => Unit::Angstrom,
            "nm" | "nanometer" => Unit::Nanometer,
            "μ" | "micron" => Unit::Micron,
            "mm" | "millimeter" => Unit::Millimeter,
            "cm" | "centimeter" => Unit::Centimeter,
            "m" | "meter" => Unit::Meter,
            "km" | "kilometer" => Unit::Kilometer,
            "ls" | "lightsecond" => Unit::LightSecond,
            "lm" | "lightminute" => Unit::LightMinute,
            "lh" | "lighthour" => Unit::LightHour,
            "ld" | "lightday" => Unit::LightDay,
            "ly" | "lightyear" => Unit::LightYear,
            "au" | "astronomical" => Unit::AstronomicalUnit,
            "ps" | "parsec" => Unit::Parsec,
            "in" | "inch" => Unit::Inch,
            "ft" | "foot" => Unit::Foot,
            "yd" | "yard" => Unit::Yard,
            "mi" | "mile" => Unit::Mile,
            "cf" | "capefoot" => Unit::Capefoot,
            "rd" | "rod" => Unit::Rod,
            _ => return Err(()),
        };
        Ok(result)
    }
    fn to_str(&self) -> &str {
        match self {
            Unit::Angstrom => "Å",
            Unit::Nanometer => "nm",
            Unit::Micron => "μm",
            Unit::Millimeter => "mm",
            Unit::Centimeter => "cm",
            Unit::Meter => "m",
            Unit::Kilometer => "km",
            Unit::LightSecond => "lss",
            Unit::LightMinute => "lm",
            Unit::LightHour => "lh",
            Unit::LightDay => "ld",
            Unit::LightYear => "ly",
            Unit::AstronomicalUnit => "AU",
            Unit::Parsec => "ps",
            Unit::Inch => "in",
            Unit::Foot => "ft",
            Unit::Yard => "yd",
            Unit::Mile => "mi",
            Unit::Capefoot => "cf",
            Unit::Rod => "rd",
        }
    }
}

const UNIT_CONVERSIONS: [f64; 20] = [
    1e-10,                   // angstrom
    1e-9,                    // nanometer
    1e-6,                    // micron
    1e-3,                    // millimeter
    1e-2,                    // centimeter
    1.0,                     // meter
    1e3,                     // kilometer
    299_792_458.0,           // light second
    17_987_547_480.0,        // light minute
    1_079_252_848_800.0,     // light hour
    25_902_068_371_200.0,    // light day
    9_460_730_472_580_800.0, // light year
    149_597_870_700.0,       // astronomical unit
    3.085677581e16,          // parsec
    0.0254,                  // inch
    0.3048,                  // foot
    0.9144,                  // yard
    1609.344,                // mile
    0.31485557516,           // capefoot
    0.314856,                // rod
];

pub fn convert_length(args: Args) -> f64 {
    let verbose = args.verbose;

    let value = match args.value.parse::<f64>() {
        Ok(v) => v,
        Err(_) => {
            println!("Invalid value");
            exit(1);
        }
    };

    let (from_unit, from_unit_i) = match Unit::from_str(args.from) {
        Ok(v) => (v, v as usize),
        Err(_) => {
            println!("Invalid unit (from)");
            exit(1);
        }
    };
    let (to_unit, to_unit_i) = match Unit::from_str(args.to) {
        Ok(v) => (v, v as usize),
        Err(_) => {
            println!("Invalid unit (to)");
            exit(1);
        }
    };

    let from_convertion = UNIT_CONVERSIONS[from_unit_i];
    let to_convertion = UNIT_CONVERSIONS[to_unit_i];

    let meter_value = value * from_convertion;
    let result = meter_value / to_convertion;

    if verbose {
        println!("Convering from {from_unit:?} to {to_unit:?}");
        println!(
            "{value} {} = {result} {}",
            from_unit.to_str(),
            to_unit.to_str()
        );
    } else {
        println!("{result}");
    }
    result
}
