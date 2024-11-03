#![windows_subsystem = "windows"]
use std::error::Error;

use eframe::NativeOptions;
use vole_disassembly::DisassemblyMachine;

fn main() -> Result<(), Box<dyn Error>> {
    let native_options = NativeOptions {
        centered: true,
        ..Default::default()
    };
    match eframe::run_native(
        "Vole Disassembler v0.1.0",
        native_options,
        Box::new(|cc| Ok(Box::new(DisassemblyMachine::new(cc)))),
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
