use open;
use config::Config;
use std::collections::HashMap;

fn get_settings() -> HashMap<String, String> {
    let default_settings = Config::builder()
    // Add in `./Settings.toml`
    .add_source(config::File::with_name("./Default_Settings"))
    .build()
    .unwrap();

    let default_settings: HashMap<String, String> = default_settings
    .try_deserialize::<HashMap<String, String>>()
    .unwrap();

    let custom_settings = Config::builder()
    // Add in `./Custom_Settings.toml`
    .add_source(config::File::with_name("./Custom_Settings"))
    .build()
    .unwrap();

    let mut custom_settings: HashMap<String, String> = custom_settings
    .try_deserialize::<HashMap<String, String>>()
    .unwrap();

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
    let mut settings = get_settings();
    let url = settings.remove("url");
    match url {
        Some(url) => {
            let url = String::from(url);
            open::that(url).unwrap();
        }
        None => println!("Could not retrieve url from settings")
    }
}
