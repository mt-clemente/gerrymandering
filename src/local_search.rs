use crate::{
    base::{Circonsciption, Problem, State},
    util::{print_cirs},
};
use rand::prelude::*;
use rand_distr::weighted_alias::WeightedAliasIndex;
use std::{cmp::max, collections::HashSet};

/// Inplace local search for a better solution

pub fn local_search(pb: &Problem, sol: &mut State, print_circs: bool) {
    let mut updated = true;

    let mut visisted_states = HashSet::new();

    let mut allow_neutral_moves = false;

    // if no better solution is found, we are in a minimum that we cannot get
    // out of using our algorithm: the solution that has been found is our best.
    while updated {
        let temp = sol.clone();
        let mut circs: Vec<&Circonsciption> = temp.circonscriptions.values().collect();

        circs.sort_by_key(|a| a.close_to_vic(pb));

        for circ in circs.clone() {
            let mut targets = circ.swap_available();
            targets.sort_by_key(|a| -circ.swap_heuristic(a.0, a.1, &mut sol.clone()).1);

            for t in targets {
                let (best_swap, val) = circ.swap_heuristic(t.0, t.1, &mut sol.clone());

                // if the best we can do has a heuristic of 0 it means that we cannot do better on this
                // circonscription (with our algorithm)
                if val == 0 {
                    updated = false;
                    continue;
                }

                let prev_score = sol.get_score();

                if !circ.domain.contains(&t) || !sol.circonscriptions.get(&sol.grid[t.0][t.1]).unwrap().domain.contains(&best_swap){
                    continue;
                }

                sol.swap(best_swap, t, pb);

                let grid = sol.get_grid();

                // handle the case where the swap results in a worse situation
                if visisted_states.contains(&grid) {
                    sol.swap(best_swap, t, pb);
                    updated = false;
                    continue;

                } else {
                    visisted_states.insert(sol.get_grid());
                }

                if sol.get_score() < prev_score
                    || !allow_neutral_moves && sol.get_score() == prev_score
                {
                    sol.swap(best_swap, t, pb);
                    updated = false;
                } else {
                    updated = true;
                    allow_neutral_moves = false;

                    break;
                }
            }

            // better solution found, as no solution will give us a better score difference than 1
            // we can break instead of testing the rest of the neighborhood
            if updated {
                if print_circs {
                    print_cirs(&sol);
                } else {
                    println!("{}",sol.get_score());
                }
            }
        }


        if !allow_neutral_moves && !updated {
            allow_neutral_moves = true;
            updated = true;
        }

    }
}

pub fn init_sol(pb: &Problem) -> State {
    let mut valid_patterns: Vec<(usize, usize)> = Vec::new();
    let k = (pb.lx * pb.ly) / pb.m;
    let max_iter = 100000;
    let mut sol = State::new_empty(pb);
    
    if pb.n % pb.m != 0 {
        
        sol = lv_fill(pb,max_iter).unwrap();



        let dbg : Vec<usize> = sol.circonscriptions.iter().map(|x| x.1.municipalities.len()).collect();
        dbg!(dbg);



    } else {
        for i in 1..pb.lx {
            for j in 1..pb.ly {
                if (i * j) % k == 0
                    && i + j - 2 <= pb.radius * (i * j) / k
                    && pb.lx % i == 0
                    && pb.ly % j == 0
                {
                    valid_patterns.push((i, j));
                }
            }
        }

        valid_patterns.sort_by_key(|a| (a.0 * a.1 + max(a.1, a.1)));

        let (x, y) = valid_patterns[0];

        let mut pattern: Vec<Vec<usize>> = vec![vec![0; y]; x];
        let mut tiles: Vec<(usize, usize)> = Vec::with_capacity(x * y);

        for i in 0..x {
            for j in 0..y {
                tiles.push((i, j));
            }
        }

        let mut c = 1;

        for t in tiles.clone().iter_mut() {
            if pattern[t.0][t.1] != 0 {
                continue;
            }

            let temp = tiles.clone();
            let mut neighbors: Vec<&(usize, usize)> =
                temp.iter().filter(|t| pattern[t.0][t.1] == 0).collect();
            neighbors.sort_by_key(|x| x.0.abs_diff(t.0) + x.1.abs_diff(t.1));

            for idxs in neighbors.iter().take(k) {
                pattern[idxs.0][idxs.1] = c;
            }

            c += 1;
        }

        let mut p = 0;

        for i in 0..(pb.lx / x) {
            for j in 0..(pb.ly / y) {
                for (di, pi) in pattern.iter().enumerate().take(x) {
                    for (dj, pj) in pi.iter().enumerate().take(y) {
                        sol.update_grid((i * x + di, j * y + dj), pj + p * x * y / k, pb)
                    }
                }

                p += 1;
            }
        }
    }

    sol
}

fn filling_heuristic((x, y): &(usize, usize), sol: &State, pb: &Problem) -> f64 {
    
    let x : i32 = x.clone().try_into().unwrap();
    let y : i32 = y.clone().try_into().unwrap();

    let mut s: i32 =  5;
    let radius = 1;
    for dx in -radius..radius+1 {
        for dy in -radius..radius+1 {

            if x + dx < 0
                || y + dy < 0
                || x + dx >= pb.lx as i32
                || y + dy >= pb.ly as i32
            {
                s+=1;
                continue;
            }

            if sol.grid[(x+dx) as usize][(y + dy) as usize] != 0 {
                s += 2;
            }
        }
    }


    (s.pow(2)) as f64
}


fn lv_fill(pb: &Problem, max_iter: usize) -> Result<State,&str> {

    for _ in 0..max_iter {

        
        if let Ok(mut sol) = proba_fill(pb) {

            for i in 0..pb.lx {
                for j in 0..pb.ly {
                    if sol.grid[i][j] != 0 {
                        continue;
                    }
                    for c in 0..pb.m {
    
                        let circ = sol.circonscriptions.get(&(c+1)).unwrap();
    
                        if circ.municipalities.len() >= pb.n / pb.m +1 {
                            continue;
                        }
    
                        if circ.domain.contains(&(i,j)) {
                            sol.update_grid((i,j),c+1, pb)
                        }
                    }
                }
            }
    
            let functional = sol.get_grid()
                        .iter()
                        .fold(true, |acc , x| acc && !x.contains(&0));

            if functional {
                return Ok(sol);
            }
        }

    }

    Err("No solution found")
}



fn proba_fill(pb: &Problem) -> Result<State,&str> {
    let mut rng = thread_rng();
    let mut sol = State::new_empty(pb);

    for i in 0..pb.m {
        let mut best: f64 = 0 as f64;
        let mut best_spot = (0, 0);

        for i in 0..pb.lx {
            for j in 0..pb.ly {
                if sol.grid[i][j] != 0 {
                    continue;
                }

                let h = filling_heuristic(&(i, j), &sol, pb);

                if h > best {
                    best = h;
                    best_spot = (i, j);
                }
            }
        }


        sol.update_grid(best_spot, i+1, pb);
        for _ in 1..pb.n/pb.m {
            let mut available = sol
            .circonscriptions
            .get(&(i +1))
            .unwrap()
            .domain
                .iter()
                .filter(|x| sol.grid[x.0][x.1] == 0);

            
            let d = available
                .clone()
                .map(|x| filling_heuristic(x, &sol,pb))
                .collect();
            
            if let Ok(w) = WeightedAliasIndex::new(d) {
                sol.update_grid(available.nth(w.sample(&mut rng)).unwrap().clone(), i+1, pb);

            }  else {
                return Err("No sol in this situation");
            }
            
        }
    }

    Ok(sol)
}