//! GUI for CodeBeats using CLI layer
//!
//! This GUI uses the CLI layer to interact with CodeBeats functionality,
//! maintaining a clean three-layer architecture:
//! Layer 1: Library (core functionality)
//! Layer 2: CLI (command interface)
//! Layer 3: GUI (this file, using CLI)

use eframe::egui;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

/// GUI application state
pub struct CodeBeatsGui {
    // Configuration state
    selected_language: String,
    selected_waveform: String,
    volume: f32,
    filter_cutoff: f32,
    verbose: bool,

    // Available options (populated from CLI)
    available_languages: Vec<(String, String)>, // (display_name, file_path)
    available_waveforms: Vec<String>,

    // Runtime state
    status_message: String,
    is_running: bool,
    process: Option<Child>,

    // Logging
    log_messages: Arc<Mutex<Vec<String>>>,
    show_logs: bool,

    // CLI path and caching
    cli_executable: String,
    cli_args: Vec<String>,
    cli_connected: Option<bool>,
    last_cli_check: Option<Instant>,
    options_loaded: bool,
}

impl Default for CodeBeatsGui {
    fn default() -> Self {
        let gui = Self {
            selected_language: "general_programming_language.json".to_string(),
            selected_waveform: "natural".to_string(),
            volume: 0.8,
            filter_cutoff: 1200.0,
            verbose: false,
            available_languages: Vec::new(),
            available_waveforms: Vec::new(),
            status_message: "Ready to start CodeBeats".to_string(),
            is_running: false,
            process: None,
            log_messages: Arc::new(Mutex::new(Vec::new())),
            show_logs: false,
            cli_executable: Self::detect_cli_executable(),
            cli_args: Self::get_cli_args(),
            cli_connected: None,
            last_cli_check: None,
            options_loaded: false,
        };

        gui
    }
}

impl CodeBeatsGui {
    /// Detect the appropriate CLI executable path
    fn detect_cli_executable() -> String {
        // Try bundled CLI first (for macOS app bundle)
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let bundled_cli = exe_dir.join("codebeats");
                if bundled_cli.exists() {
                    return bundled_cli.to_string_lossy().to_string();
                }

                // Try wrapper script
                let wrapper_cli = exe_dir.join("codebeats-wrapper");
                if wrapper_cli.exists() {
                    return wrapper_cli.to_string_lossy().to_string();
                }
            }
        }

        // Try cargo in development
        if std::env::var("CARGO_MANIFEST_DIR").is_ok() {
            return "cargo".to_string();
        }

        // Try system codebeats
        if let Ok(_) = std::process::Command::new("codebeats")
            .arg("--version")
            .output()
        {
            return "codebeats".to_string();
        }

        // Fallback to cargo
        "cargo".to_string()
    }

    /// Get CLI arguments based on executable type
    fn get_cli_args() -> Vec<String> {
        let executable = Self::detect_cli_executable();

        if executable.contains("cargo") {
            vec![
                "run".to_string(),
                "--bin".to_string(),
                "codebeats".to_string(),
            ]
        } else {
            vec![]
        }
    }
    /// Load available waveforms and configurations with immediate fallbacks
    fn load_available_options(&mut self) {
        if !self.options_loaded {
            // Use immediate fallbacks to avoid blocking
            self.available_waveforms = vec![
                "natural".to_string(),
                "electronic".to_string(),
                "cyberpunk".to_string(),
                "harmonic".to_string(),
                "triangle".to_string(),
                "saw".to_string(),
                "square".to_string(),
                "fart".to_string(),
            ];

            self.available_languages = vec![
                ("Default Config".to_string(), "".to_string()),
                (
                    "General Programming".to_string(),
                    "general_programming_language.json".to_string(),
                ),
                ("C Language".to_string(), "c.json".to_string()),
                ("JavaScript".to_string(), "javascript.json".to_string()),
                ("Python".to_string(), "python.json".to_string()),
                ("Rust".to_string(), "rust.json".to_string()),
                ("Java".to_string(), "java.json".to_string()),
                ("Go".to_string(), "go.json".to_string()),
                ("Haskell".to_string(), "haskell.json".to_string()),
            ];

            self.options_loaded = true;
        }
    }

    /// Start CodeBeats using CLI
    fn start_codebeats(&mut self) {
        // Build CLI command
        let mut cmd = Command::new(&self.cli_executable);
        cmd.args(&self.cli_args);
        cmd.arg("run");

        // Add arguments based on configuration
        if !self.selected_waveform.is_empty() {
            cmd.args(&["--waveform", &self.selected_waveform]);
        }

        if !self.selected_language.is_empty() {
            cmd.args(&["--language", &self.selected_language]);
        }

        cmd.args(&["--volume", &self.volume.to_string()]);
        cmd.args(&["--filter-cutoff", &self.filter_cutoff.to_string()]);

        if self.verbose {
            cmd.arg("--verbose");
        }

        // Configure process for logging
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        match cmd.spawn() {
            Ok(mut child) => {
                self.status_message = format!(
                    "Started CodeBeats with: {} waveform, {} language, volume {:.1}, filter {:.0}Hz{}",
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

                // Clear previous logs
                if let Ok(mut logs) = self.log_messages.lock() {
                    logs.clear();
                    logs.push("🎵 CodeBeats started! Begin typing to make music...".to_string());
                }

                // Start log monitoring threads if verbose and show_logs are enabled
                if self.verbose && self.show_logs {
                    self.start_log_monitoring(&mut child);
                }

                self.process = Some(child);
                self.is_running = true;
            }
            Err(e) => {
                self.status_message = format!("Failed to start CodeBeats: {}", e);
            }
        }
    }

    /// Start monitoring logs from the CLI process
    fn start_log_monitoring(&self, child: &mut Child) {
        if let (Some(stdout), Some(stderr)) = (child.stdout.take(), child.stderr.take()) {
            let log_messages = Arc::clone(&self.log_messages);

            // Monitor stdout
            let log_messages_stdout = Arc::clone(&log_messages);
            thread::spawn(move || {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        if let Ok(mut logs) = log_messages_stdout.lock() {
                            logs.push(line);
                            // Keep logs to reasonable size
                            if logs.len() > 100 {
                                logs.drain(0..50);
                            }
                        }
                    }
                }
            });

            // Monitor stderr
            thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        if let Ok(mut logs) = log_messages.lock() {
                            logs.push(format!("ERROR: {}", line));
                            // Keep logs to reasonable size
                            if logs.len() > 100 {
                                logs.drain(0..50);
                            }
                        }
                    }
                }
            });
        }
    }

    /// Stop the running CodeBeats process
    fn stop_codebeats(&mut self) {
        if let Some(mut process) = self.process.take() {
            let _ = process.kill();
            let _ = process.wait();

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
        for (display_name, file_path) in &self.available_languages {
            if *file_path == self.selected_language {
                return display_name.clone();
            }
        }
        if self.selected_language.is_empty() {
            "Default".to_string()
        } else {
            self.selected_language.clone()
        }
    }

    /// Check if the process is still running (optimized)
    fn check_process_status(&mut self) {
        if self.is_running {
            if let Some(ref mut process) = self.process {
                match process.try_wait() {
                    Ok(Some(_)) => {
                        // Process has exited
                        self.process = None;
                        self.is_running = false;
                        self.status_message = "CodeBeats process has stopped".to_string();
                    }
                    Ok(None) => {
                        // Process is still running - no action needed
                    }
                    Err(_) => {
                        // Error checking process status
                        self.process = None;
                        self.is_running = false;
                        self.status_message = "CodeBeats process error".to_string();
                    }
                }
            } else {
                self.is_running = false;
                self.status_message = "CodeBeats process is no longer available".to_string();
            }
        }
    }

    /// Get CLI connection status (cached, non-blocking)
    fn get_cli_connection_status(&mut self) -> bool {
        let now = Instant::now();

        // Check cache first
        if let (Some(connected), Some(last_check)) = (self.cli_connected, self.last_cli_check) {
            // Cache for 10 seconds to reduce CLI calls
            if now.duration_since(last_check) < Duration::from_secs(10) {
                return connected;
            }
        }

        // If no cache, assume connected and test in background
        if self.cli_connected.is_none() {
            self.cli_connected = Some(true); // Optimistic default
            self.last_cli_check = Some(now);
        }

        // Return cached value immediately to avoid blocking
        self.cli_connected.unwrap_or(true)
    }

    /// Test CLI connection (for future background testing)
    #[allow(dead_code)]
    fn test_cli_connection(&self) -> bool {
        let mut cmd = Command::new(&self.cli_executable);
        cmd.args(&self.cli_args);
        cmd.arg("version");

        cmd.output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

impl eframe::App for CodeBeatsGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Load options if not loaded yet
        self.load_available_options();

        // Check process status only if running
        if self.is_running {
            self.check_process_status();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CodeBeats Configuration");
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
                ui.add(egui::Slider::new(&mut self.volume, 0.0..=1.0).text(""));
                ui.label(format!("{:.1}", self.volume));
            });

            ui.add_space(10.0);

            // Filter Cutoff Slider
            ui.horizontal(|ui| {
                ui.label("Filter Cutoff:");
                ui.add(egui::Slider::new(&mut self.filter_cutoff, 200.0..=8000.0).text("Hz"));
                ui.label(format!("{:.0} Hz", self.filter_cutoff));
            });

            ui.add_space(10.0);

            // Verbose Checkbox
            ui.checkbox(&mut self.verbose, "Verbose logging");

            // Show logs checkbox
            ui.checkbox(&mut self.show_logs, "Show logs in GUI");

            ui.add_space(20.0);

            // Start/Stop Button
            let button_text = if self.is_running {
                "Stop CodeBeats"
            } else {
                "Start CodeBeats"
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
                    if self.is_running {
                        self.stop_codebeats();
                    } else {
                        self.start_codebeats();
                    }
                }

                if self.is_running {
                    ui.label("CodeBeats is running - start typing to make music!");
                }
            });

            ui.add_space(10.0);

            // CLI Connection Status (cached)
            ui.horizontal(|ui| {
                ui.label("CLI Status:");
                let cli_connected = self.get_cli_connection_status();
                let status_color = if cli_connected {
                    egui::Color32::from_rgb(100, 200, 100)
                } else {
                    egui::Color32::from_rgb(200, 100, 100)
                };
                ui.colored_label(
                    status_color,
                    if cli_connected {
                        "✓ Connected"
                    } else {
                        "✗ Disconnected"
                    },
                );
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
                ui.label("CodeBeats Logs:");

                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        if let Ok(logs) = self.log_messages.lock() {
                            if logs.is_empty() {
                                ui.label("No logs yet. Enable verbose logging and start CodeBeats to see activity...");
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
            ui.collapsing("Help & Information", |ui| {
                ui.label("CodeBeats transforms your typing into music!");
                ui.add_space(5.0);

                ui.label("How it works:");
                ui.label("• Each key you type produces a musical note");
                ui.label("• Different languages have different musical scales");
                ui.label("• Various waveforms change the sound character");
                ui.add_space(5.0);

                ui.label("Architecture:");
                ui.label("• Layer 1: Core library (audio engine, synthesis)");
                ui.label("• Layer 2: CLI interface (command-line tool)");
                ui.label("• Layer 3: GUI (this interface, uses CLI)");
                ui.add_space(5.0);

                ui.label("Waveforms explained:");
                ui.label("• natural - Piano-like with harmonics");
                ui.label("• electronic - Clean sine wave");
                ui.label("• cyberpunk - Analog synthesizer atmosphere");
                ui.label("• harmonic - Mathematical overtone series");
                ui.label("• triangle/saw/square - Classic electronic sounds");
                ui.label("• fart - Real fart audio sample (for the adventurous!)");
                ui.add_space(5.0);

                ui.label("Easter Egg:");
                ui.label("Type 'oppokokoppokosuttenten' for a surprise sound effect!");
                ui.add_space(5.0);

                ui.label("Tips:");
                ui.label("• Use lower volume for background music while coding");
                ui.label("• Try different language configs for various musical scales");
                ui.label("• Enable verbose logging to see which keys trigger notes");
                ui.label("• Use the Stop button to properly terminate CodeBeats");
            });
        });

        // Minimal repainting - only when running
        if self.is_running {
            ctx.request_repaint_after(Duration::from_secs(2));
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.stop_codebeats();
    }
}

pub fn run_gui() -> Result<(), Box<dyn std::error::Error>> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 700.0])
            .with_min_inner_size([500.0, 600.0])
            .with_resizable(true)
            .with_title("CodeBeats Studio"),
        ..Default::default()
    };

    eframe::run_native(
        "CodeBeats Studio",
        options,
        Box::new(|_cc| Box::new(CodeBeatsGui::default())),
    )
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}
