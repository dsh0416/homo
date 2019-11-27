use crate::portal::{Portal, Position};
use std::f64::consts::PI;

#[derive(Copy, Clone)]
pub struct Field<'a> {
    pub portals: [&'a Portal; 3],
    pub area: f64, // The area of the field on a sphere which r=1
}

impl<'a> Field<'a> {
    pub fn new(x: &'a Portal, y: &'a Portal, z: &'a Portal) -> Field<'a> {
        Field {
            area: Self::spherical_angle(&x, &y, &z)
                + Self::spherical_angle(&y, &z, &x)
                + Self::spherical_angle(&z, &x, &y)
                - PI,
            portals: [x, y, z],
        }
    }

    pub fn contains(&self, t: &Portal) -> bool {
        if t == self.portals[0] || t == self.portals[1] || t == self.portals[2] {
            return false;
        }

        if self.portals[2].is_left(self.portals[0], self.portals[1]) {
            return t.is_left(self.portals[0], self.portals[2])
                && t.is_left(self.portals[2], self.portals[1])
                && t.is_left(self.portals[1], self.portals[0]);
        }

        t.is_left(self.portals[0], self.portals[1])
            && t.is_left(self.portals[1], self.portals[2])
            && t.is_left(self.portals[2], self.portals[0])
    }

    fn spherical_angle(a: &Portal, b: &Portal, c: &Portal) -> f64 {
        let x0 = a.position.y * b.position.z - a.position.z * b.position.y;
        let y0 = a.position.z * b.position.x - a.position.x * b.position.z;
        let z0 = a.position.x * b.position.y - a.position.y * b.position.x;
        let tmp0 = Position {
            x: b.position.y * z0 - b.position.z * y0,
            y: b.position.z * x0 - b.position.x * z0,
            z: b.position.x * y0 - b.position.y * x0,
        };

        let x1 = c.position.y * b.position.z - c.position.z * b.position.y;
        let y1 = c.position.z * b.position.x - c.position.x * b.position.z;
        let z1 = c.position.x * b.position.y - c.position.y * b.position.x;
        let tmp1 = Position {
            x: b.position.y * z1 - b.position.z * y1,
            y: b.position.z * x1 - b.position.x * z1,
            z: b.position.x * y1 - b.position.y * x1,
        };

        ((tmp0.x * tmp1.x + tmp0.y * tmp1.y + tmp0.z * tmp1.z)
            / (tmp0.x * tmp0.x + tmp0.y * tmp0.y + tmp0.z * tmp0.z).sqrt()
            / (tmp1.x * tmp1.x + tmp1.y * tmp1.y + tmp1.z * tmp1.z).sqrt())
        .acos()
    }
}
