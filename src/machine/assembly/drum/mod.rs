use std::collections::HashMap;
use std::fmt::Formatter;

#[derive(Copy, Clone, Debug)]
pub enum DrumVariant {
    Standard,
    Reflector,
    Thin,
    ThinReflector,
}

struct Permutation {
    forward: HashMap<char, char>,
    inverse: HashMap<char, char>,
}

impl Permutation {
    fn from_vec(alphabet: &Vec<char>, characters: &Vec<char>) -> Option<Self> {
        if alphabet.len() != characters.len() {
            return None;
        }
        let fw = alphabet
            .iter()
            .enumerate()
            .map(|(i, c)| (*c, characters[i]))
            .collect::<HashMap<char, char>>();
        let inv = characters
            .iter()
            .enumerate()
            .map(|(i, c)| (*c, alphabet[i]))
            .collect::<HashMap<char, char>>();
        Some(Self {
            forward: fw,
            inverse: inv,
        })
    }
}

pub struct Drum {
    size: usize,
    permutations: Option<Permutation>,
    notch: Vec<usize>,
    offset: u32,
    variant: DrumVariant,
}

impl Drum {
    pub fn new() -> Self {
        Self {
            size: 0,
            permutations: Permutation::from_vec(&vec!['a'], &Vec::new()),
            notch: Vec::new(),
            offset: 0,
            variant: DrumVariant::Standard,
        }
    }
    pub fn new_dummy() -> Self {
        let al = "A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z"
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<Vec<char>>();
        let sc = "E,K,M,F,L,G,D,Q,V,Z,N,T,O,W,Y,H,X,U,S,P,A,I,B,R,C,J"
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<Vec<char>>();
        let x = Permutation::from_vec(&al, &sc);
        Self {
            size: 26,
            permutations: x,
            notch: vec![4],
            offset: 0,
            variant: DrumVariant::Standard,
        }
    }
}

impl std::fmt::Display for DrumVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl std::fmt::Display for Drum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        /*let perms = self
            .permutations
            .iter()
            .map(|(key, value)| format!("{}:{}", key, value))
            .collect::<Vec<String>>()
            .join(" ");
        //Permutacje here
        */
        let perms = "TODO";
        let notches = self
            .notch
            .iter()
            .map(|u| u.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        write!(
            f,
            "{}\n{}\n{}\n{}\n{}",
            self.size, perms, notches, self.offset, self.variant
        )
    }
}
