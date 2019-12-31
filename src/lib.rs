use config::Config;
use wasm_bindgen::prelude::*;

pub mod config;
mod wasm_utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod tests {
	use super::*;
    #[test]
    fn test_process_letter() {
        let config = config::Config::load_from_file("config.fesz").unwrap();

        let key = process_key("AAB", &config.alphabet);
        let mut drums: Vec<(Vec<char>, char, i32)> = Vec::new();
        set_drums(&mut drums, &config.drum_settings, key);
        assert_eq!(
            'Z',
            process_letter(&mut drums, &config.alphabet, &config.alphabet, 'A')
        );
    }

    #[test]
    fn test_message() {
        let mut enigma = Enigma::new(Config::load_from_file("config.fesz").unwrap());
        assert_eq!("RRBLHVGść AZGHVPF", &enigma.run("AAA", "Wiadomość testowa"));
    }

    #[test]
    fn test_subs() {
        let mut enigma = Enigma::new(Config::load_from_file("config.fesz").unwrap());
        enigma.substitute('A', 'B');
        assert_eq!("VXXI", &enigma.run("AAA", "ASDF"));
    }
}

type Drum = (Vec<char>, char, i32);

#[wasm_bindgen]
#[derive(Debug)]
pub struct Enigma {
    config: Config,
    substitutions: Vec<char>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Enigma {
	pub fn new(config: Config) -> Self {
        let substitutions = config.alphabet.clone();

        Enigma {
            config,
            substitutions,
        }
    }
}

#[wasm_bindgen]
impl Enigma {
	#[cfg(target_arch = "wasm32")]
	pub fn new() -> Self {
		let config = Config::load_from_buf(include_str!("../config.fesz").as_bytes()).unwrap();
		let substitutions = config.alphabet.clone();

        Enigma {
            config,
            substitutions,
        }
	}

    pub fn run(&mut self, key: &str, message: &str) -> String {
        let key = process_key(key, &self.config.alphabet);
        let mut drums: Vec<Drum> = Vec::new();
        set_drums(&mut drums, &self.config.drum_settings, key);

        process_message(
            &message,
            &mut drums,
            &self.config.alphabet,
            &self.substitutions,
        )
    }

    pub fn substitute(&mut self, a: char, b: char) {
        let a_idx = self.substitutions.iter().position(|&c| c == a).unwrap();
        let b_idx = self.substitutions.iter().position(|&c| c == b).unwrap();
        let tmp = self.substitutions[a_idx];
        self.substitutions[a_idx] = self.substitutions[b_idx];
        self.substitutions[b_idx] = tmp;
    }
}
/*
fn encrypt(message: &str, key: &str) -> String {
    if key.len() != 3 {
        panic!("key must be of 3 length") //TODO error handling
    }
    let config = config::Config::load_from_file("config.fesz").unwrap();
    let substitutions = config.alphabet.clone();
    let enigma = Enigma {
        config,
        substitutions,
    };
    let key = process_key(key, &config.alphabet);

    let mut drums: Vec<(Vec<char>, char, i32)> = Vec::new();
    set_drums(&mut drums, &config.drum_settings, key);

    process_message(message, &mut drums, &config.alphabet)
}*/

fn process_key(key: &str, alphabet: &Vec<char>) -> [usize; 3] {
    let mut result = [0usize; 3];
    key.chars()
        .take(3)
        .map(|c| alphabet.iter().position(|&x| x == c).unwrap())
        .enumerate()
        .for_each(|(idx, el)| result[idx] = el);
    result
}

fn process_message(
    message: &str,
    drums: &mut Vec<(Vec<char>, char, i32)>,
    alphabet: &Vec<char>,
    subs_alphabet: &Vec<char>,
) -> String {
    let message = message.to_ascii_uppercase();
    let encrypted = message
        .chars()
        .map(|c| process_letter(drums, alphabet, subs_alphabet, c))
        .collect::<String>();
    encrypted
}

fn process_letter(
    drums: &mut Vec<(Vec<char>, char, i32)>,
    alphabet: &Vec<char>,
    subs_alphabet: &Vec<char>,
    letter: char,
) -> char {
    if !alphabet.contains(&letter) {
        return letter;
    }
    let mut tmp = letter;
    move_drums(drums, &alphabet);

    let mut idx: i32 = subs_alphabet.iter().position(|&x| x == tmp).unwrap() as i32 - drums[2].2;
    if idx < 0 {
        idx = alphabet.len() as i32 + idx
    }
    tmp = drums[2].0[idx as usize];

    idx = alphabet.iter().position(|&x| x == tmp).unwrap() as i32 - drums[1].2;
    if idx < 0 {
        idx = alphabet.len() as i32 + idx
    }
    tmp = drums[1].0[idx as usize];

    idx = alphabet.iter().position(|&x| x == tmp).unwrap() as i32 - drums[0].2;
    if idx < 0 {
        idx = alphabet.len() as i32 + idx
    }
    tmp = drums[0].0[idx as usize];

    idx = alphabet.iter().position(|&x| x == tmp).unwrap() as i32;
    tmp = drums[3].0[idx as usize];

    idx = drums[0].0.iter().position(|&x| x == tmp).unwrap() as i32 + drums[0].2;
    if idx >= alphabet.len() as i32 {
        idx = idx - alphabet.len() as i32
    }
    tmp = alphabet[idx as usize];

    idx = drums[1].0.iter().position(|&x| x == tmp).unwrap() as i32 + drums[1].2;
    if idx >= alphabet.len() as i32 {
        idx = idx - alphabet.len() as i32
    }
    tmp = alphabet[idx as usize];

    idx = drums[2].0.iter().position(|&x| x == tmp).unwrap() as i32 + drums[2].2;
    if idx >= alphabet.len() as i32 {
        idx = idx - alphabet.len() as i32
    }
    tmp = subs_alphabet[idx as usize];
    return tmp;
}

fn move_drums(drums: &mut Vec<(Vec<char>, char, i32)>, alphabet: &Vec<char>) {
    drums[2].2 += 1;

    if drums[2].2 == alphabet.iter().position(|&x| x == drums[2].1).unwrap() as i32 {
        drums[1].2 += 1;
    }
    if drums[1].2 == alphabet.iter().position(|&x| x == drums[1].1).unwrap() as i32 {
        drums[0].2 += 1;
    }
    if drums[2].2 >= alphabet.len() as i32 {
        drums[2].2 = 0;
    }
    if drums[2].2 >= alphabet.len() as i32 {
        drums[2].2 = 0;
    }
    if drums[2].2 >= alphabet.len() as i32 {
        drums[2].2 = 0;
    }
}

fn set_drums(
    drums: &mut Vec<(Vec<char>, char, i32)>,
    sets: &Vec<config::DrumSetting>,
    pos_set: [usize; 3],
) {
    drums.push((sets[0].settings.clone(), sets[0].rot_idx, pos_set[0] as i32));
    drums.push((sets[1].settings.clone(), sets[1].rot_idx, pos_set[1] as i32));
    drums.push((sets[2].settings.clone(), sets[2].rot_idx, pos_set[2] as i32));
    drums.push((sets[3].settings.clone(), sets[3].rot_idx, -1));
}
