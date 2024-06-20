use std::io::{self, BufRead, Write};
use std::str::from_utf8;

#[derive(Debug)]
enum Command {
    TurnOn,
    TurnOff,
    SetBrightness(u8),
    Unknown,
}

impl Command {
    fn from_str(command_str: &str) -> Command {
        let parts: Vec<&str> = command_str.split(',').collect();
        match parts[0] {
            "TURN_ON" => Command::TurnOn,
            "TURN_OFF" => Command::TurnOff,
            "SET_BRIGHTNESS" => {
                if parts.len() > 1 {
                    if let Ok(value) = parts[1].parse::<u8>() {
                        return Command::SetBrightness(value);
                    }
                }
                Command::Unknown
            }
            _ => Command::Unknown,
        }
    }
}

fn handle_command(command: Command) {
    match command {
        Command::TurnOn => {
            println!("Turning on the LED");
        }
        Command::TurnOff => {
            println!("Turning off the LED");
        }
        Command::SetBrightness(level) => {
            println!("Setting brightness to {}", level);
        }
        Command::Unknown => {
            println!("Unknown command received");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    loop {
        print!("Enter command: ");
        io::stdout().flush().unwrap();
        
        buffer.clear();
        stdin.lock().read_line(&mut buffer).unwrap();
        
        if let Some('\n') = buffer.chars().next_back() {
            buffer.pop();
        }
        if let Some('\r') = buffer.chars().next_back() {
            buffer.pop();
        }

        let command = Command::from_str(&buffer);
        handle_command(command);
    }
}
