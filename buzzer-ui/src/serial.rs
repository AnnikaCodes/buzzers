// Code for serial interface with the buzzers

use serialport::SerialPortType::UsbPort;
use serialport::SerialPort;
use std::time::Duration;

pub fn get_serial_connection() -> Option<Result<Box<dyn SerialPort>, serialport::Error>> {
    let ports = match serialport::available_ports() {
        Ok(ports) => ports,
        Err(e) => return Some(Err(e)),
    };

    for p in ports {
        match p.port_type {
            UsbPort(info) => {
                match info.manufacturer {
                    Some(manufacturer) => {
                        if manufacturer != "Raspberry Pi" {
                            continue;
                        }
                        match info.product {
                            Some(product) => {
                                if product.contains("Pico 2") {
                                    return Some(serialport::new(p.port_name, 115_200) // todo figure out baud?
                                        .timeout(Duration::from_millis(10))
                                        .open());
                                }
                            }
                            None => continue,
                        }
                    },
                    _ => continue,
                }
            },
            _ => continue,
        }
    }
    return None;
}

/// Recursively tries to get a serial connection, doubling the timeout each time
/// Timeout is in units of seconds; exits the process if an error occurs
pub fn try_get_serial_connection(timeout: u64) -> Box<dyn SerialPort> {
    match get_serial_connection() {
        None => {
            eprintln!("Couldn't find the buzzer system! Please plug it in to a USB port on this computer. Retrying in {} second(s)...", timeout);
            std::thread::sleep(Duration::from_secs(timeout));
            return try_get_serial_connection(timeout * 2);
        },
        Some(Ok(port)) => port,
        Some(Err(e)) => {
            eprintln!("Error when trying to connect to buzzer system: {}. Exiting...", e);
            std::process::exit(-1);
        },
    }
}

// See protocol.md
pub enum SerialMessage {
    /// Someone buzzed
    Buzz(Color, BuzzerNumber),
    /// Someone tried to buzz and was locked out
    Lockout(Color, BuzzerNumber),
    /// THe buzzers were cleared
    Clear,
    /// We got an unknown command over serial!
    UnknownCommand(String),
}

impl SerialMessage {
    /// Converts a string to a serial message
    pub fn from_raw(msg: char) -> Self {
        match msg {
            'x' => SerialMessage::Clear,

            'a' => SerialMessage::Lockout(Color::Red, BuzzerNumber::One),
            'b' => SerialMessage::Lockout(Color::Red, BuzzerNumber::Two),
            'c' => SerialMessage::Lockout(Color::Red, BuzzerNumber::Three),
            'd' => SerialMessage::Lockout(Color::Red, BuzzerNumber::Four),
            'e' => SerialMessage::Lockout(Color::Red, BuzzerNumber::Five),
            'f' => SerialMessage::Lockout(Color::Green, BuzzerNumber::One),
            'g' => SerialMessage::Lockout(Color::Green, BuzzerNumber::Two),
            'h' => SerialMessage::Lockout(Color::Green, BuzzerNumber::Three),
            'i' => SerialMessage::Lockout(Color::Green, BuzzerNumber::Four),
            'j' => SerialMessage::Lockout(Color::Green, BuzzerNumber::Five),
            
            'A' => SerialMessage::Buzz(Color::Red, BuzzerNumber::One),
            'B' => SerialMessage::Buzz(Color::Red, BuzzerNumber::Two),
            'C' => SerialMessage::Buzz(Color::Red, BuzzerNumber::Three),
            'D' => SerialMessage::Buzz(Color::Red, BuzzerNumber::Four),
            'E' => SerialMessage::Buzz(Color::Red, BuzzerNumber::Five),
            'F' => SerialMessage::Buzz(Color::Green, BuzzerNumber::One),
            'G' => SerialMessage::Buzz(Color::Green, BuzzerNumber::Two),
            'H' => SerialMessage::Buzz(Color::Green, BuzzerNumber::Three),
            'I' => SerialMessage::Buzz(Color::Green, BuzzerNumber::Four),
            'J' => SerialMessage::Buzz(Color::Green, BuzzerNumber::Five),

            _ => SerialMessage::UnknownCommand(msg.to_string()),
        }
    }

    pub fn to_human_readable(&self) -> String {
        match self {
            SerialMessage::Buzz(color, number) => {
                format!("{} #{} buzzed!", color.to_str(), *number as u8)
            },
            SerialMessage::Lockout(color, number) => {
                format!("{} #{} was locked out!", color.to_str(), *number as u8)
            },
            SerialMessage::Clear => "Buzzers were cleared".to_string(),
            SerialMessage::UnknownCommand(cmd) => format!("Unknown Command: {}", cmd),
        }
    }
}

// Buzzer colors
pub enum Color {
    Red,
    Green
}

impl Color {
    pub fn to_str(&self) -> &str {
        match self {
            Color::Red => "Red",
            Color::Green => "Green",
        }
    }
}

#[derive(Clone, Copy)]
pub enum BuzzerNumber {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

pub fn get_serial_message(connection: &mut Box<dyn SerialPort>) -> Result<Option<Vec<SerialMessage>>, std::io::Error> {
    let mut serial_buf: Vec<u8> = vec![0; 32];
    match connection.read(serial_buf.as_mut_slice()) {
        Ok(_) => {},
        Err(e) => {
            if e.kind() == std::io::ErrorKind::TimedOut {
                return Ok(None);
            } else {
                return Err(e);
            }
        }
    }
    let mut result = Vec::new();
    for byte in serial_buf {
        if byte != 0 {
            result.push(SerialMessage::from_raw(byte as char));
        }
    }
    Ok(Some(result))
}