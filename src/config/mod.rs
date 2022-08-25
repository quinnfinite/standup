use config_crate::Config;
use std::collections::HashMap;

fn get_config(file_path: &str) -> HashMap<String, String> {
    let settings = Config::builder()
    // Add in `./Settings.toml`
    .add_source(config_crate::File::with_name(file_path))
    .build()
    .unwrap();
    let serialized_settings: HashMap<String, String> = settings
    .try_deserialize::<HashMap<String, String>>()
    .unwrap();
    serialized_settings
}

pub fn build_config() -> HashMap<String, String> {
    let default_settings = get_config("./Default_Config");
    let mut custom_settings = get_config("./Custom_Config");

    let final_settings = default_settings.into_iter().map(|(key, value)| {
        if custom_settings.contains_key(&key) {
            return custom_settings.remove_entry(&key).unwrap();
        } else {
            return (key, value);
        }
    }).collect::<HashMap<String, String>>();

    final_settings
}