use open;
mod config;
use config::build_config;
use clap::Parser;
use std::collections::HashMap;
use chrono::prelude::*;

#[derive(Parser)]
struct Cli {
    command: String,
}

fn main() {
    let args = Cli::parse();
    let config = build_config();
    let command = args.command;
    
    match command.as_str() {
        "go" => {
            go_to_standup(config);
        }
        "notes" => {
            open_notes();
        }
        false_command => println!("Command {:?} not found", false_command)
    }
}

fn open_notes() {
    let local_date = Local::now().format("%Y-%m-%d").to_string();
    let file_name = local_date + ".txt";
    println!("file_name: {:?}", file_name);
    std::process::Command::new("vim")
        .arg(file_name)
        .spawn()
        .expect("Error: Failed to run editor")
        .wait()
        .expect("Error: Editor returned a non-zero status");
}

fn go_to_standup(mut config: HashMap<String, String>) {
    let url = config.remove("url");
    match url {
        Some(url) => {
            let url = String::from(url);
            open::that(url).unwrap();
        }
        None => println!("Could not retrieve url from config")
    }
}