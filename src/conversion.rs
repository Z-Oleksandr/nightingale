use eframe::egui;

pub fn bytes_to_mb(bytes: u64) -> f64 {
    bytes as f64 / 1_048_576.0
}

pub fn to_rich_text(text: &str, size: f32) -> egui::RichText {
    egui::RichText::new(text).size(size).color(egui::Color32::WHITE)
}
