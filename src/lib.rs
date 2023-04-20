use command_list::CommandList;
use commands::Commands;
use std::{collections::HashMap, io::Write};

mod command_list;
#[allow(unused)]
mod commands;

#[derive(Debug)]
struct Command<'a> {
    name: &'a str,
    args: Vec<&'a str>,
}

impl<'a> Command<'a> {
    fn new(name: &'a str, len: usize) -> Self {
        Self {
            name,
            args: Vec::with_capacity(len),
        }
    }

    fn process(self, command_list: HashMap<&'a str, Vec<&'a str>>) {
        let mut command = Commands::UnInit;
        match self.name {
            "ls" => command = Commands::Ls,
            "cd" => command = Commands::Cd(self.args.clone()),
            "cargo" => command = Commands::Cargo(self.args.clone()),
            _ => (),
        };

        if command == Commands::UnInit {
            println!("'{}' is a invalid command.", self.name);
            return;
        }

        if let Some(args) = command_list.get(&self.name) {
            if self.name == "cd" || self.name == "cargo" {
            } else {
                for arg in self.args.iter() {
                    if !args.contains(arg) {
                        println!("'{arg}' is a invalid argument.");
                        return;
                    }
                }
            }

            command.run();
        };
    }
}

pub struct Shell {
    io: std::io::Stdin,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            io: std::io::stdin(),
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        loop {
            let mut input = String::new();

            print!(
                "{} > ",
                std::env::current_dir()
                    .unwrap()
                    .into_os_string()
                    .into_string()
                    .unwrap()
            );
            std::io::stdout().flush()?;
            self.io.read_line(&mut input)?;

            let input = input.trim();

            let commands = input.split(" ").filter(|s| *s != "").collect::<Vec<&str>>();

            if commands[0] == "quit" && commands.len() == 1 {
                std::process::exit(0);
            }

            if commands.len() > 0 {
                let mut command = Command::new(commands[0], commands.len());

                for i in 1..commands.len() {
                    command.args.push(commands[i]);
                }

                command.process(CommandList::init());
            }
        }
    }
}
