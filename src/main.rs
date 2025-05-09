use eframe::{egui, NativeOptions, run_native, App, Frame};
use egui::{Context, FontDefinitions};
mod morse_code;
mod phone_numbers_code;
mod runes_code;

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

impl App for TextProcessorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "runic_font".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/NotoSansRunic-Regular.ttf")),
        );
        fonts.families.entry(egui::FontFamily::Proportional).or_default().push("runic_font".to_owned());
        ctx.set_fonts(fonts);
        
        // Top panel with heading and encode/decode radio buttons
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Text Processor");
                
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
                    .desired_rows(8)
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
                        .desired_rows(8)
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
            (1, 1) => self.output_text = self.input_text.clone(),                   // Decode (placeholder)
            
            // Phone Code
            (2, 0) => self.output_text = phone_numbers_code::to_phone_number(&self.input_text), // Encode
            (2, 1) => self.output_text = phone_numbers_code::from_phone_number(&self.input_text), // Decode
            
            // Default case
            _ => self.output_text = self.input_text.clone(),
        }
    }
}

fn main() {
    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    
    run_native(
        "Text Processor",
        options,
        Box::new(|_cc| Box::new(TextProcessorApp::default())),
    )
    .unwrap();
}