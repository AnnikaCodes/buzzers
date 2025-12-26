// Adapted from https://docs.rs/eframe/latest/eframe/
mod app;
mod serial;

fn main() {
    // let mut serial_connection = serial::try_get_serial_connection(1);

    // println!("Reading serial messages...");
    // loop {
    //     match serial::get_serial_message(&mut serial_connection) {
    //         Ok(Some(messages)) => {
    //             for message in messages {
    //                 println!("{}", message.to_human_readable());    
    //             }
    //         },
    //         Ok(None) => {
    //             // No message received within timeout
    //         },
    //         Err(e) => {
    //             eprintln!("Error reading serial message: {}", e);
    //         },
    //     }
    // }

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
