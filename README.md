# Conv

Command line tool to convert between scales.

---

## --help, -h

```
-----------------------------------------------------------
Conv - a converter to convert between unit scales and units
-----------------------------------------------------------
Temperature scales:
  - °C  Celsius     "c" | "celsius"
  - °F  Farenheit   "f" | "farenheit"
  - K   Kelvin      "k" | "kelvin"
  - °R  Rankine     "r" | "rankine"
  - °Ré Réaumur     "re" | "ré" | "reaumur" | "réaumur"
Length units:
  - Planck              "lp" | "planck"
  - Angstrom            "å" | "a" | "ångström" | "angstrom"
  - Nanometer           "nm" | "nanometer"
  - Micron              "μ" | "micron"
  - Millimeter          "mm" | "millimeter"
  - Centimeter          "cm" | "centimeter"
  - Meter               "m" | "meter"
  - Kilometer           "km" | "kilometer"
  - LightSecond         "ls" | "lightsecond"
  - LightMinute         "lm" | "lightminute"
  - LightHour           "lh" | "lighthour"
  - LightDay            "ld" | "lightday"
  - LightYear           "ly" | "lightyear"
  - AstronomicalUnit    "au" | "astronomical"
  - Parsec              "ps" | "parsec"
  - Inch                "in" | "inch"
  - Foot                "ft" | "foot"
  - Yard                "yd" | "yard"
  - Mile                "mi" | "mile"
  - Capefoot            "cf" | "capefoot"
  - Rod                 "rd" | "rod"

Usage: conv [OPTIONS] <VALUE> <FROM> <TO>

Arguments:
  <VALUE>  The numerical value to be converted
  <FROM>   The scale/unit from which to convert
  <TO>     The scale/unit to which the value should be converted

Options:
  -v, --verbose      
  -m, --mode <MODE>  Determines what to convert - "t" (temperature), "l" (length) [default: t]
  -h, --help         Print help
  -V, --version      Print version
```
