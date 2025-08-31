//! GUI module for CodeBeats
//!
//! Provides a cross-platform graphical interface for configuring and launching
//! the CodeBeats command-line application with different parameters.

use eframe::egui;
use std::process::Command;

/// GUI application state
pub struct CodeBeatsGui {
    // Configuration options
    selected_language: String,
    selected_waveform: String,
    volume: f32,
    filter_cutoff: f32,
    verbose: bool,

    // Available options
    available_languages: Vec<(String, String)>, // (display_name, file_path)
    available_waveforms: Vec<String>,

    // UI state
    status_message: String,
    is_running: bool,
    current_process: Option<std::process::Child>,
}

impl Default for CodeBeatsGui {
    fn default() -> Self {
        let available_languages = Self::discover_language_configs();
        let available_waveforms = vec![
            "natural".to_string(),
            "electronic".to_string(),
            "cyberpunk".to_string(),
            "harmonic".to_string(),
            "triangle".to_string(),
            "saw".to_string(),
            "square".to_string(),
            "fart".to_string(),
        ];

        Self {
            selected_language: "general_programming_language.json".to_string(),
            selected_waveform: "natural".to_string(),
            volume: 0.8,
            filter_cutoff: 1200.0,
            verbose: false,
            available_languages,
            available_waveforms,
            status_message: "Ready to start CodeBeats".to_string(),
            is_running: false,
            current_process: None,
        }
    }
}

impl CodeBeatsGui {
    /// Discover available language configuration files
    fn discover_language_configs() -> Vec<(String, String)> {
        let mut configs = Vec::new();

        // Add default option
        configs.push(("Default (Built-in)".to_string(), "".to_string()));

        // Scan language_configs directory
        if let Ok(entries) = std::fs::read_dir("language_configs") {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("json") {
                        if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                            let display_name = Self::format_language_name(filename);
                            configs.push((display_name, filename.to_string()));
                        }
                    }
                }
            }
        }

        // Sort by display name
        configs.sort_by(|a, b| a.0.cmp(&b.0));
        configs
    }

    /// Format language filename into readable display name
    fn format_language_name(filename: &str) -> String {
        let name = filename.strip_suffix(".json").unwrap_or(filename);

        match name {
            "general_programming_language" => "General Programming".to_string(),
            "c" => "C Language".to_string(),
            "javascript" => "JavaScript".to_string(),
            "python" => "Python".to_string(),
            "rust" => "Rust".to_string(),
            "java" => "Java".to_string(),
            "go" => "Go".to_string(),
            "haskell" => "Haskell".to_string(),
            "clojure" => "Clojure".to_string(),
            "scheme" => "Scheme".to_string(),
            "emacs-lisp" => "Emacs Lisp".to_string(),
            "english" => "English Language".to_string(),
            "chinese" => "Chinese Language".to_string(),
            "japanese" => "Japanese Language".to_string(),
            _ => name
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if i == 0 {
                        c.to_uppercase().collect::<String>()
                    } else {
                        c.to_string()
                    }
                })
                .collect::<String>(),
        }
    }

    /// Start CodeBeats with current configuration
    fn start_codebeats(&mut self) {
        if self.is_running {
            self.stop_codebeats();
            return;
        }

        // Try to use release binary first, fall back to cargo run
        let mut cmd = if std::path::Path::new("target/release/codebeats").exists() {
            Command::new("./target/release/codebeats")
        } else if std::path::Path::new("target/debug/codebeats").exists() {
            Command::new("./target/debug/codebeats")
        } else {
            let mut cargo_cmd = Command::new("cargo");
            cargo_cmd.arg("run").arg("--bin").arg("codebeats").arg("--");
            cargo_cmd
        };

        // Only add -- separator if using cargo
        let using_cargo = cmd.get_program() == "cargo";
        if !using_cargo {
            // For direct binary execution, no -- separator needed
        }

        // Add language config if not default
        if !self.selected_language.is_empty() {
            cmd.arg("-l")
                .arg(format!("language_configs/{}", self.selected_language));
        }

        // Add waveform
        cmd.arg("-w").arg(&self.selected_waveform);

        // Add volume
        cmd.arg("-v").arg(self.volume.to_string());

        // Add filter cutoff
        cmd.arg("--filter-cutoff")
            .arg(self.filter_cutoff.to_string());

        // Add verbose if enabled
        if self.verbose {
            cmd.arg("--verbose");
        }

        let binary_type = if cmd.get_program() == "cargo" {
            "cargo"
        } else {
            "binary"
        };

        self.status_message = format!(
            "Starting CodeBeats ({}) with: {} waveform, {} language, volume {:.1}, filter {:.0}Hz{}",
            binary_type,
            self.selected_waveform,
            self.get_selected_language_display(),
            self.volume,
            self.filter_cutoff,
            if self.verbose {
                ", verbose logging"
            } else {
                ""
            }
        );

        match cmd.spawn() {
            Ok(child) => {
                self.current_process = Some(child);
                self.is_running = true;
                self.status_message
                    .push_str(" - Running! Press Stop to terminate.");
            }
            Err(e) => {
                self.status_message = format!("Failed to start CodeBeats: {}", e);
                self.is_running = false;
            }
        }
    }

    /// Stop the running CodeBeats process
    fn stop_codebeats(&mut self) {
        if let Some(mut child) = self.current_process.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        self.is_running = false;
        self.status_message = "CodeBeats stopped".to_string();
    }

    /// Get display name for selected language
    fn get_selected_language_display(&self) -> String {
        if self.selected_language.is_empty() {
            return "Default".to_string();
        }

        self.available_languages
            .iter()
            .find(|(_, path)| path == &self.selected_language)
            .map(|(display, _)| display.clone())
            .unwrap_or_else(|| self.selected_language.clone())
    }

    /// Check if the running process is still alive
    fn check_process_status(&mut self) {
        if let Some(ref mut child) = self.current_process {
            match child.try_wait() {
                Ok(Some(_)) => {
                    // Process has exited
                    self.current_process = None;
                    self.is_running = false;
                    self.status_message = "CodeBeats process has exited".to_string();
                }
                Ok(None) => {
                    // Process is still running
                }
                Err(_) => {
                    // Error checking process status
                    self.current_process = None;
                    self.is_running = false;
                    self.status_message = "Lost connection to CodeBeats process".to_string();
                }
            }
        }
    }
}

impl eframe::App for CodeBeatsGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check process status
        self.check_process_status();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎵 CodeBeats Configuration");
            ui.separator();

            // Language Configuration
            ui.horizontal(|ui| {
                ui.label("Language/Config:");
                egui::ComboBox::from_id_source("language_combo")
                    .selected_text(self.get_selected_language_display())
                    .show_ui(ui, |ui| {
                        for (display_name, file_path) in &self.available_languages {
                            ui.selectable_value(
                                &mut self.selected_language,
                                file_path.clone(),
                                display_name,
                            );
                        }
                    });
            });

            ui.add_space(10.0);

            // Waveform Selection
            ui.horizontal(|ui| {
                ui.label("Waveform:");
                egui::ComboBox::from_id_source("waveform_combo")
                    .selected_text(&self.selected_waveform)
                    .show_ui(ui, |ui| {
                        for waveform in &self.available_waveforms {
                            ui.selectable_value(
                                &mut self.selected_waveform,
                                waveform.clone(),
                                waveform,
                            );
                        }
                    });
            });

            ui.add_space(10.0);

            // Volume Slider
            ui.horizontal(|ui| {
                ui.label("Volume:");
                ui.push_id("volume_slider", |ui| {
                    ui.add(egui::Slider::new(&mut self.volume, 0.0..=1.0).text(""));
                });
                ui.label(format!("{:.1}", self.volume));
            });

            ui.add_space(10.0);

            // Filter Cutoff Slider
            ui.horizontal(|ui| {
                ui.label("Filter Cutoff:");
                ui.push_id("filter_slider", |ui| {
                    ui.add(egui::Slider::new(&mut self.filter_cutoff, 200.0..=8000.0).text("Hz"));
                });
                ui.label(format!("{:.0} Hz", self.filter_cutoff));
            });

            ui.add_space(10.0);

            // Verbose Checkbox
            ui.checkbox(
                &mut self.verbose,
                "Verbose logging (show keystrokes in terminal)",
            );

            ui.add_space(20.0);

            // Start/Stop Button
            let button_text = if self.is_running {
                "⏹ Stop CodeBeats"
            } else {
                "▶ Start CodeBeats"
            };
            let button_color = if self.is_running {
                egui::Color32::from_rgb(200, 100, 100)
            } else {
                egui::Color32::from_rgb(100, 200, 100)
            };

            ui.horizontal(|ui| {
                if ui
                    .add(egui::Button::new(button_text).fill(button_color))
                    .clicked()
                {
                    self.start_codebeats();
                }

                if self.is_running {
                    ui.label("🔊 CodeBeats is running - start typing to make music!");
                }
            });

            ui.add_space(10.0);

            // Status Message
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Status:");
                ui.label(&self.status_message);
            });

            ui.add_space(20.0);

            // Help Section
            ui.separator();
            ui.collapsing("ℹ Help & Information", |ui| {
                ui.label("CodeBeats transforms your typing into music!");
                ui.add_space(5.0);

                ui.label("🎹 How it works:");
                ui.label("• Each key you type produces a musical note");
                ui.label("• Different languages have different musical scales");
                ui.label("• Various waveforms change the sound character");
                ui.add_space(5.0);

                ui.label("🎵 Waveforms explained:");
                ui.label("• natural - Piano-like with harmonics");
                ui.label("• electronic - Clean sine wave");
                ui.label("• cyberpunk - Analog synthesizer atmosphere");
                ui.label("• harmonic - Mathematical overtone series");
                ui.label("• triangle/saw/square - Classic electronic sounds");
                ui.label("• fart - Real fart audio sample (for the adventurous!)");
                ui.add_space(5.0);

                ui.label("🥚 Easter Egg:");
                ui.label("Type 'oppokokoppokosuttenten' for a surprise sound effect!");
                ui.add_space(5.0);

                ui.label("💡 Tips:");
                ui.label("• Use lower volume for background music while coding");
                ui.label("• Try different language configs for various musical scales");
                ui.label("• Enable verbose logging to see which keys trigger notes");
                ui.label("• Press Ctrl+C in the terminal to stop CodeBeats");
            });
        });

        // Request repaint to keep checking process status
        ctx.request_repaint();
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.stop_codebeats();
    }
}

/// Run the GUI application
pub fn run_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "CodeBeats Configuration",
        options,
        Box::new(|_cc| Box::new(CodeBeatsGui::default())),
    )
}
