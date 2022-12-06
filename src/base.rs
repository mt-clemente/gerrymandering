#![allow(dead_code)]
use std::{cmp::{self}, vec, collections::HashMap};

use crate::util::{ceil_div, null_saturated_sub};

pub struct Problem {
    pub lx: usize,
    pub ly: usize,
    pub n: usize,
    pub m: usize,
    pub votes : Vec<Vec<usize>>,
    pub threshold: usize,
    pub radius: usize

}

impl Problem {

    pub fn new(lx:usize,ly:usize,m:usize,votes:Vec<Vec<usize>>,threshold: usize) -> Problem{
        Problem{
            lx: lx,
            ly: ly,
            n : lx * ly,
            m: m,
            votes: votes,
            threshold: threshold,
            radius: ceil_div(lx * ly, 2*m)
        }
    }


    fn size_split(&self) -> (usize,usize) {
        let r = self.n % self.m;
        (r, self.m -r)

    }

    pub fn m(&self) -> usize {
        self.m
    }

}


pub struct State {
    pub grid: Vec<Vec<usize>>,
    pub circonscriptions : HashMap<usize,Circonsciption>,
}

impl State {

    pub fn new_empty(pb: &Problem) -> State {
        let grid = vec![vec![0;pb.ly]; pb.lx];
        let mut circonscriptions: HashMap<usize,Circonsciption>=  HashMap::with_capacity(pb.m);
        
        for i in 0..pb.m {
            circonscriptions.insert(i+1,Circonsciption::new(i+1, &pb));
        }

        State { grid: grid, circonscriptions: circonscriptions }
    }

    fn is_valid(&self, pb: &Problem) -> bool {

        if self.circonscriptions.len() != pb.m{
            return false
        }

        for circ in self.circonscriptions.values() {
            if pb.n % pb.m == 0 && pb.n / pb.m != circ.len() {
                return false
            }

            if (pb.n / pb.m) >= circ.len() || circ.len() >= (pb.n / pb.m) + 1 {
                return false
            }

            let temp = circ.clone();


            let mut lb = (0,0,0);
            let mut lt = (0,0,0);
            let mut rb = (0,0,0);
            let mut rt = (0,0,0);


            for elt in temp.municipalities.iter() {
                if elt.0 <= lb.0 && elt.1 <= lb.1  {
                    lb = *elt;
                    continue
                }
                if elt.0 <= lt.0 && elt.1 >= lt.1  {
                    lt = *elt;
                    continue
                }
                if elt.0 >= rb.0 && elt.1 <= rb.1  {
                    rb = *elt;
                    continue
                }
                if elt.0 >= rt.0 && elt.1 >= rt.1  {
                    rt = *elt;
                }
                
            }


            if cmp::max(rt.0.abs_diff(lb.0) + rt.1.abs_diff(lb.1), lt.0.abs_diff(rb.0) + lt.1.abs_diff(rb.1) ) > pb.n / (2 * pb.m) {
                return false
            }

        }

        true
        
    }


    pub fn get_score(&self) -> usize {
        let mut score = 0;

        for circ in self.circonscriptions.values() {

            let mut  circ_score = 0;

            for mun in circ.municipalities.iter() {
                circ_score += mun.2
            }

            if circ_score > circ.len() * 50 {
                score += 1
            } 

        }

        score

    }

    pub fn get_grid(&self) -> Vec<Vec<usize>> {
        self.grid.clone()
    }

    pub fn update_grid(& mut self, (x,y) : (usize,usize), circ_id:usize, pb: &Problem) {

        let prev_circ_id = self.grid[x][y];
        if prev_circ_id != 0 {
            self.circonscriptions.get_mut(&prev_circ_id).unwrap().remove_mun(x, y, &pb);
        }

        self.circonscriptions.get_mut(&circ_id).unwrap().push((x,y,pb.votes[x][y]), pb);
        self.grid[x][y] = circ_id;

    }


    pub fn swap(& mut self, origin: (usize, usize), target: (usize, usize), pb: &Problem ) {
        
        let temp = self.grid[origin.0][origin.1];
        dbg!(self.grid[target.0][target.1]);
        dbg!(self.grid[origin.0][origin.1]);
        self.update_grid((origin.0, origin.1),self.grid[target.0][target.1], pb);
        self.update_grid((target.0, target.1),temp, pb);

    }

}

impl Clone for State {

    fn clone(&self) -> Self {
        State { grid: self.grid.clone(), circonscriptions: self.circonscriptions.clone() }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Circonsciption {
    pub id: usize,
    pub domain : Vec<(usize,usize)>,
    pub municipalities : Vec<(usize,usize,usize)>
}

impl Circonsciption {

    pub fn new(id:usize,pb : &Problem) -> Circonsciption {
        Circonsciption{id: id, domain : Vec::with_capacity(2 * pb.radius.pow(2) - 2 * pb.radius), municipalities: Vec::with_capacity(pb.m)}
    }

    pub fn init(& mut self, pb: &Problem,mun : (usize,usize,usize)) {
        
        self.municipalities.push(mun);
        self.domain = create_domain(mun, &pb);

    }

    pub fn len(&self) -> usize {
        self.municipalities.len()
    }


    pub fn remove_mun(& mut self, x: usize, y: usize, pb : &Problem) {

        
        for i in 0..self.municipalities.len() {
            
            let m = self.municipalities[i];
            
            if m.0 == x && m.1 == y {
                self.municipalities.swap_remove(i);
                
                break;
            }
        }

        let d = create_domain((x,y,0), &pb);
        let temp = self.domain.clone();

        let t =self.domain.len(); 
        for d_mun in d {
            
            let mut in_domain = true;

            if temp.contains(&d_mun) {
                continue;
            }
            
            for mun in &temp {
                

                if mun.1.abs_diff(d_mun.1) + mun.0.abs_diff(d_mun.0) > pb.radius {
                    in_domain = false;
                    break;
                }
                
            }    

            if in_domain {
                self.domain.push(d_mun);
                print!("added");
            }
        }

    }

    fn push(& mut self, mun : (usize,usize,usize),pb : &Problem) {


        if self.domain.len() == 0 {
            self.init(&pb, mun);

        } else {

            assert!(self.domain.contains(&(mun.0,mun.1)));
            assert!(!self.municipalities.contains(&mun));
            
            self.municipalities.push(mun);
            
                
            self.domain.retain(|elt| mun.0.abs_diff(elt.0) + mun.1.abs_diff(elt.1) <= pb.radius);
                
        }
        
    }

    fn get_available_muns(&self) -> Vec<(usize, usize)> {
        self.domain.clone()
    }

    fn get_votes(&self) -> usize {

        let mut votes = 0;

        for mun in &self.municipalities {
            votes += mun.2;
        }

        votes
    }


    pub fn close_to_vic(&self, pb: &Problem) -> usize {
        
        let votes = self.get_votes();

        if votes < pb.threshold * self.municipalities.len() {
            pb.threshold * self.municipalities.len() - votes
        } else {
            pb.threshold + 1
        }
    }
    
    
    pub fn swap_heuristic(&self, i: usize, j: usize, state: & mut State, pb : &Problem) -> ((usize,usize), i128) {
        
        let grid = state.get_grid();
        let target_circ = state.circonscriptions.get(&state.grid[i][j]).unwrap();
        let target_threshold = 50 * target_circ.municipalities.len();
        let mut swapable: Vec<(usize,usize)> = Vec::new();

        for mun in &self.municipalities {

            if target_circ.swap_available(pb).contains(&(mun.0,mun.1)){
                swapable.push((mun.0, mun.1));
            }
        }

        if swapable.len() == 0 {
            return ((0,0),0);
        }

        
        let best = swapable.iter().max_by_key(|a| - (grid[a.0][a.1] as i128) ).unwrap();

        let v1 : i128 = (grid[i][j] + target_threshold).try_into().unwrap();
        let v2: i128 = (grid[best.0][best.1] + target_circ.get_votes()).try_into().unwrap();

        (* best, v1 - v2)

    }


    /// Return positions that are available to swap with this a municipality from this circonscription
    pub fn swap_available(&self, pb: &Problem) -> Vec<(usize, usize)>{
        let mut available = self.domain.clone();

        /*         let i = self.municipalities.iter().map(|x| x.0);
        let j = self.municipalities.iter().map(|x| x.1);


        let low_i = null_saturated_sub(i.clone().max().unwrap(),pb.radius);
        let low_j = null_saturated_sub(j.clone().max().unwrap(), pb.radius);
        let up_i = min(pb.lx-1, i.min().unwrap() + pb.radius);
        let up_j = min(pb.ly-1, j.min().unwrap() + pb.radius);

        for i in  low_i..up_i{
            for j in low_j..up_j {

                let temp: Vec<(usize,usize)> = self.municipalities.clone().iter().map(|x| (x.0,x.1)).collect();
                if self.domain.contains(&(i,j)) && !temp.contains(&(i,j)){
                    available.push((i,j));
                }
            }
        } */

        let occupied : Vec<(usize,usize)>= self.municipalities.iter().map(|x| (x.0,x.1)).collect();

        available.retain(|x| !occupied.contains(x));

        available

    }


}

impl Clone for Circonsciption {

    fn clone(&self) -> Self {
        Circonsciption { domain: self.domain.clone(), municipalities: self.municipalities.clone(), id : self.id}
    }
}

struct Neighborhood {
    states : Vec<State>
}

impl Neighborhood {
    
    fn new(states : Vec<State>) -> Neighborhood{
        Neighborhood {states: states}
    }

    fn is_empty(&self) -> bool {
        return self.states.len() == 0
    }

    fn remove(& mut self,idx: usize) {
        self.states.remove(idx);
    }


}
 


pub fn create_domain(mun: (usize,usize,usize), pb: &Problem) -> Vec<(usize,usize)> {

    let x = mun.0;
    let y = mun.1;

    let mut domain = Vec::with_capacity(2 * pb.radius.pow(2) - 2 * pb.radius);

    for i in -(pb.radius as i128)..pb.radius as i128 + 1{
        for j in  -(pb.radius as i128)..pb.radius as i128 + 1{
            if x  as i128 + i >= pb.lx as i128 ||
                x  as i128 + i < 0 ||
                y as i128 + j >= pb.ly as i128|| 
                y as i128 + j < 0||
                i.abs() + j.abs() > pb.radius as i128 ||
                (i == 0 && j == 0)
                {

                    continue;

            } else {
                domain.push(((x as i128 + i) as usize, (y as i128 + j )as usize));
            }
                
        }

    }
    
    domain
}