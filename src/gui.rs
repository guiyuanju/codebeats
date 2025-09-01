//! GUI module for CodeBeats
//!
//! Provides a cross-platform graphical interface for configuring and launching
//! the CodeBeats command-line application with different parameters.

use eframe::egui;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

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

    // Log display
    log_messages: Arc<Mutex<Vec<String>>>,
    show_logs: bool,
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
            log_messages: Arc::new(Mutex::new(Vec::new())),
            show_logs: false,
        }
    }
}

impl CodeBeatsGui {
    /// Discover available language configuration files
    fn discover_language_configs() -> Vec<(String, String)> {
        let mut configs = Vec::new();

        // Add default option
        configs.push(("Default (Built-in)".to_string(), "".to_string()));

        // Find language_configs directory relative to executable
        let config_dir = Self::find_config_directory().unwrap_or_else(|| "language_configs".into());

        // Scan language_configs directory
        if let Ok(entries) = std::fs::read_dir(&config_dir) {
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

    /// Find the language_configs directory relative to the GUI executable
    fn find_config_directory() -> Option<std::path::PathBuf> {
        // Get the directory where the GUI executable is located
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let config_path = exe_dir.join("language_configs");
                if config_path.exists() {
                    return Some(config_path);
                }
            }
        }

        // Fallback: look in current working directory
        let cwd_path = std::path::Path::new("language_configs");
        if cwd_path.exists() {
            Some(cwd_path.to_path_buf())
        } else {
            None
        }
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

        // Find the CLI binary relative to the GUI executable
        let cli_binary_path = self.find_cli_binary();
        if cli_binary_path.is_none() {
            self.status_message = "Error: CodeBeats CLI binary not found. Make sure 'codebeats' or 'codebeats.exe' is in the same directory.".to_string();
            return;
        }

        let mut cmd = Command::new(cli_binary_path.unwrap());

        // Add language config if not default
        if !self.selected_language.is_empty() {
            let config_dir =
                Self::find_config_directory().unwrap_or_else(|| "language_configs".into());
            let config_path = config_dir.join(&self.selected_language);
            cmd.arg("-l").arg(config_path);
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

        // Configure process to capture output
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        self.status_message = format!(
            "Starting CodeBeats with: {} waveform, {} language, volume {:.1}, filter {:.0}Hz{}",
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
            Ok(mut child) => {
                // Clear previous logs
                if let Ok(mut logs) = self.log_messages.lock() {
                    logs.clear();
                }

                // Capture stdout
                if let Some(stdout) = child.stdout.take() {
                    let log_messages = Arc::clone(&self.log_messages);
                    thread::spawn(move || {
                        let reader = BufReader::new(stdout);
                        for line in reader.lines() {
                            if let Ok(line) = line {
                                if let Ok(mut logs) = log_messages.lock() {
                                    logs.push(format!("🎵 {}", line));
                                    // Keep only last 100 lines
                                    if logs.len() > 100 {
                                        logs.remove(0);
                                    }
                                }
                            }
                        }
                    });
                }

                // Capture stderr
                if let Some(stderr) = child.stderr.take() {
                    let log_messages = Arc::clone(&self.log_messages);
                    thread::spawn(move || {
                        let reader = BufReader::new(stderr);
                        for line in reader.lines() {
                            if let Ok(line) = line {
                                if let Ok(mut logs) = log_messages.lock() {
                                    logs.push(format!("⚠️ {}", line));
                                    // Keep only last 100 lines
                                    if logs.len() > 100 {
                                        logs.remove(0);
                                    }
                                }
                            }
                        }
                    });
                }

                self.current_process = Some(child);
                self.is_running = true;
                self.status_message
                    .push_str(" - Running! Press Stop to terminate.");

                // Add initial log message
                if let Ok(mut logs) = self.log_messages.lock() {
                    logs.push("🎵 CodeBeats started! Begin typing to make music...".to_string());
                }
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

            // Add stop message to logs
            if let Ok(mut logs) = self.log_messages.lock() {
                logs.push("🔇 CodeBeats stopped".to_string());
            }
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

    /// Find the CLI binary relative to the GUI executable
    fn find_cli_binary(&self) -> Option<std::path::PathBuf> {
        // Get the directory where the GUI executable is located
        let exe_path = std::env::current_exe().ok()?;
        let exe_dir = exe_path.parent()?;

        // Look for the CLI binary in the same directory
        let binary_name = if cfg!(windows) {
            "codebeats.exe"
        } else {
            "codebeats"
        };
        let cli_path = exe_dir.join(binary_name);

        if cli_path.exists() {
            Some(cli_path)
        } else {
            // Fallback: look in current working directory
            let cwd_path = std::path::Path::new(binary_name);
            if cwd_path.exists() {
                Some(cwd_path.to_path_buf())
            } else {
                None
            }
        }
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
                "Verbose logging (show detailed key information)",
            );

            // Show logs checkbox
            ui.checkbox(&mut self.show_logs, "Show logs in GUI");

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

            // Log Display
            if self.show_logs {
                ui.add_space(10.0);
                ui.separator();
                ui.label("🎵 CodeBeats Logs:");

                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        if let Ok(logs) = self.log_messages.lock() {
                            if logs.is_empty() {
                                ui.label("No logs yet. Start CodeBeats to see activity...");
                            } else {
                                for log_message in logs.iter() {
                                    ui.label(log_message);
                                }
                            }
                        }
                    });
            }

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
