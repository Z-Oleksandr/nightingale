use conversion::to_rich_text;
use process::{analyze_folder, get_full_output};
use walkdir::WalkDir;
use std::fs::{File, metadata};
use std::io::{self, prelude::*};
use std::collections::HashMap;
use eframe::egui::{self, vec2, ViewportBuilder};
use rfd::FileDialog;

mod conversion;
mod process;

struct Nightingale {
    path: String,
    output_size: String,
    output_full: String,
}

impl Default for Nightingale {
    fn default() -> Self {
        Self {
            path: "".to_owned(),
            output_size: "Here output".to_owned(),
            output_full: "".to_owned(),
        }
    }
}

impl eframe::App for Nightingale {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {

                ui.add_space(20.0);

                ui.label(to_rich_text("Folder Path:", 24.0));

                ui.add_space(10.0);

                ui.text_edit_singleline(&mut self.path);

                if ui.add(egui::Button::new("Select folder")).clicked() {
                    if let Some(path) = FileDialog::new()
                        .set_title("Select Folder")
                        .pick_folder()
                    {
                        self.path = path.to_str().unwrap_or_default().to_string();
                    }
                }

                ui.add_space(20.0);

                if ui.add(egui::Button::new("Calculate Folder Size")
                    .frame(true)
                    .min_size(egui::vec2(200.0, 50.0))).clicked() {
                        if self.path != "" {
                            match analyze_folder(&self.path) {
                                Ok(size) => {
                                    self.output_size = format!(
                                        "Total size: {:.2} MB", size
                                    );
                                    self.output_full = get_full_output();
                                },
                                Err(e) => self.output_size = format!(
                                    "Error: {}", e
                                )
                            }
                        } else {
                            self.output_size = "Path is empty".to_owned();
                        }
                    }

                ui.add_space(20.0);

                ui.label(to_rich_text("Output:", 24.0));

                ui.add_space(10.0);

                ui.add(
                    egui::TextEdit::singleline(&mut self.output_size)
                        .interactive(false).horizontal_align(egui::Align::Center)
                );

                ui.add_space(20.0);

                egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .enable_scrolling(true)
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.output_full)
                            .desired_rows(30)
                            .desired_width(650.0)
                            .frame(true)
                    );

                    ui.add_space(20.0);
                });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
            viewport: ViewportBuilder {
                inner_size: Some(vec2(1200.0, 800.0)),
                ..Default::default()
            },
        ..Default::default()
    };
    eframe::run_native(
        "Nightingale - Folder Size Calculator", 
        options, 
        Box::new(
            |_cc| Ok(Box::new(Nightingale::default()))
        ),
    )
}
