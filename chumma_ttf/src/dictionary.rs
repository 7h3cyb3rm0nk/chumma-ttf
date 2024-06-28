use std::collections::HashMap;
use std::include_str;

fn read_config() -> String {
    let config: &str = include_str!("words.conf");
    config.strip_suffix("\n").expect(config).to_string()
}

fn parse_config(config: String) -> HashMap<String, String> {
    let mut store: HashMap<String, String> = HashMap::new();

    let list: Vec<&str> = config.split("\n").collect();
    // println!("{:?}", list);
    for i in list.iter() {
        let temp: Vec<_> = i.split(":").collect();
        store.insert(temp[0].to_string(), temp[1].to_string());
    }
    store
}

pub fn generate_words() -> HashMap<String, String> {
    let config = read_config();
    parse_config(config)
}

#[cfg(test)]

mod tests {

    use super::*;
    #[test]
    fn test_parse_config() {
        let parsed_config_value: Vec<String> =
            parse_config("Hello:Hi".to_string()).into_values().collect();
        assert_eq!(parsed_config_value, ["Hi"]);

        let parsed_config_key: Vec<String> =
            parse_config("Hello:Hi".to_string()).into_keys().collect();

        assert_eq!(parsed_config_key, ["Hello"]);
    }

    #[test]
    fn test_generate_config() {
        println!("{:?}", generate_words());
        assert!(true);
        // assert!(false);
        // uncomment the above line to print the stdout
    }
}
