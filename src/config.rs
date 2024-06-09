
pub trait ProjectSettings<T> {

    fn new() -> T where T: serde::de::DeserializeOwned {

        let path: &'static str = if cfg!(debug_assertions) {
            "dev_settings.toml"
        } else {
            "prod_settings.toml"
        };

        match std::fs::read_to_string(path) {
            Ok(contents) => toml::from_str(&contents).unwrap(),
            Err(_) => panic!("Could not read config file"),
        }

    }

}