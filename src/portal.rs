use std::f64::consts::PI;

#[derive(Copy, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Portal {
    pub lat: f64,           // Latitude
    pub lng: f64,           // Longitude
    pub position: Position, // 3d position on a sphere which r=1
    pub guid: String,
    pub latlng: String,
    pub label: String,
}

impl Portal {
    pub fn new(guid: String, latlng: String, label: String) -> Portal {
        let mut tmp_latlng = latlng.split_whitespace();

        let lat: f64 = tmp_latlng.next().unwrap().parse().unwrap();
        let lng: f64 = tmp_latlng.next().unwrap().parse().unwrap();
        let lat_rad = lat / 180.0_f64 * PI;
        let lng_rad = lng / 180.0_f64 * PI;

        let x = lat_rad.cos() * lng_rad.cos();
        let y = lat_rad.sin() * lng_rad.cos();
        let z = lng_rad.sin();

        Portal {
            lat: lat,
            lng: lng,
            position: Position { x: x, y: y, z: z },
            guid: guid,
            latlng: latlng,
            label: label,
        }
    }

    // If self is at the left side of vector a -> b
    pub fn is_left(&self, a: &Portal, b: &Portal) -> bool {
        let x0 = a.position.y * b.position.z - a.position.z * b.position.y;
        let y0 = a.position.z * b.position.x - a.position.x * b.position.z;
        let z0 = a.position.x * b.position.y - a.position.y * b.position.x;

        a.position.x * x0 + a.position.y * y0 + a.position.z * z0 > 0.0
    }
}

impl PartialEq for Portal {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid
    }
}
