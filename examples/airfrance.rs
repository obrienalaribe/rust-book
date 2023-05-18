#![allow(unused)] // silence unused warnings while exploring (to comment out)

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("./examples/all_flights_today")?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        calculate_flight_time(line.unwrap());
        // println!("{}", line?);
    }
    Ok(())
}

fn calculate_flight_time(value: String) {
    let mut word = String::new();
    let mut items: Vec<String> = vec![];
    for (i, c) in value.chars().enumerate() {
        if c.is_alphanumeric() || c == ' ' || c == ':' {
            &word.push(c);
            continue
        }
        // println!("{:?}", items);
        let sanitized_word_is_empty = word.trim().is_empty();
        if !sanitized_word_is_empty {
            &items.push(word.trim().to_string());
            word = String::new();
        }
    }

    &items.push(value.split(" ").collect::<Vec<&str>>().last().unwrap().trim().to_string());

    // println!("{:?}", items);


    if items.len() == 5 {
        let airfrance_flight = AirFranceFlight {
            from: items[0].to_string(),
            destination: items[1].to_string(),
            flight_time: items[2].parse().unwrap(),
            flight_number: items[3].to_string(),
            aircraft_type: items[4].to_string()
        };


        println!("{:?}", airfrance_flight);

        let airbus_flight_time = airfrance_flight.flight_time / 2;
        // println!("{} to {} => {} takes {}hrs whilst Boeing takes {}hrs ", airfrance_flight.from, airfrance_flight.destination, airfrance_flight.aircraft_type, airbus_flight_time, airfrance_flight.flight_time);

    }
}

#[derive(Debug)]
struct AirFranceFlight {
    from: String,
    destination: String,
    flight_time: u32,
    flight_number: String,
    aircraft_type: String
}