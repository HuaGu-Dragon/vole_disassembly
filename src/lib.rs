mod disassembly_machine;

pub use disassembly_machine::DisassemblyMachine;
use disassembly_machine::DisassemblyMachineState;

pub fn solve(machine: &mut DisassemblyMachine) {
    let counter = machine.get_counter();
    let vole_code = machine.get_vole_code(counter);
    let next_vole_code = machine.get_vole_code(counter + 1);
    match vole_code / 16 {
        1 => machine.cpu[(vole_code as usize) % 16] = machine.get_vole_code(next_vole_code),
        2 => machine.cpu[(vole_code as usize) % 16] = next_vole_code,
        3 => {
            machine.memory[next_vole_code as usize] =
                format!("0x{:02X}", machine.cpu[(vole_code as usize) % 16]);
        }
        4 => {
            if (vole_code as usize) % 16 == 0 {
                machine.cpu[(next_vole_code as usize) % 16] =
                    machine.cpu[(next_vole_code as usize) / 16];
            } else {
                machine.log_error(
                    format!(
                        "Invalid vole code at 0x{:02X}: opcode 0x4 must be followed by 0",
                        machine.get_counter()
                    )
                    .as_str(),
                );
                machine.update_state(DisassemblyMachineState::Stopped);
                return;
            }
        }
        5 => {
            match machine.cpu[(next_vole_code as usize) / 16]
                .checked_add(machine.cpu[(next_vole_code as usize) % 16])
            {
                Some(v) => machine.cpu[(vole_code as usize) % 16] = v,
                None => {
                    machine.log_error(
                        format!(
                            "Invalid vole code at 0x{:02X}: overflow in opcode 0x5",
                            machine.get_counter()
                        )
                        .as_str(),
                    );
                    machine.update_state(DisassemblyMachineState::Stopped);
                    return;
                }
            }
        }
        6 => {
            machine.log_error("Developing opcode 0x6");
            machine.update_state(DisassemblyMachineState::Stopped);
            return;
        }
        7 => {
            machine.cpu[(vole_code as usize) % 16] = machine.cpu[(next_vole_code as usize) % 16]
                | machine.cpu[(next_vole_code as usize) / 16];
        }
        8 => {
            machine.cpu[(vole_code as usize) % 16] = machine.cpu[(next_vole_code as usize) % 16]
                & machine.cpu[(next_vole_code as usize) / 16];
        }
        9 => {
            machine.cpu[(vole_code as usize) % 16] = machine.cpu[(next_vole_code as usize) % 16]
                ^ machine.cpu[(next_vole_code as usize) / 16];
        }
        10 => {
            if (next_vole_code) / 16 == 0 {
                machine.cpu[(vole_code as usize) % 16] =
                    rotate_right(machine.cpu[(vole_code as usize) % 16], next_vole_code % 16);
            } else {
                machine.log_error(
                    format!(
                        "Invalid vole code at 0x{:02X}: opcode 0x{:02X} must be followed by 0",
                        machine.get_counter(),
                        vole_code
                    )
                    .as_str(),
                );
                machine.update_state(DisassemblyMachineState::Stopped);
                return;
            }
        }
        11 => {
            if machine.cpu[(vole_code as usize) % 16] == machine.cpu[0] {
                machine.set_counter(next_vole_code);
                machine.timer_dec();
                return;
            }
        }
        12 => {
            machine.update_state(DisassemblyMachineState::Stopped);
            if next_vole_code != 0 && vole_code % 16 != 0 {
                machine.log_error(
                    format!(
                        "Invalid vole code at 0x{:02X}: opcode 0xC must be followed by 0",
                        machine.get_counter()
                    )
                    .as_str(),
                );
                return;
            }
        }
        _ => {
            machine.log_error(
                format!(
                    "Invalid vole code at 0x{:02X}: opcode 0x{:02X} not recognized",
                    machine.get_counter(),
                    vole_code
                )
                .as_str(),
            );
            machine.update_state(DisassemblyMachineState::Stopped);
            return;
        }
    };
    machine.set_counter(counter + 2);
    machine.timer_dec();
}

fn rotate_right(value: u8, shift: u8) -> u8 {
    let shift = shift % 8;
    (value >> shift) | (value << (8 - shift))
}
