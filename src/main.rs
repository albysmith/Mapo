#[macro_use]
extern crate serde;

#[derive(Deserialize, Debug, Default, Clone)]
struct Toml {
    maps: Vec<TomlMap>,
}

#[derive(Deserialize, Debug, Default, Clone)]
struct TomlMap {
    name: String,
    region: String,
    tier: Vec<i32>,
    connections: Vec<Vec<String>>
}

fn main() {
    let toml_str = include_str!("atlas.toml");
    let toml_parsed: Toml = toml::from_str(&toml_str).expect("ERROR: toml parsing");

}
