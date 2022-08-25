use open;
use config::Config;
use std::collections::HashMap;

fn get_settings(file_path: &str) -> HashMap<String, String> {
    let settings = Config::builder()
    // Add in `./Settings.toml`
    .add_source(config::File::with_name(file_path))
    .build()
    .unwrap();
    let serialized_settings: HashMap<String, String> = settings
    .try_deserialize::<HashMap<String, String>>()
    .unwrap();
    serialized_settings
}

fn build_settings() -> HashMap<String, String> {
    let default_settings = get_settings("./Default_Settings");
    let mut custom_settings = get_settings("./Custom_Settings");

    let final_settings = default_settings.into_iter().map(|(key, value)| {
        if custom_settings.contains_key(&key) {
            return custom_settings.remove_entry(&key).unwrap();
        } else {
            return (key, value);
        }
    }).collect::<HashMap<String, String>>();

    final_settings
}

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
