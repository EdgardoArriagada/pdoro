use std::process;

use std::thread;
use std::time::Duration;

pub fn sleep(secs: u64) {
    thread::sleep(Duration::from_secs(secs))
}

pub fn stderr(msg: &str) {
    eprintln!("{}", msg);
    process::exit(1);
}

pub fn stdout(msg: &str) {
    println!("{}", msg);
    process::exit(0);
}

pub enum TimeFormat {
    Hours,
    Minutes,
    Seconds,
    Invalid,
}

pub struct Time {
    format: TimeFormat,
    value: u32,
}

impl Time {
    pub fn new(format: TimeFormat, value: &str) -> Self {
        Self {
            format,
            value: value.parse::<u32>().unwrap(),
        }
    }

    pub fn new_invalid() -> Self {
        Self {
            format: TimeFormat::Invalid,
            value: 0,
        }
    }
}

pub fn get_time_format(time_str: &str) -> Time {
    let mut result = String::new();
    let mut has_num = false;

    for c in time_str.chars() {
        if c.is_alphabetic() && !has_num {
            return Time::new_invalid();
        }

        has_num = true;

        match c {
            'h' | 'H' => return Time::new(TimeFormat::Hours, &result),
            'm' | 'M' => return Time::new(TimeFormat::Minutes, &result),
            's' | 'S' => return Time::new(TimeFormat::Seconds, &result),
            _ => {
                if c.is_alphabetic() {
                    return Time::new_invalid();
                }
                result.push(c)
            }
        }
    }

    return Time::new_invalid();
}

pub fn get_seconds_from_fromat(time: Time) -> u32 {
    match time.format {
        TimeFormat::Hours => time.value * 60 * 60,
        TimeFormat::Minutes => time.value * 60,
        TimeFormat::Seconds => time.value,
        TimeFormat::Invalid => 0,
    }
}
