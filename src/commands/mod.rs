mod cargo;
mod cd;
mod ls;

#[derive(PartialEq)]
pub enum Commands<'a> {
    UnInit,
    Ls,
    Cd(Vec<&'a str>),
    Cargo(Vec<&'a str>),
}

impl<'a> Commands<'a> {
    pub fn run(self) {
        match self {
            Commands::UnInit => (),
            Commands::Ls => ls::Ls::run(),
            Commands::Cd(args) => {
                if args.len() == 1 {
                    cd::Cd::run(args[0]);
                }
            }
            Commands::Cargo(args) => {
                cargo::Cargo::run(args);
            }
        }
    }
}
