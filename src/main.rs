use eframe::{NativeOptions, run_native, App, Frame};
use egui::{CentralPanel, Context, TopBottomPanel, TextEdit, Ui, ScrollArea};

struct TextProcessorApp {
    input_text: String,
    output_text: String,
    selected_option: i32,
}

impl Default for TextProcessorApp {
    fn default() -> Self {
        Self {
            input_text: String::new(),
            output_text: String::new(),
            selected_option: 0,
        }
    }
}

impl App for TextProcessorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // Horní lišta s ovládacími prvky
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Coder Generator");
                ui.separator();
                self.draw_controls(ui);
            });
        });

        // Hlavní panel s textem
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading("Input text:");
                ui.add_space(5.0);
                
                // Vstupní textové pole
                ui.add(TextEdit::multiline(&mut self.input_text)
                    .desired_width(ui.available_width() * 0.95)
                    .desired_rows(8
                    .hint_text("Input text..."));

                // Tlačítko pro zpracování textu
                if ui.button("Process").clicked() {
                    self.process_text();
                }
                
                ui.add_space(10.0);
                ui.heading("Result:");
                ui.add_space(5.0);
                
                // Výstupní textové pole (pouze pro čtení)
                ScrollArea::vertical().show(ui, |ui| {
                    ui.add(TextEdit::multiline(&mut self.output_text.clone())
                        .desired_width(ui.available_width())
                        .desired_rows(8)
                        .interactive(false));
                });
                
                // Tlačítko pro kopírování výsledku
                if ui.button("Copy to clicpboard").clicked() {
                    ui.output_mut(|o| o.copied_text = self.output_text.clone());
                }
            });
        });
    }
}

impl TextProcessorApp {
    fn draw_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Operace:");
            ui.radio_value(&mut self.selected_option, 0, "Žádná změna");
            ui.radio_value(&mut self.selected_option, 1, "Velká písmena");
            ui.radio_value(&mut self.selected_option, 2, "Malá písmena");
            ui.radio_value(&mut self.selected_option, 3, "Počet znaků");
        });
    }

    fn process_text(&mut self) {
        match self.selected_option {
            0 => self.output_text = self.input_text.clone(),
            1 => self.output_text = self.input_text.to_uppercase(),
            2 => self.output_text = self.input_text.to_lowercase(),
            3 => self.output_text = format!("Počet znaků: {}", self.input_text.chars().count()),
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
        "Coder Generator",
        options,
        Box::new(|_cc| Box::new(TextProcessorApp::default())),
    )
    .unwrap();
}