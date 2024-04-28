use std::collections::HashSet;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct State {
    pub start : Option<(usize,usize)>,
    pub cleaned : Vec<(usize,usize)>,
    pub uncleaned: HashSet<(usize,usize)>,
    pub portals : Vec<(usize,usize)>,
    pub moves : Vec<String>,
}

impl State{
    pub fn new() -> Self{
        State{
            start: None,
            cleaned : Vec::new(),
            uncleaned : HashSet::new(),
            portals : Vec::new(),
            moves : Vec::new(),
        }
    }

    pub fn move_cleaner(&mut self,c: char){
        if self.start != None{
            let (x,y) = self.start.unwrap().clone();
            if c== 'N' || c == 'S'{
                if x != 0usize && x != 11usize {
                    match c {
                        'N' => self.make_if_contains(x-1usize,y),
                        'S'=> self.make_if_contains(x+1usize,y),
                        _ => ()
                    }


                }
            } else if c == 'W' || c == 'E'{
                if y != 0usize && y != 17usize{
                    match c {
                        'W' => self.make_if_contains(x,y-1usize),
                        'E' => self.make_if_contains(x,y+1usize),
                        _ => ()
                    }
                }
            }
        }

    }

    pub fn make_if_contains(&mut self,x:usize,y:usize){
      if self.portals.contains(&(x,y)){
          let mut m=(0usize,0usize);
          for i in &self.portals{
              if i != &(x,y){
                  m = *i;
              }
          }
          self.start = Some(m);
      }else{
            if self.uncleaned.contains(&(x,y)) {
                self.start = Some((x, y));
                self.cleaned.push((x, y));
                self.uncleaned.remove(&(x, y));
            }else{
                if self.cleaned.contains(&(x,y)){
                    self.start = Some((x,y));
                }
            }
      }
    }
}

impl Display for State{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"start:{:?},cleaned:{},uncleaned:{},portals: {}",self.start,self.cleaned.len(),self.uncleaned.len(),self.portals.len())
    }
}
