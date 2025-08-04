use eframe::egui;
use emora_core::{plot_pad_vector, EmotionEngine};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Emora Core Debugger",
        options,
        Box::new(|_cc| Ok(Box::<EmoraDebugger>::default())),
    )
}

#[derive(Default)]
struct EmoraDebugger {
    engine: EmotionEngine,
    stimulus: String, 
}

impl eframe::App for EmoraDebugger {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Emora Control");
            
            ui.horizontal(|ui| {
                ui.label("Input Emo:");
                ui.text_edit_singleline(&mut self.stimulus);
                if ui.button("Send").clicked() {
                    self.engine.update();
                }
            });

            ui.label(format!("current emo: {:?}", self.engine.current_pad.to_emotion()));

            plot_pad_vector(ui, &self.engine.current_pad);
        });
    }
}