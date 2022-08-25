use open;
use config::Config;
use std::collections::HashMap;

fn get_settings() -> HashMap<String, String> {
    let settings = Config::builder()
    // Add in `./Settings.toml`
    .add_source(config::File::with_name("./Default_Settings"))
    // Add in settings from the environment (with a prefix of APP)
    // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
    .add_source(config::Environment::with_prefix("DEFAULT"))
    .build()
    .unwrap();

    settings
    .try_deserialize::<HashMap<String, String>>()
    .unwrap()
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
