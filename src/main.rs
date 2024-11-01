use eframe::NativeOptions;
use vole_disassembly::DisassemblyMachine;

fn main() {
    let native_options = NativeOptions::default();
    match eframe::run_native(
        "Vole Disassembler v0.1.0",
        native_options,
        Box::new(|cc| Ok(Box::new(DisassemblyMachine::new(cc)))),
    ) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
    println!("Hello, world!");
}
