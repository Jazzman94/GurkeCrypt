use eframe::{egui, NativeOptions, run_native, App, Frame};
use egui::{Context, FontDefinitions};

mod morse_code;
mod phone_numbers_code;
mod runes_code;

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
        // Přidání vlastního fontu
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "runic_font".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/NotoSansRunic-Regular.ttf")),
        );
        fonts.families.entry(egui::FontFamily::Proportional).or_default().push("runic_font".to_owned());
        ctx.set_fonts(fonts);

        // Horní lišta s ovládacími prvky
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Text Processor");
                ui.separator();
                self.draw_controls(ui);
            });
        });

        // Hlavní panel s textem
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading("Zadej text:");
                ui.add_space(5.0);

                // Vstupní textové pole
                let input_changed = ui.add(egui::TextEdit::multiline(&mut self.input_text)
                    .desired_width(ui.available_width())
                    .desired_rows(8)
                    .hint_text("Sem zadej text...")
                );

                if input_changed.changed() {
                    self.process_text();
                }

                ui.add_space(5.0);

                ui.add_space(10.0);
                ui.heading("Výsledek:");
                ui.add_space(5.0);

                // Výstupní textové pole
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(egui::TextEdit::multiline(&mut self.output_text.clone())
                        .desired_width(ui.available_width())
                        .desired_rows(8)
                        .interactive(false));
                });

                // Tlačítko pro kopírování výsledku
                if ui.button("Kopírovat do schránky").clicked() {
                    ui.output_mut(|o| o.copied_text = self.output_text.clone());
                }
            });
        });
    }
}

impl TextProcessorApp {
    fn draw_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Encode to:");
            ui.radio_value(&mut self.selected_option, 0, "Default");
            ui.radio_value(&mut self.selected_option, 1, "Runes Code");
            ui.radio_value(&mut self.selected_option, 2, "Phone Number");
            ui.radio_value(&mut self.selected_option, 3, "Morse Code");
            ui.radio_value(&mut self.selected_option, 4, "Počet znaků");
        });
    }

    fn process_text(&mut self) {
        match self.selected_option {
            0 => self.output_text = self.input_text.clone(),
            1 => self.output_text = runes_code::to_runes(&self.input_text),
            2 => self.output_text = phone_numbers_code::to_phone_number(&self.input_text),
            3 => self.output_text = morse_code::to_morse(&self.input_text),
            4 => self.output_text = format!("Počet znaků: {}", self.input_text.chars().count()),
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
