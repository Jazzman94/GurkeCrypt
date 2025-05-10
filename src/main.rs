#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{egui, App, Frame, CreationContext};
use egui::{FontDefinitions, FontFamily, FontData};
use image::io::Reader as ImageReader;
use image::GenericImageView;
mod morse_code;
mod phone_numbers_code;
mod runes_code;

const ICON_PNG: &[u8] = include_bytes!("../assets/pickle_icon_128.png");

struct TextProcessorApp {
    input_text: String,
    output_text: String,
    processing_mode: i32,
    operation_mode: i32, // 0 for encode, 1 for decode
    char_count: usize,
}

impl Default for TextProcessorApp {
    fn default() -> Self {
        Self {
            input_text: String::new(),
            output_text: String::new(),
            processing_mode: 0,
            operation_mode: 0,
            char_count: 0,
        }
    }
}

impl TextProcessorApp {
    fn new(cc: &CreationContext<'_>) -> Self {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "runic_font".to_owned(),
            FontData::from_static(include_bytes!("../assets/NotoSansRunic-Regular.ttf")),
        );
        fonts.families.get_mut(&FontFamily::Proportional).unwrap().push("runic_font".to_owned());
        cc.egui_ctx.set_fonts(fonts);
        
        Self::default()
    }
}

impl App for TextProcessorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Top panel with heading and encode/decode radio buttons
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Engurk it!");
                
                // Add some space between title and radio buttons
                ui.add_space(20.0);
                
                // Encode/Decode radio buttons
                ui.label("Operation:");
                if ui.radio_value(&mut self.operation_mode, 0, "Encode").clicked() {
                    self.process_text();
                }
                if ui.radio_value(&mut self.operation_mode, 1, "Decode").clicked() {
                    self.process_text();
                }
            });
        });
        
        // Bottom panel with author info
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Created by: Jiří Jaskowiec");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let email = "jirijaskowiec@seznam.cz";
                    
                    // Make the email look like a hyperlink
                    let response = ui.hyperlink_to(email, format!("mailto:{}", email));
                    
                    // Add a context menu for copying the email
                    response.context_menu(|ui| {
                        if ui.button("Copy Email").clicked() {
                            ui.output_mut(|o| o.copied_text = email.to_string());
                            ui.close_menu();
                        }
                    });
                    
                    ui.label("All feedback is welcomed. Please send it to:");
                });
            });
        });
        
        // Left panel with processing mode buttons
        egui::SidePanel::left("side_panel")
            .resizable(true)
            .default_width(150.0)
            .width_range(120.0..=200.0)
            .show(ctx, |ui| {
                ui.add_space(5.0);
                ui.strong("Codes:");
                ui.add_space(5.0);
                self.draw_controls(ui);
            });

        // Main content in central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading("Enter text:");
                ui.add_space(5.0);
                let input_changed = ui.add(egui::TextEdit::multiline(&mut self.input_text)
                    .desired_width(ui.available_width())
                    .desired_rows(12)
                    .hint_text("Enter text here...")
                );
                
                if input_changed.changed() {
                    // Update character count whenever input changes
                    self.char_count = self.input_text.chars().count();
                    self.process_text();
                }
                
                ui.add_space(10.0);
                ui.heading("Result:");
                ui.add_space(5.0);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(egui::TextEdit::multiline(&mut self.output_text.clone())
                        .desired_width(ui.available_width())
                        .desired_rows(12)
                        .interactive(false));
                });
                
                // Display character count and copy button on the same line
                ui.horizontal(|ui| {
                    ui.label(format!("Character count: {}", self.char_count));
                    
                    // Add flexible space to push the button to the right
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Copy to Clipboard").clicked() {
                            ui.output_mut(|o| o.copied_text = self.output_text.clone());
                        }
                    });
                });
            });
        });
    }
}

impl TextProcessorApp {
    fn draw_controls(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            if ui.radio_value(&mut self.processing_mode, 0, "Morse Code").clicked() {
                self.process_text();
            }
            if ui.radio_value(&mut self.processing_mode, 1, "Runes").clicked() {
                self.process_text();
            }
            if ui.radio_value(&mut self.processing_mode, 2, "Phone Code").clicked() {
                self.process_text();
            }
        });
    }

    fn process_text(&mut self) {
        match (self.processing_mode, self.operation_mode) {
            // Morse Code
            (0, 0) => self.output_text = morse_code::to_morse(&self.input_text),    // Encode
            (0, 1) => self.output_text = morse_code::from_morse(&self.input_text),  // Decode
            
            // Runes
            (1, 0) => self.output_text = runes_code::to_runes(&self.input_text),    // Encode
            (1, 1) => self.output_text = runes_code::from_runes(&self.input_text),   // Decode     
            // Phone Code
            (2, 0) => self.output_text = phone_numbers_code::to_phone_number(&self.input_text), // Encode
            (2, 1) => self.output_text = phone_numbers_code::from_phone_number(&self.input_text), // Decode
            
            // Default case
            _ => self.output_text = self.input_text.clone(),
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let icon = load_icon_from_png();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([400.0, 300.0])
            .with_icon(icon.clone())
            .with_title("GurkeCrypt v0.1"),
        ..Default::default()
    };

    eframe::run_native(
        "GurkeCrypt v0.1", 
        options,
        Box::new(|cc| Box::new(TextProcessorApp::new(cc)))
    )
}

// Načtení ikony z PNG souboru
fn load_icon_from_png() -> egui::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(ICON_PNG)
            .expect("Failed to load icon")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    egui::IconData {
        rgba: icon_rgba,
        width: icon_width as _,
        height: icon_height as _,
    }
}

fn _load_icon_from_file(path: &str) -> Result<egui::IconData, Box<dyn std::error::Error>> {
    let image = ImageReader::open(path)?.decode()?;
    let (width, height) = image.dimensions();
    let rgba = image.into_rgba8().into_raw();
    Ok(egui::IconData {
        rgba,
        width,
        height,
    })
}