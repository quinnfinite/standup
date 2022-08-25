use open;
mod settings;
use settings::build_settings;

fn main() {
    let mut settings = build_settings();
    let url = settings.remove("url");
    match url {
        Some(url) => {
            let url = String::from(url);
            open::that(url).unwrap();
        }
        None => println!("Could not retrieve url from settings")
    }
}
