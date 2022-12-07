mod util;
use base::Problem;
use util::parse_file;
mod base;
mod local_search;
use local_search::{init_sol, local_search};


fn main() {

    let args: Vec<_> = std::env::args().collect();
    let fname = &args[1];
    let m: usize = args[2].parse().unwrap();
    let print_circs = args[3].parse().unwrap();


    let (lx, ly, grid) = parse_file(fname.as_str());
    let threshold: usize = 500;
    let pb = Problem::new(lx, ly, m, grid, threshold);

    let sol = &mut init_sol(&pb);
    local_search(&pb, sol, print_circs);
}
