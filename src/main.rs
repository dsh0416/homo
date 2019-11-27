extern crate homo;

use crate::homo::solver::Solver;

fn main() {
    Solver::from_file(
        "/Users/delton/WorkSpace/Ingress/Ingress-Field-Design/HCF/portal.txt".to_string(),
    );
    println!("Hello, world!");
}
