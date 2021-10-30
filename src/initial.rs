use std::collections::HashMap;

pub fn initial_config() {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("settings")).unwrap()
        .merge(config::Environment::with_prefix("APP")).unwrap();
    println!("{:?}", settings.try_into::<HashMap<String, String>>().unwrap());
}

pub fn getConfig(key: &str) -> String {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("settings")).unwrap()
        .merge(config::Environment::with_prefix("APP")).unwrap();
    let hashConfig = settings.try_into::<HashMap<String, String>>().unwrap();
    let conn = hashConfig.get(key).unwrap();
    let std =String::from(conn);
    return std;
}

