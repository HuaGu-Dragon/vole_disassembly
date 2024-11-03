use eframe::egui::{Color32, RichText, TextEdit};

use crate::solve;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisassemblyMachineState {
    Running,
    Stopped,
}

pub struct DisassemblyMachine {
    // --snip--
    pub memory: Vec<String>,
    pub cpu: [u8; 16],
    counter: u8,
    timer: u16,
    log: bool,
    log_update: bool,
    logs: Vec<String>,
    state: DisassemblyMachineState,
}

impl DisassemblyMachine {
    // --snip--
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            memory: vec![String::new(); 255],
            cpu: [0; 16],
            counter: 0,
            timer: 500,
            log: false,
            log_update: false,
            logs: Vec::new(),
            state: DisassemblyMachineState::Stopped,
        }
    }

    pub fn update_state(&mut self, state: DisassemblyMachineState) {
        self.state = state;
    }

    pub fn get_state(&self) -> DisassemblyMachineState {
        self.state
    }

    pub fn reset(&mut self) {
        self.cpu = [0; 16];
        self.timer = 500;
        self.log_update = false;
        self.logs = Vec::new();
    }

    pub fn reset_all(&mut self) {
        self.reset();
        self.memory = vec![String::new(); 255];
        self.counter = 0;
        self.state = DisassemblyMachineState::Stopped;
    }

    pub fn get_counter(&self) -> u8 {
        self.counter
    }

    pub fn set_counter(&mut self, counter: u8) {
        self.counter = counter;
    }

    pub fn timer_dec(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
        }
    }

    pub fn get_vole_code(&mut self, index: u8) -> u8 {
        match u8::from_str_radix(
            &self.memory[index as usize].trim().trim_start_matches("0x"),
            16,
        ) {
            Ok(v) => v,
            Err(_) => {
                self.logs
                    .push(format!("Error: Invalid vole code at 0x{:02X}", index));
                self.log_update = true;
                0
            }
        }
    }

    pub fn log_info(&mut self, info: &str) {
        self.logs.push(format!("Info: {}", info));
        self.log_update = true;
    }

    pub fn log_error(&mut self, error: &str) {
        self.logs.push(format!("Error: {}", error));
        self.log_update = true;
    }

    pub fn log_warning(&mut self, warning: &str) {
        self.logs.push(format!("Warning: {}", warning));
        self.log_update = true;
    }
}

impl eframe::App for DisassemblyMachine {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        eframe::egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Menu", |ui| {
                    ui.menu_button("Theme", |ui| {
                        if ui
                            .button("Light")
                            .on_hover_text("Change to light theme")
                            .clicked()
                        {
                            ui.ctx().set_visuals(eframe::egui::Visuals::light());
                        }
                        if ui
                            .button("Dark")
                            .on_hover_text("Change to dark theme")
                            .clicked()
                        {
                            ui.ctx().set_visuals(eframe::egui::Visuals::dark());
                        }
                    });
                    ui.button("Reset")
                        .on_hover_text("Reset the disassembler")
                        .clicked()
                        .then(|| self.reset());
                    ui.button("Reset All")
                        .on_hover_text("Reset the disassembler")
                        .clicked()
                        .then(|| self.reset_all());
                    ui.separator();
                    ui.menu_button("About", |ui| {
                        ui.label("Version: 0.1.0");
                        ui.label("Author: HuaGu_Dragon");
                        ui.label("License: MIT");
                        ui.add(eframe::egui::Hyperlink::from_label_and_url(
                            "GitHub Repository",
                            "https://github.com/HuaGu-Dragon/vole_disassembly",
                        ));
                    });
                });
                if ui
                    .button("Run")
                    .on_hover_text("Run the disassembler")
                    .clicked()
                {
                    // Run the disassembler
                    self.reset();
                    self.update_state(DisassemblyMachineState::Running);
                    self.log_info(
                        format!("Disassembler is running from {:02X}", self.counter).as_str(),
                    );
                }
                if ui
                    .button("Debug")
                    .on_hover_text("Using Debug Mode")
                    .clicked()
                {
                    // Undo
                    solve(self);
                }
            });
        });
        if let DisassemblyMachineState::Running = self.state {
            solve(self);
        }
        eframe::egui::SidePanel::right("right").show(ctx, |ui| {
            ui.heading("Disassembly Machine");
            ui.label("Welcome to Vole Disassembler!");
            ui.separator();
            ui.label("This is a disassembler for Vole Code.");
            ui.add(eframe::egui::Hyperlink::from_label_and_url(
                "By HuaGu_Dragon",
                "https://space.bilibili.com/313037750",
            ));
            ui.separator();
            ui.label("Please input vole code to disassemble.");
            eframe::egui::CollapsingHeader::new("Registers").show(ui, |ui| {
                for i in 0..8 {
                    ui.horizontal(|ui| {
                        ui.label(format!("R{:02}:", i));
                        if self.cpu[i] == 0 {
                            ui.label("0x00");
                        } else {
                            ui.label(
                                RichText::new(format!("0x{:02X}", self.cpu[i]))
                                    .color(Color32::from_rgb(183, 232, 189)),
                            );
                        }
                        ui.add_space(10.0);
                        ui.label(format!("R{:02}:", i + 8));
                        if self.cpu[i + 8] == 0 {
                            ui.label("0x00");
                        } else {
                            ui.label(
                                RichText::new(format!("0x{:02X}", self.cpu[i + 8]))
                                    .color(Color32::from_rgb(183, 232, 189)),
                            );
                        }
                    });
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Timer:");
                    ui.add_space(14.0);
                    ui.add(
                        eframe::egui::DragValue::new(&mut self.timer)
                            .speed(1)
                            .range(0..=1500),
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Counter:");
                    ui.add(
                        eframe::egui::DragValue::new(&mut self.counter)
                            .speed(1)
                            .range(0..=255),
                    );
                });
            });
        });
        eframe::egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Powered by Egui");
                ui.with_layout(
                    eframe::egui::Layout::right_to_left(eframe::egui::Align::RIGHT),
                    |ui| {
                        ui.checkbox(&mut self.log, "Log");
                    },
                );
            });
        });
        if self.log {
            eframe::egui::TopBottomPanel::bottom("log_panel").show(ctx, |ui| {
                ui.label("Logs:");
                ui.separator();
                eframe::egui::ScrollArea::vertical().show_rows(
                    ui,
                    17.,
                    self.logs.len(),
                    |ui, reg| {
                        ui.set_width(ui.available_width());
                        for log in reg {
                            ui.label(&self.logs[log]);
                        }
                        if self.log_update {
                            ui.scroll_to_cursor(Some(eframe::egui::Align::BOTTOM));
                            self.log_update = false;
                        }
                    },
                );
            });
        }
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            eframe::egui::ScrollArea::vertical().show_rows(ui, 17., 255, |ui, reg| {
                for i in reg {
                    ui.horizontal(|ui| {
                        if i == self.counter as usize {
                            ui.label(
                                RichText::new(format!("0x{:08X}:", i))
                                    .color(Color32::from_rgb(183, 232, 189)),
                            );
                        } else {
                            ui.label(format!("0x{:08X}:", i));
                        }
                        ui.text_edit_singleline(&mut self.memory[i])
                            .on_hover_text("Please input vole code here");
                    });
                }
            });
        });
    }
    // --snip--
}
