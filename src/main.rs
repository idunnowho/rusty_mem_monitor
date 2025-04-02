use eframe::egui;
use egui_plot::{Line, Plot};  // Removed unused PlotPoints
use sysinfo::{System, SystemExt};
use std::time::Duration;
use rand::Rng;

struct MemoryMonitor {
    sys: System,
    memory_history: Vec<f32>,
    swap_history: Vec<f32>,
    max_history: usize,
    glitch_effect: bool,
    critical_alarm: bool,
}

impl MemoryMonitor {
    fn new() -> Self {
        Self {
            sys: System::new_all(),
            memory_history: Vec::new(),
            swap_history: Vec::new(),
            max_history: 100,
            glitch_effect: false,
            critical_alarm: false,
        }
    }

    fn generate_glitch_text(&self, text: &str) -> String {
        let mut rng = rand::thread_rng();
        text.chars()
            .map(|c| if rng.gen_bool(0.3) { 
                let glitch_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?/~";
                glitch_chars.chars().nth(rng.gen_range(0..glitch_chars.len())).unwrap_or(c)
            } else { 
                c 
            })
            .collect()
    }
}

impl eframe::App for MemoryMonitor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.sys.refresh_memory();
        
        self.glitch_effect = rand::thread_rng().gen_bool(0.05);
        
        let total_memory = self.sys.total_memory() as f64;
        let used_memory = self.sys.used_memory() as f64;
        let memory_percentage = (used_memory / total_memory * 100.0) as f32;
        
        let swap_percentage = if self.sys.total_swap() > 0 {
            (self.sys.used_swap() as f64 / self.sys.total_swap() as f64 * 100.0) as f32
        } else {
            0.0
        };

        // Update history
        self.memory_history.push(memory_percentage);
        self.swap_history.push(swap_percentage);
        if self.memory_history.len() > self.max_history {
            self.memory_history.remove(0);
            self.swap_history.remove(0);
        }

        self.critical_alarm = memory_percentage > 90.0;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().panel_fill = egui::Color32::from_rgb(0, 15, 0);
            
            ui.vertical_centered(|ui| {
                let title = if self.glitch_effect {
                    self.generate_glitch_text("MEMORY MONITOR")
                } else {
                    "MEMORY MONITOR".to_string()
                };
                
                ui.heading(
                    egui::RichText::new(title)
                        .color(if self.critical_alarm {
                            egui::Color32::from_rgb(255, 0, 0)
                        } else {
                            egui::Color32::from_rgb(0, 255, 0)
                        })
                        .monospace()
                );

                ui.add_space(20.0);
                
                ui.label(
                    egui::RichText::new(format!("Memory Usage: {:.1}%", memory_percentage))
                        .color(if memory_percentage > 90.0 {
                            egui::Color32::from_rgb(255, 0, 0)
                        } else if memory_percentage > 70.0 {
                            egui::Color32::from_rgb(255, 255, 0)
                        } else {
                            egui::Color32::from_rgb(0, 255, 0)
                        })
                        .monospace()
                );

                let bar_text = if self.glitch_effect {
                    self.generate_glitch_text(&format!("[{:^50}]", "#".repeat((memory_percentage/2.0) as usize)))
                } else {
                    format!("[{:^50}]", "#".repeat((memory_percentage/2.0) as usize))
                };
                
                ui.label(
                    egui::RichText::new(bar_text)
                        .color(if memory_percentage > 90.0 {
                            egui::Color32::from_rgb(255, 0, 0)
                        } else {
                            egui::Color32::from_rgb(0, 255, 0)
                        })
                        .monospace()
                );

                ui.add_space(20.0);
                
                let plot = Plot::new("memory_usage")
                    .height(200.0)
                    .show_axes([false, true])
                    .show_background(false);
                
                let memory_points: Vec<[f64; 2]> = self.memory_history.iter()
                    .enumerate()
                    .map(|(i, &y)| [i as f64, y as f64])
                    .collect();
                
                let swap_points: Vec<[f64; 2]> = self.swap_history.iter()
                    .enumerate()
                    .map(|(i, &y)| [i as f64, y as f64])
                    .collect();

                plot.show(ui, |plot_ui| {
                    plot_ui.line(
                        Line::new(memory_points)
                            .color(egui::Color32::from_rgb(0, 255, 0))
                            .name("RAM")
                            .width(2.0)
                    );
                    plot_ui.line(
                        Line::new(swap_points)
                            .color(egui::Color32::from_rgb(255, 100, 0))
                            .name("Swap")
                            .width(2.0)
                    );
                });

                if self.critical_alarm {
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new("WARNING: CRITICAL MEMORY USAGE!")
                            .color(egui::Color32::from_rgb(255, 0, 0))
                            .strong()
                            .heading()
                    );
                }

                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new(format!("Total Memory: {:.1} GB", total_memory / 1024.0 / 1024.0 / 1024.0))
                        .color(egui::Color32::from_rgb(0, 255, 255))
                        .monospace()
                );
                ui.label(
                    egui::RichText::new(format!("Used Memory:  {:.1} GB", used_memory / 1024.0 / 1024.0 / 1024.0))
                        .color(egui::Color32::from_rgb(0, 255, 255))
                        .monospace()
                );
            });
        });

        ctx.request_repaint_after(Duration::from_millis(500));
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 700.0])
            .with_title("Memory Monitor - Hacker Edition"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Memory Monitor",
        options,
        Box::new(|_cc| Box::new(MemoryMonitor::new())),
    )
}