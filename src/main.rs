extern crate homo;

use crate::homo::solver::Solver;

fn main() {
    let mut solver = Solver::from_file(
        "/Users/delton/WorkSpace/Ingress/Ingress-Field-Design/HCF/portal.txt".to_string(),
    );
    solver.solve();
    println!("Hello, world!");
}
