use crate::models::PAD;

pub fn plot_pad_vector(ui: &mut egui::Ui, pad: &PAD) {
    ui.heading("emo version");

    ui.label(format!("pleasure(P): {:.2}", pad.pleasure.min(100.0) / 100.0));
    ui.add(
        egui::ProgressBar::new(pad.pleasure.min(100.0) / 100.0)
            .text("")
            .fill(egui::Color32::from_rgb(255, 105, 180)) // 粉红色
    );
    ui.label(format!("arousal(A): {:.2}", pad.arousal.min(100.0) / 100.0));
    ui.add(
        egui::ProgressBar::new(pad.arousal.min(100.0) / 100.0)
            .text("")
            .fill(egui::Color32::from_rgb(70, 130, 180)) // 钢蓝色
    );

    ui.label(format!("dominance(D): {:.2}", pad.dominance.min(100.0) / 100.0));
    ui.add(
        egui::ProgressBar::new(pad.dominance.min(100.0) / 100.0)
            .text("")
            .fill(egui::Color32::from_rgb(50, 205, 50)) // 绿色
    );
}