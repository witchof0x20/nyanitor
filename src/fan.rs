use std::io::{self, BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn lm_sensors(chip_name: &str) -> Result<Vec<(String, i32)>, io::Error> {
    // Run the sensors command and get its output
    let sensors_output = Command::new("sensors")
        .arg(chip_name)
        .stdout(Stdio::piped())
        .output()?;
    // Check status
    if !sensors_output.status.success() {
        panic!("Failed to get sensors");
    }
    // Parse output into lines
    let lines = BufReader::new(sensors_output.stdout.as_slice());
    Ok(lines
        .lines()
        .flatten()
        .skip(2)
        .filter(|line| line.starts_with("fan"))
        .flat_map(|line| {
            let mut tokens = line.split_whitespace();
            let fan_name = tokens.next()?.trim_end_matches(':');
            let rpm = tokens.next()?.parse::<i32>().ok()?;
            Some((fan_name.to_string(), rpm))
        })
        .collect())
}
