#[macro_use]
extern crate serde;

use postgres::{Client, NoTls};

#[derive(Deserialize, Debug, Default, Clone)]
struct Toml {
    maps: Vec<TomlMap>,
}

#[derive(Deserialize, Debug, Default, Clone)]
struct TomlMap {
    name: String,
    region: String,
    tier: Vec<i32>,
    connections: Vec<Vec<String>>,
}

#[derive(Deserialize, Debug, Default, Clone)]
struct MapImport {
    name: Option<String>,
    regionname: Option<String>,
    tier0: Option<i32>,
    tier1: Option<i32>,
    tier2: Option<i32>,
    tier3: Option<i32>,
    tier4: Option<i32>,
    watchstones_0: Option<Vec<String>>,
    watchstones_1: Option<Vec<String>>,
    watchstones_2: Option<Vec<String>>,
    watchstones_3: Option<Vec<String>>,
    watchstones_4: Option<Vec<String>>,
}

fn main() {
    // let toml_str = include_str!("atlas.toml");
    // let toml_parsed: Toml = toml::from_str(&toml_str).expect("ERROR: toml parsing");

    let mut client = Client::connect("host=localhost user=postgres", NoTls).expect("ERROR");

    //     client.simple_query(
    //         "
    //     CREATE TABLE person (
    //         id              SERIAL PRIMARY KEY,
    //         WorldAreasKey
    //         name            TEXT NOT NULL,
    //         data            BYTEA
    //     )
    // ",
    //     ).expect("ERROR");

    //     let name = "Ferris";
    //     let data = None::<&[u8]>;
    //     client.execute(
    //         "INSERT INTO person (name, data) VALUES ($1, $2)",
    //         &[&name, &data],
    //     ).expect("ERROR");
    //     let bname = "Paris";
    //     let bdata = None::<&[u8]>;
    //     client.execute(
    //         "INSERT INTO person (name, data) VALUES ($1, $2)",
    //         &[&bname, &bdata],
    //     ).expect("ERROR");

    //     for row in client.query("SELECT id, name, data FROM person", &[]).expect("ERROR") {
    //         let id: i32 = row.get(0);
    //         let name: &str = row.get(1);
    //         let data: Option<&[u8]> = row.get(2);

    //         println!("found person: {} {} {:?}", id, name, data);
    //     }

    for row in client.query("SELECT * FROM rustatlas", &[]).expect("ERROR") {
        let map: MapImport = MapImport {
            name: row.get(0),
            regionname: row.get(1),
            tier0: row.get(2),
            tier1: row.get(3),
            tier2: row.get(4),
            tier3: row.get(5),
            tier4: row.get(6),
            watchstones_0: row.get(7),
            watchstones_1: row.get(8),
            watchstones_2: row.get(9),
            watchstones_3: row.get(10),
            watchstones_4: row.get(11),
        };
        // println!("{:#?}", map);
    }
}
