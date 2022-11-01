use std::iter::repeat;
use futures::FutureExt;
use crate::cell::State;
use crate::State::{S0, S1, S2};

pub struct Field {

    pub raw: Vec<Vec<State>>,

}

impl Field {

    pub fn new(max_x: usize, max_y: usize, default: State) -> Field {
        let mut vec_y = Vec::with_capacity(200);
        for _y in 0..max_y {
            let mut vec_x = Vec::with_capacity(200);
            for _x in 0..max_x {
                vec_x.push(default.clone());
            }
            vec_y.push(vec_x);
        }
        return Field { raw: vec_y };
    }

    pub fn tick_all(&mut self) {
        let mut new = self.raw.clone();
        for (y,xs) in (0_i32..).zip(self.raw.iter()) {
            for (x, _state) in (0_i32..).zip(xs.iter()) {
                new[y as usize][x as usize] = self.tick(x,y);
            }
        }
        self.raw = new
    }

    pub fn tick(&self, x: i32, y: i32) -> State {
        match self.get_cell(x, y) {
            S0 => {
                if self.get_around_cells(x, y).iter().filter(|state|{ state.clone().clone() == S1 }).count() == 3 {
                    S1
                } else {
                    S0
                }
            }
            S1 => {
                match self.get_around_cells(x, y).iter().filter(|state|{ state.clone().clone() == S1 }).count() {
                    0..=1 => S0,
                    2 | 3 => S1,
                    _ => S0
                }
            }
            _ => S2
        }
    }

    pub fn get_cell(&self, x: i32, y: i32) -> &State {
        return self.raw.get(y as usize).and_then(|ys| {
            ys.get(x as usize)
        }).unwrap_or(&S2);
    }

    pub fn get_around_cells(&self, x: i32, y: i32) -> Vec<State> {
        let mut ret = vec![];
        let mut f = |state: &State| {
            ret.push(state.clone())
        };
        f(self.get_cell(x-1,y+1));
        f(self.get_cell(x+0,y+1));
        f(self.get_cell(x+1,y+1));

        f(self.get_cell(x+1,y+0));
        f(self.get_cell(x-1,y+0));

        f(self.get_cell(x+1,y-1));
        f(self.get_cell(x-0, y-1));
        f(self.get_cell(x-1,y-1));
        return ret;
    }

}