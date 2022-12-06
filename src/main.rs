mod util;
use base::Problem;
use util::parse_file;

mod base;
mod local_search;
use local_search::{init_sol, local_search};

use crate::util::print_sol;

fn main() {
    let fname = "exemplaires/36_50_0.txt";
    let (lx, ly, grid) = parse_file(fname);
    let threshold: usize = 50;
    let m = 200;
    let pb = Problem::new(lx, ly, m, grid, threshold);

    let sol = &mut init_sol(&pb);

    dbg!(pb.radius);
    dbg!(sol.get_score());
    print_sol(sol.get_grid());
    local_search(&pb, sol);
    dbg!(sol.is_valid(&pb));
    print_sol(sol.get_grid());
}
