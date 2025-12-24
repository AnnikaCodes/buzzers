// Adapted from https://docs.rs/eframe/latest/eframe/
mod app;
use serialport::SerialPortType::UsbPort;
use serialport::SerialPort;
use std::time::Duration;

fn get_serial_connection() -> Option<Result<Box<dyn SerialPort>, serialport::Error>> {
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
fn try_get_serial_connection(timeout: u64) -> Box<dyn SerialPort> {
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

fn main() {
    let serial_connection = try_get_serial_connection(1);

    let native_options = eframe::NativeOptions::default();
    match eframe::run_native(
        "Annika's Buzzer System",
        native_options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc))))
    ) {
        Ok(_) => println!("GUI ran succesfully."),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(-1);
        },
    }
}
