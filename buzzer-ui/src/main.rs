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

    // from https://github.com/emilk/eframe_template/blob/main/src/main.rs#L13
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(
                    &include_bytes!("../openclipart-vectors-button-155539.png")[..],
                )
                .expect("Failed to load icon"),
            ),
        ..Default::default()
    };

    match eframe::run_native(
        "Annika's Buzzer System",
        native_options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    ) {
        Ok(_) => println!("GUI ran succesfully."),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(-1);
        }
    }
}
