use crate::field::Field;
use crate::portal::Portal;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

pub struct Solver<'a> {
    portals: Vec<Portal>,
    solutions: [Vec<Solution<'a>>; 6],
}

pub struct Solution<'a> {
    pub field: Field<'a>,
    pub central: Option<&'a Portal>,
}

impl<'a> Solver<'a> {
    pub fn from_string(contents: String) -> Solver<'a> {
        // Initialize Portals
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

        // Initialize Solutions
        let mut solutions: [Vec<Solution<'a>>; 6] = Default::default();

        for i in 0..6 {
            solutions[i] = Vec::new();
        }

        Solver {
            portals: portals,
            solutions: solutions,
        }
    }

    pub fn from_file(path: String) -> Solver<'a> {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        Self::from_string(contents)
    }

    pub fn solve(&'a mut self) {
        // First Layer
        print!("Generating Layer 1 Homogeneus Solutions...");
        for i in 0..self.portals.len() {
            for j in i..self.portals.len() {
                for k in j..self.portals.len() {
                    self.solutions[0].push(Solution {
                        field: Field::new(&self.portals[i], &self.portals[j], &self.portals[k]),
                        central: None,
                    });
                }
            }
        }
        println!("OK. Solutions: {}", self.solutions[0].len());

        // 2 - 6 Layers
        for layer in 0..5 {
            print!("Generating Layer {} Homogeneus Solutions...", layer + 2);
            for i in 0..self.solutions[layer].len() {
                for j in i..self.solutions[layer].len() {
                    for k in j..self.solutions[layer].len() {
                        // Check if i, j, k could construct a new field
                        let x = self.solutions[layer][i].field.portals;
                        let y = self.solutions[layer][j].field.portals;
                        let z = self.solutions[layer][k].field.portals;

                        for c in 0..3 {
                            // Assume the central portal
                            if y.contains(&x[c]) && z.contains(&x[c]) {
                                // Central portal Found
                                let mut t0 = x.to_vec();
                                let mut t1 = y.to_vec();
                                let mut t2 = z.to_vec();
                                t0.retain(|&t| !y.contains(&t));
                                t1.retain(|&t| !x.contains(&t));
                                t2.retain(|&t| !x.contains(&t));
                                if t0.len() == 1 && t1.len() == 1 && t2.len() == 1 {
                                    // Found a higher field!
                                    self.solutions[layer + 1].push(Solution {
                                        field: Field::new(t0[0], t1[0], t2[0]),
                                        central: Some(x[c]),
                                    });
                                }
                                break;
                            }
                        }
                    }
                }
            }
            println!("OK. Solutions: {}", self.solutions[layer + 1].len());
        }
    }
}
