use std::{cmp::max};

use crate::{base::{Problem, State, Circonsciption}, util::print_sol};


/// Inplace local search for a better solution

pub fn local_search(pb: &Problem,sol: & mut State) {

    let mut updated = true;
    
    // if no better solution is found, we are in a minimum that we cannot get
    // out of using our algorithm: the solution that has been found is our best.
    while updated {

        let temp = sol.clone();
        let mut circs: Vec<&Circonsciption>=  temp.circonscriptions.values().collect();

        circs.sort_by_key(|a| a.close_to_vic(&pb));

        for circ in circs.clone() {

            let mut targets = circ.swap_available(&pb);
            targets.sort_by_key(|a| - circ.swap_heuristic(a.0, a.1, &mut sol.clone(), &pb).1);


            for t in targets {
                let (best_swap, val) = circ.swap_heuristic(t.0, t.1, & mut sol.clone(), &pb);

                // if the best we can do has a heuristic of 0 it means that we cannot do better on this
                // circonscription (with our algorithm)
                if val == 0 {
                    updated = false;
                    continue;
                }


                let prev_score = sol.get_score();
                sol.swap(best_swap, t, pb);

                
                // handle the case where the swap results in a worse situation
                if  sol.get_score() <= prev_score {
                    
                    sol.swap(best_swap, t, pb);
                    updated = false;
                    
                } else {
                    updated = true;
                    
                    break;
                }

            }

            // better solution found, as no solution will give us a better score difference than 1
            // we can break instead of testing the rest of the neighborhood
            if updated {
                println!("New score --> {} \r",sol.get_score());
                break
            }
            
        }

    }

}


pub fn init_sol(pb: &Problem) -> State{

    let mut valid_patterns: Vec<(usize,usize)> = Vec::new();
    let k = (pb.lx * pb.ly) / pb.m;


    for i in 1..pb.lx {
        for j in 1..pb.ly {
            if (i * j) % k == 0  && i + j - 2 <= pb.radius * (i * j) / k  && pb.lx%i == 0 && pb.ly % j == 0{
                valid_patterns.push((i,j));
            } 
        }
    }

    valid_patterns.sort_by(|a,b| (a.0*a.1 + max(a.1,a.1)).cmp(&(b.0*b.1 + max(b.1,b.1))));


    let (x,y) = valid_patterns[0];

    let mut pattern: Vec<Vec<usize>> = vec![vec![0; y]; x];
    let mut tiles: Vec<(usize,usize)> = Vec::with_capacity(x * y);

    for i in 0..x {
        for j in 0..y {
            tiles.push((i,j));
        }
    }

    let mut c = 1;


    for t in tiles.clone().iter_mut() {

        if pattern[t.0][t.1] != 0{
            continue;
        }

        let temp = tiles.clone();
        let mut neighbors: Vec<&(usize,usize)> = temp.iter().filter(|t| pattern[t.0][t.1] == 0).collect();
        neighbors.sort_by_key(|x| x.0.abs_diff(t.0) + x.1.abs_diff(t.1));


        for i in 0..k {
            let idxs = neighbors[i];
            pattern[idxs.0][idxs.1] = c;
        }

        c += 1;
    }


    let mut sol = State::new_empty(&pb);

    let mut p = 0;

    for i in 0..(pb.lx / x) {
        for j in 0..(pb.ly / y) {
            for di in 0..x {
                for dj in 0..y{

                    sol.update_grid((i * x + di, j * y + dj), pattern[di][dj] + p * x * y / k, pb)
                
                }
            }

            p += 1;
        }
    }

    sol

}

