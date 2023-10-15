pub enum TimeFormat {
    Hours,
    Minutes,
    Seconds,
    Invalid,
}

pub struct Time {
    pub format: TimeFormat,
    pub value: u32,
}

impl Time {
    pub fn new(input: &str) -> Self {
        let mut value = String::new();
        let mut has_num = false;

        for c in input.chars() {
            match c {
                c if c.is_alphabetic() && !has_num => return Self::invalid(),
                c if c.is_numeric() => {
                    has_num = true;
                    value.push(c)
                }
                'h' | 'H' => return Self::parsed(TimeFormat::Hours, &value),
                'm' | 'M' => return Self::parsed(TimeFormat::Minutes, &value),
                's' | 'S' => return Self::parsed(TimeFormat::Seconds, &value),
                _ => return Self::invalid(),
            }
        }

        Self::invalid()
    }

    pub fn parsed(format: TimeFormat, value: &str) -> Self {
        Self {
            format,
            value: value.parse::<u32>().expect("Failed to parse time"),
        }
    }

    pub fn invalid() -> Self {
        Self {
            format: TimeFormat::Invalid,
            value: 0,
        }
    }

    pub fn get_total_seconds(&self) -> u32 {
        match self.format {
            TimeFormat::Hours => self.value * 60 * 60,
            TimeFormat::Minutes => self.value * 60,
            TimeFormat::Seconds => self.value,
            TimeFormat::Invalid => 0,
        }
    }

    pub fn get_clock_from_seconds(seconds: &u32) -> String {
        let hours = seconds / 60 / 60;
        let minutes = seconds / 60 % 60;
        let seconds = seconds % 60;

        match (hours, minutes, seconds) {
            (0, 0, _) => return format!("{:02}", seconds),
            (0, _, _) => return format!("{:02}:{:02}", minutes, seconds),
            _ => return format!("{:02}:{:02}:{:02}", hours, minutes, seconds),
        }
    }
}
