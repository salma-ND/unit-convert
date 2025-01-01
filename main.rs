use std::io::{self, Write};

#[derive(Debug)]
enum Unit {
    Celsius,
    Fahrenheit,
    Kelvin,
    Meter,
    Kilometer,
    Gram,
    Kilogram,
    Pound,
    Second,
    Minute,
}

impl Unit {
    fn convert(&self, value: f64, to: &Unit) -> f64 {
        match (self, to) {
            (Unit::Celsius, Unit::Fahrenheit) => value * 1.8 + 32.0,
            (Unit::Fahrenheit, Unit::Celsius) => (value - 32.0) / 1.8,
            (Unit::Celsius, Unit::Kelvin) => value + 273.15,
            (Unit::Kelvin, Unit::Celsius) => value - 273.15,
            (Unit::Fahrenheit, Unit::Kelvin) => (value - 32.0) / 1.8 + 273.15,
            (Unit::Kelvin, Unit::Fahrenheit) => (value - 273.15) * 1.8 + 32.0,
            (Unit::Meter, Unit::Kilometer) => value / 1000.0,
            (Unit::Kilometer, Unit::Meter) => value * 1000.0,
            (Unit::Gram, Unit::Kilogram) => value / 1000.0,
            (Unit::Kilogram, Unit::Gram) => value * 1000.0,
            (Unit::Pound, Unit::Kilogram) => value * 0.453592,
            (Unit::Kilogram, Unit::Pound) => value / 0.453592,
            (Unit::Second, Unit::Minute) => value / 60.0,
            (Unit::Minute, Unit::Second) => value * 60.0,
            _ => value, // If same unit, no conversion
        }
    }
}

fn main() {
    let mut input = String::new();
    println!("Unit Converter");
    println!("Choose a unit to convert from (e.g. Celsius, Fahrenheit, Meter, Kilometer, Gram, Kilogram, Pound, Second, Minute):");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let from_unit_input = input.trim().to_lowercase();

    let mut input = String::new();
    println!("Choose a unit to convert to:");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let to_unit_input = input.trim().to_lowercase();

    let from_unit = match from_unit_input.as_str() {
        "celsius" | "c" => Unit::Celsius,
        "fahrenheit" | "f" => Unit::Fahrenheit,
        "kelvin" | "k" => Unit::Kelvin,
        "meter" | "m" => Unit::Meter,
        "kilometer" | "km" => Unit::Kilometer,
        "gram" | "g" => Unit::Gram,
        "kilogram" | "kg" => Unit::Kilogram,
        "pound" | "lb" => Unit::Pound,
        "second" | "s" => Unit::Second,
        "minute" | "min" => Unit::Minute,
        _ => {
            println!("Invalid unit!");
            return;
        }
    };

    let to_unit = match to_unit_input.as_str() {
        "celsius" | "c" => Unit::Celsius,
        "fahrenheit" | "f" => Unit::Fahrenheit,
        "kelvin" | "k" => Unit::Kelvin,
        "meter" | "m" => Unit::Meter,
        "kilometer" | "km" => Unit::Kilometer,
        "gram" | "g" => Unit::Gram,
        "kilogram" | "kg" => Unit::Kilogram,
        "pound" | "lb" => Unit::Pound,
        "second" | "s" => Unit::Second,
        "minute" | "min" => Unit::Minute,
        _ => {
            println!("Invalid unit!");
            return;
        }
    };

    let mut input = String::new();
    println!("Enter the value to convert:");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let value: f64 = match input.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            println!("Invalid value!");
            return;
        }
    };

    // Display the result before conversion
    println!(
        "You are converting {} {:?} to {:?}. The conversion result is:",
        value, from_unit, to_unit
    );

    // Perform conversion
    let result = from_unit.convert(value, &to_unit);

    // Display result after conversion
    println!("{} {:?} = {} {:?}", value, from_unit, result, to_unit);
}
