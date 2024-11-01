fn main() {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Vole Disassembler v0.1.0",
        native_options,
        Box::new(|_app| {
            let state = State::default();
            let ui = Ui::new(state);
            ui
        }),
    );
    println!("Hello, world!");
}
