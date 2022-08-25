use open;
mod config;
use config::build_config;

fn main() {
    let mut settings = build_config();
    let url = settings.remove("url");
    match url {
        Some(url) => {
            let url = String::from(url);
            open::that(url).unwrap();
        }
        None => println!("Could not retrieve url from settings")
    }
}
