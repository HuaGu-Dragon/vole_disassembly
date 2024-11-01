#[derive(Default)]
pub struct DisassemblyMachine {
    // --snip--
}

impl DisassemblyMachine {
    // --snip--
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            Self::default()
        }
    }
}

impl eframe::App for DisassemblyMachine {
    // --snip--
    fn update(&mut self, _ctx: &eframe::Context, _state: &mut State) {
        // --snip--
    }
    
}
