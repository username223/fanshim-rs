use crate::MilliCelsius;
use std::io::Read;
use std::{fs, io};

#[derive(Debug, Clone, Copy)]
pub struct Hertz(pub i64);

fn read_file(path: &str) -> io::Result<String> {
    let mut s = String::new();
    fs::File::open(path)
        .and_then(|mut f| f.read_to_string(&mut s))
        .map(|_| s)
}

pub fn get_cpu_temperature() -> MilliCelsius {
    MilliCelsius(
        read_file("/sys/class/thermal/thermal_zone0/temp")
            .or_else(|_| read_file("/sys/class/hwmon/hwmon0/temp1_input"))
            .and_then(|data| match data.trim().parse::<i32>() {
                Ok(x) => Ok(x),
                Err(_) => Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Could not parse cpu temperature",
                )),
            })
            .expect("unable to parse cpu temperature"),
    )
}
