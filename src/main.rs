extern crate homo;

use crate::homo::field::Field;
use crate::homo::portal::Portal;

fn main() {
    let a = Portal::new(
        "test".to_string(),
        "30.02 120.01".to_string(),
        "label".to_string(),
    );
    let b = Portal::new(
        "test".to_string(),
        "30.01 120.02".to_string(),
        "label".to_string(),
    );
    let c = Portal::new(
        "test".to_string(),
        "30.03 120.03".to_string(),
        "label".to_string(),
    );

    let f = Field::new(&a, &b, &c);
    println!("Hello, world!, {}", f.area);
}
