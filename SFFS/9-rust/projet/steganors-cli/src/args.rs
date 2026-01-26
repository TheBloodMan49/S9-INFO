use crate::Result;
use steganors::commands::{decode, encode, remove};
use steganors::error::Error::{IncorrectNbArg, NoCommand, UnknownCommand};

pub fn run(args: &[String]) -> Result<()> {
    if args.len() < 2 {
        return Err(Box::new(NoCommand));
    }

    let command = args[1].as_str();

    match command {
        "encode" => {
            if args.len() < 5 {
                return Err(Box::new(IncorrectNbArg(args.len(), 5)));
            }

            let path = &args[2];
            let key = &args[3];
            let message = &args[4];

            encode(path, key, message)
        }
        "decode" => {
            if args.len() < 4 {
                return Err(Box::new(IncorrectNbArg(args.len(), 4)));
            }

            let path = &args[2];
            let key = &args[3];

            decode(path, key)
        }
        "remove" => {
            if args.len() < 4 {
                return Err(Box::new(IncorrectNbArg(args.len(), 4)));
            }

            let path = &args[2];
            let key = &args[3];

            remove(path, key)
        }
        "help" => {
            println!(
                "Usage :\n\
                        steganors encode <PATH> <KEY> <MESSAGE>\n\
                        steganors decode <PATH> <KEY>\n\
                        steganors remove <PATH> <KEY>\n\
                        steganors help"
            );
            Ok(())
        }
        autre => Err(Box::new(UnknownCommand(autre.to_string()))),
    }
}
