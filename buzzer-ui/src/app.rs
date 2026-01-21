// Initially copied from https://docs.rs/eframe/latest/eframe/
use crate::serial;
use crate::serial::SerialMessage;
use chrono::Local;
use egui::Vec2;

pub struct App {
    messages_received: Vec<String>,
    serial_connection: Box<dyn serialport::SerialPort>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let mut result = Self {
            messages_received: Vec::new(),
            serial_connection: crate::serial::try_get_serial_connection(1),
        };
        // Drain serial connection
        loop {
            match serial::get_serial_message(&mut result.serial_connection) {
                Ok(Some(..)) => {}
                _ => break,
            }
        }
        result
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match serial::get_serial_message(&mut self.serial_connection) {
            Ok(Some(messages)) => {
                for message in messages {
                    match message {
                        crate::serial::SerialMessage::UnknownCommand(..) => {
                            println!("{}", message.to_human_readable())
                        }
                        _ => self.messages_received.push(message.to_human_readable()),
                    };
                }
            }
            Ok(None) => {
                // No message received within timeout
            }
            Err(e) => {
                eprintln!("Error reading serial message: {}", e);
            }
        }
        egui::TopBottomPanel::top("button")
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                                ctx.set_zoom_factor(2.0);
                if ui.add(egui::Button::new("Clear Buzzers").min_size(Vec2::new(25.0, 25.0))).clicked() {
                    // TODO: different types for sent/received messages so we don't need
                    // a dummy time object
                    serial::send_serial_message(&mut self.serial_connection, SerialMessage::Clear(Local::now())).unwrap();
                }

            });
         });
            egui::TopBottomPanel::top("button2")
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                                ctx.set_zoom_factor(2.0);
                if ui.add(egui::Button::new("Play 'Two Bits' (only works between buzz and clear)").min_size(Vec2::new(25.0, 25.0))).clicked() {
                    // TODO: different types for sent/received messages so we don't need
                    // a dummy time object
                    serial::send_serial_message(&mut self.serial_connection, SerialMessage::TwoBits).unwrap();
                }

            });
         });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    let mut last = None;
                    for message in &self.messages_received {
                        last = Some(ui.label(message));
                    }
                    if let Some(scrollto) = last {
                        scrollto.scroll_to_me(None);
                    }
                });
        });
        // todo: advanced — have separate thread watch serial port and only request repaint whjen needed.
        ctx.request_repaint();
    }
}
