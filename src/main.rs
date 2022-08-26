use open;
mod config;
use config::build_config;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    command: String,
}

fn main() {
    let args = Cli::parse();
    let mut config = build_config();
    let command = args.command;
    
    match command.as_str() {
        "open" => {
            let url = config.remove("url");
            match url {
                Some(url) => {
                    let url = String::from(url);
                    open::that(url).unwrap();
                }
                None => println!("Could not retrieve url from config")
            }
        }
        false_command => println!("Command {:?} not found", false_command)
    }
}
