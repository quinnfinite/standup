#![allow(unused)]
use open;
mod config;
use config::build_config;
use clap::Parser;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use chrono::prelude::*;
use std::thread;
use std::thread::JoinHandle;
use std::io::{self, prelude::*};

#[derive(Parser)]
struct Cli {
    command: String,
}

fn main() {
    let args = Cli::parse();
    let config = build_config();
    let command = args.command;
    
    match command.as_str() {
        "start" => {
            println!("Starting Standup");
            go_to_standup(config);
            keep_track_of_whos_gone();
        }
        "notes" => {
            open_notes();
        }
        false_command => println!("Command {:?} not found", false_command)
    }
}

fn keep_track_of_whos_gone() -> Result<(), Box<dyn Error>>  {
    let has_spoken = Arc::new(Mutex::new(vec![]));
    
    let thread_join_handle = thread::spawn(move || loop {
        println!("\n");
        println!("Who has spoken:");
        for speaker in has_spoken.lock().unwrap().iter() {
            println!("{speaker} \n");
        }
        println!("Who just spoke?");
        let mut name = String::new();
        
        
        match io::stdin().read_line(&mut name) {
            Ok(_) => {
                let lowercase = name.to_lowercase();
                let cleaned_input = lowercase.trim();

                if (cleaned_input == "exit") {
                    println!("Exiting Standup");
                    std::process::exit(1);
                } else {
                    let clone = Arc::clone(&has_spoken);
                    let mut v = clone.lock().unwrap();
                    v.push(cleaned_input.to_string());
                }
            }
            Err(error) => println!("error reading command: {}", error),
        }
    });

    let res = thread_join_handle.join();

    Ok(())
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