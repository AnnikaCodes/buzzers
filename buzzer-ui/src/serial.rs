// Code for serial interface with the buzzers

use chrono::DateTime;
use chrono::Local;
use serialport::SerialPort;
use serialport::SerialPortType::UsbPort;
use std::time::Duration;

const TIME_FMT_STRING: &str = "[%H:%M:%S%.3f]";


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
                    }
                    _ => continue,
                }
            }
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
            eprintln!(
                "Couldn't find the buzzer system! Please plug it in to a USB port on this computer. Retrying in {} second(s)...",
                timeout
            );
            std::thread::sleep(Duration::from_secs(timeout));
            return try_get_serial_connection(timeout * 2);
        }
        Some(Ok(port)) => port,
        Some(Err(e)) => {
            eprintln!(
                "Error when trying to connect to buzzer system: {}. Exiting...",
                e
            );
            std::process::exit(-1);
        }
    }
}

// See protocol.md
#[derive(Debug)]
pub enum SerialMessage {
    /// Someone buzzed
    Buzz(Color, BuzzerNumber, DateTime<Local>),
    /// Someone tried to buzz and was locked out
    Lockout(Color, BuzzerNumber, DateTime<Local>),
    /// THe buzzers were cleared
    Clear(DateTime<Local>),
    /// We got an unknown command over serial!
    UnknownCommand(String, DateTime<Local>),
}

impl SerialMessage {
    /// Converts a string to a serial message
    pub fn from_raw(msg: char, time: DateTime<Local>) -> Self {
        match msg {
            'x' => SerialMessage::Clear(time),

            'a' => SerialMessage::Lockout(Color::Red, BuzzerNumber::One, time),
            'b' => SerialMessage::Lockout(Color::Red, BuzzerNumber::Two, time),
            'c' => SerialMessage::Lockout(Color::Red, BuzzerNumber::Three, time),
            'd' => SerialMessage::Lockout(Color::Red, BuzzerNumber::Four, time),
            'e' => SerialMessage::Lockout(Color::Red, BuzzerNumber::Five, time),
            'f' => SerialMessage::Lockout(Color::Green, BuzzerNumber::One, time),
            'g' => SerialMessage::Lockout(Color::Green, BuzzerNumber::Two, time),
            'h' => SerialMessage::Lockout(Color::Green, BuzzerNumber::Three, time),
            'i' => SerialMessage::Lockout(Color::Green, BuzzerNumber::Four, time),
            'j' => SerialMessage::Lockout(Color::Green, BuzzerNumber::Five, time),

            'A' => SerialMessage::Buzz(Color::Red, BuzzerNumber::One, time),
            'B' => SerialMessage::Buzz(Color::Red, BuzzerNumber::Two, time),
            'C' => SerialMessage::Buzz(Color::Red, BuzzerNumber::Three, time),
            'D' => SerialMessage::Buzz(Color::Red, BuzzerNumber::Four, time),
            'E' => SerialMessage::Buzz(Color::Red, BuzzerNumber::Five, time),
            'F' => SerialMessage::Buzz(Color::Green, BuzzerNumber::One, time),
            'G' => SerialMessage::Buzz(Color::Green, BuzzerNumber::Two, time),
            'H' => SerialMessage::Buzz(Color::Green, BuzzerNumber::Three, time),
            'I' => SerialMessage::Buzz(Color::Green, BuzzerNumber::Four, time),
            'J' => SerialMessage::Buzz(Color::Green, BuzzerNumber::Five, time),

            _ => SerialMessage::UnknownCommand(msg.to_string(), time),
        }
    }

    pub fn to_human_readable(&self) -> String {
        match self {
            SerialMessage::Buzz(color, number, time) => {
                format!(
                    "{}: {} #{} buzzed!",
                    time.format(TIME_FMT_STRING),
                    color.to_str(),
                    *number as u8
                )
            }
            SerialMessage::Lockout(color, number, time) => {
                format!(
                    "{}: {} #{} was locked out!",
                    time.format(TIME_FMT_STRING),
                    color.to_str(),
                    *number as u8
                )
            }
            SerialMessage::Clear(time) => {
                format!("{}: Buzzers were cleared", time.format(TIME_FMT_STRING))
            }
            SerialMessage::UnknownCommand(cmd, time) => {
                format!("{}: Unknown Command: '{}'", time.format(TIME_FMT_STRING), cmd)
            }
        }
    }
}

// Buzzer colors
#[derive(Debug)]
pub enum Color {
    Red,
    Green,
}

impl Color {
    pub fn to_str(&self) -> &str {
        match self {
            Color::Red => "Red",
            Color::Green => "Green",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BuzzerNumber {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

pub fn get_serial_message(
    connection: &mut Box<dyn SerialPort>,
) -> Result<Option<Vec<SerialMessage>>, std::io::Error> {
    let mut serial_buf: Vec<u8> = vec![0; 32];
    match connection.read(serial_buf.as_mut_slice()) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() == std::io::ErrorKind::TimedOut {
                return Ok(None);
            } else {
                return Err(e);
            }
        }
    }
    let time = Local::now();
    let mut result = Vec::new();
    for byte in serial_buf {
        if byte != 0 && (byte != ' ' as u8) && (byte != '\n' as u8) && (byte != '\r' as u8) {
            result.push(SerialMessage::from_raw(byte as char, time));
        }
    }
    Ok(Some(result))
}

pub fn send_serial_message(connection: &mut Box<dyn SerialPort>, message: SerialMessage) -> Result<(), std::io::Error> {
    let message_raw: u8 = match message {
        SerialMessage::Clear(_) => b'x',
        _ => panic!("Unimplemented: sending serial message not of clear type: {:#?}", message),
    };
    connection.write_all(&[message_raw])
}

