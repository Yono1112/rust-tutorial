use std::io::{self, BufRead, Write};

#[derive(Debug)]
enum Command {
    TurnOn,
    TurnOff,
    SetBrightness(u8),
    Unknown(String),
}

impl Command {
    fn from_str(command_str: &str) -> Vec<Command> {
        let mut commands = Vec::new();
        let parts: Vec<&str> = command_str.split(',').collect();

        let mut iter = parts.iter();
        while let Some(part) = iter.next() {
            match *part {
                "TURN_ON" => commands.push(Command::TurnOn),
                "TURN_OFF" => commands.push(Command::TurnOff),
                "SET_BRIGHTNESS" => {
                    if let Some(value_str) = iter.next() {
                        if let Ok(value) = value_str.parse::<u8>() {
                            commands.push(Command::SetBrightness(value));
                        } else {
                            commands.push(Command::Unknown(value_str.to_string()));
                        }
                    } else {
                        commands.push(Command::Unknown("Missing value for SET_BRIGHTNESS".to_string()));
                    }
                }
                _ => commands.push(Command::Unknown(part.to_string())),
            }
        }
        commands
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
        Command::Unknown(cmd) => {
            println!("Unknown command received: {}", cmd);
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

        let commands = Command::from_str(&buffer);
        for command in commands {
            handle_command(command);
        }
    }
}
