use crate::portal::Portal;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

pub struct Solver {
    portals: Vec<Portal>,
}

impl Solver {
    pub fn from_string(contents: String) -> Solver {
        let mut portals = vec![];

        let data: Value = serde_json::from_str(&contents.to_string()).unwrap();
        let bkmrk: &Value = &data["portals"]["idOthers"]["bkmrk"];

        for entity in bkmrk.as_object().unwrap() {
            portals.push(Portal::new(
                entity.1["guid"].as_str().unwrap().to_string(),
                entity.1["latlng"].as_str().unwrap().to_string(),
                entity.1["label"].as_str().unwrap().to_string(),
            ));
        }
        println!("Json File Parsed, {} Portals Founded.", portals.len());
        Solver { portals: portals }
    }

    pub fn from_file(path: String) -> Solver {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        Self::from_string(contents)
    }

    pub fn solve(&self) {}
}
