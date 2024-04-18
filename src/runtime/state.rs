use std::{collections::HashMap, process::exit};

use crate::parser::{action::Action, condition::Condition, direction::Direction};

pub struct AppState {
    pub tape: Vec<String>,
    pub state: String,
    pub position: usize,
    pub default: String,
}

impl AppState {
    pub fn print_tape(&self) {
        for (index, symbol) in self.tape.iter().enumerate() {
            if index == self.position {
                print!("\x1b[31m{}\x1b[0m ", symbol);
            } else {
                print!("{} ", symbol);
            }
        }
        println!("State: {}", self.state);
    }

    pub fn act(&mut self, machine: &HashMap<Condition, Action>) {
        let symbol = self.tape[self.position].clone();
        let action = machine.get(&Condition {
            state: self.state.clone(),
            symbol: symbol.clone(),
        });
        if let Some(action) = action {
            self.state = action.change.state.clone();
            self.tape[self.position] = action.change.symbol.clone();
            match action.move_direction {
                Direction::Left => {
                    if self.position > 0 {
                        self.position -= 1;
                    } else {
                        self.tape.insert(0, self.default.clone());
                    }
                }
                Direction::Right => {
                    self.position += 1;
                    if self.position == self.tape.len() {
                        self.tape.push(self.default.clone());
                    }
                }
            }
        } else {
            if self.state == "Halt" {
                println!("Halted");
                exit(0);
            }
            panic!(
                "No action found for state {} and symbol {}",
                &self.state, symbol
            );
        }
    }
}
