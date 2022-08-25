use open;
mod config;
use config::build_config;

fn main() {
    let mut config = build_config();
    let url = config.remove("url");
    match url {
        Some(url) => {
            let url = String::from(url);
            open::that(url).unwrap();
        }
        None => println!("Could not retrieve url from config")
    }
}
