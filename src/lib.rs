pub mod state;

use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use state::State;
pub fn create_state(path: &str) -> State {
    let mut f = File::open(path);
    let mut state = State::new();
    let reader = BufReader::new(f.unwrap());
    let mut m = 0usize;
    for line in reader.lines() {
        let mut n = 0usize;
        let mut store = String::from(line.unwrap());
        if store.contains("X") {
            for c in store.chars() {
                match c {
                    'S' => {state.start = Some((m,n));
                            state.cleaned.push((m,n))},
                    ' ' => {state.uncleaned.insert((m,n));},
                    'P' => state.portals.push((m,n)),
                    _ => (),
                }

                n = n + 1usize;
            }
            m = m + 1usize;
        }else{
            state.moves.push(store);
        }
    }

    state
}

pub fn no_start_given(state: &mut State) -> HashSet<(usize,usize)>{
    let mut ret: HashSet<(usize,usize)> = HashSet::new();
    if state.start == None{
        let mut moves = &state.uncleaned.clone();
        for i in moves{
            let mut m=String::new();
            if state.moves.len() == 2 {
                m = String::from(&state.moves[1]);
            }
            let mut new_state = state.clone();
            &new_state.uncleaned.remove(&i);
            &new_state.cleaned.push(*i);
            new_state.start =Some(*i);
            for i in m.chars(){
                &new_state.move_cleaner(i);
            }
            ret.extend(&new_state.uncleaned);

        }
    }

    ret
}