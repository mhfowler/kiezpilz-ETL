extern crate serde_json;
use serde_json::{Value};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Read;
use chrono::{DateTime, NaiveDateTime, FixedOffset};
use chrono::TimeZone;
use chrono_tz::Europe::Berlin;


#[derive(Deserialize, Debug)]
struct ResultsFormat {
    messages: Vec<Value>
}

#[derive(Deserialize, Debug)]
struct SensorFormat {
    time: String,
    temperature: String,
    humidity: String,
}


fn main() {
    let mut file = File::open("result.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let json: ResultsFormat =
        serde_json::from_str(&data).expect("JSON was not well-formatted");


    let mut output_file = File::create("output.csv").expect("Failed to create output file");
    output_file.write_all("time,temperature,humidity\n".as_bytes());

    let mut output_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("output.csv")
        .unwrap();

    let messages: Vec<Value> = json.messages;
    let mut i: i32 = 0;
    for message in messages {
        let id = &message["id"];
        let text: String = (&message["text"]).to_string();
        let text = text.replace("\\", "");
        let text = &text[1..(text.len()-1)]; // remove quote at beginning and end
        let entry_result: Result<SensorFormat, _> = serde_json::from_str(&text);

        let cutoff_dt = Berlin.ymd(2021, 2, 3).and_hms(16, 0, 0);

        match entry_result {
            Ok(entry) => {
                let humidity = &entry.humidity;
//                println!("ds: {}", &entry.time);
                let dt_result = NaiveDateTime::parse_from_str(&entry.time, "%a, %d %b %Y %H:%M:%S %Z");
                if  let Err(e) = dt_result {
                    println!("dterr: {}", e)
                }
                let dt = dt_result.expect("++ badly formatted date");
                let dt_with_tz = Berlin.from_local_datetime(&dt).unwrap();
                if dt_with_tz > cutoff_dt {
                    let dt_string = dt_with_tz.to_rfc2822();
                    let humidity = &entry.humidity.replace("%", "");
                    let temperature = &entry.temperature.replace("°C", "");
                    let line = format!("{},{},{}", dt_with_tz.to_string(), temperature, humidity);
                    writeln!(output_file, "{}", &line).expect("Couldn't write to file");
                } else {
//                    println!("dt: {}", dt_with_tz);
                }
            }
            Err(e) => {
                println!("++ not matched: {:?} {}", e, &text)
            }
        }
        i += 1;
    }

    println!("Hello, world h2i!");
}
