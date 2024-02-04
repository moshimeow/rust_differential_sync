use eframe::egui;

struct MyApp {
    text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { text: String::new() }
    }
}


impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Egui Text Edit Box");
            ui.label("Type something in the box:");
            let mut output = ui.text_edit_multiline(&mut self.text);
            ui.add_space(10.0); // Add some space between elements
            ui.label(format!("Current text: {}", self.text));

            ui.horizontal(|ui| {
                ui.label("Move cursor to the:");
    
                if ui.button("start").clicked() {
                    let text_edit_id = output.id;
                    if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), text_edit_id) {
                        // let ccursor = egui::text::CCursor::new(0);
                        let mut ccursor_range = egui::text::CCursorRange::default();
                        ccursor_range.primary = egui::text::CCursor::new(0);
                        ccursor_range.secondary = egui::text::CCursor::new(5);
                        // ccursor.
                        state.set_ccursor_range(Some(ccursor_range));
                            // .ccursor_range.primary = 0;
                            // .set_char_range(Some(egui::text::CCursorRange::one(ccursor)));
                        state.store(ui.ctx(), text_edit_id);
                        ui.ctx().memory_mut(|mem    | mem.request_focus(text_edit_id)); // give focus back to the [`TextEdit`].
                    }
                }
    
            });

        });



       

    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native("mrow 🧐🧐🧐🧐🧐", options, Box::new(|_| Box::new(MyApp::default())) );
}
