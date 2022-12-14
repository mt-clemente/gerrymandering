use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::base::{State};

pub fn parse_file(fname: &str) -> (usize, usize, Vec<Vec<usize>>) {
    if let Ok(mut lines) = read_lines(fname) {
        if let Some(Ok(s)) = lines.next() {
            let dims: Vec<&str> = s.split(' ').collect();
            let lx: usize = dims[0].parse().unwrap();
            let ly: usize = dims[1].parse().unwrap();
            println!("Treating {lx} x {ly} grid");

            let mut grid: Vec<Vec<usize>> = Vec::with_capacity(lx * ly);
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(s) = line {
                    let parsed: Vec<usize> =
                        s.split(' ').flat_map(|x| x.parse::<usize>()).collect();

                    grid.push(parsed);
                }
            }

            (lx, ly, grid)
        } else {
            panic!("Error while parsing the grid");
        }
    } else {
        panic!("Error while parsing the grid");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[allow(dead_code)]
pub fn print_sol(sol: Vec<Vec<usize>>) {
    for i in 0..sol.len() {
        for j in 0..sol[0].len() {
            print!("{} ", sol[i][j]);
        }

        println!("");
    }
}

pub fn ceil_div(n: usize, k: usize) -> usize {
    if n % k == 0 {
        return n / k;
    }

    n / k + 1
}


pub fn print_cirs(sol : &State) {
    for (_, circ) in sol.circonscriptions.iter() {
        for mun in circ.municipalities.clone() {
            print!("{} {} ",mun.0,mun.1);
        }
        print!("\n");
    }
    print!("\n");
}