use std::collections::HashMap;
mod config;

fn main() {
    println!(
        "Your message is now: {}",
        enigma("Wiadomość testowa", "AAA")
    );
}

mod tests {
    use super::*;
    #[test]
    fn test_process_letter() {
        let config = config::Config::load_from_file("config.fesz").unwrap();

        let key = process_key("AAB", &config.alphabet);
        let mut drums: Vec<(Vec<char>, char, i32)> = Vec::new();
        set_drums(&mut drums, &config.drum_settings, key);
        assert_eq!('Z', process_letter(&mut drums, &config.alphabet, 'A'));
    }

    #[test]
    fn test_message() {
        assert_eq!("RRBLHVGść AZGHVPF", &enigma("Wiadomość testowa", "AAA"));
    }
}

fn enigma(message: &str, key: &str) -> String {
    if key.len() != 3 {
        panic!("key must be of 3 length") //TODO error handling
    }
    let config = config::Config::load_from_file("config.fesz").unwrap();
    let key = process_key(key, &config.alphabet);

    let mut drums: Vec<(Vec<char>, char, i32)> = Vec::new();
    set_drums(&mut drums, &config.drum_settings, key);

    process_message(message, &mut drums, &config.alphabet)
}

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
) -> String {
    let message = message.to_ascii_uppercase();
    let encrypted = message
        .chars()
        .map(|c| process_letter(drums, alphabet, c))
        .collect::<String>();
    encrypted
}

fn process_letter(
    drums: &mut Vec<(Vec<char>, char, i32)>,
    alphabet: &Vec<char>,
    letter: char,
) -> char {
    if !alphabet.contains(&letter) {
        return letter;
    }
    let mut tmp = letter;
    move_drums(drums, &alphabet);

    let mut idx: i32 = alphabet.iter().position(|&x| x == tmp).unwrap() as i32 - drums[2].2;
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
    tmp = alphabet[idx as usize];
    return tmp;
    //a i sumie jest tu naprawiony jeden błąd który mam w wersji JS :)
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

    //czym różnią się copy i clone?
    //copy jest dla prostych typow, ktorych kopiowanie nie jest kosztowne
    //np i32, albo jakas tupla (i32, i32)
    //a clone jest dla typow, które maja duzo danych, albo maja pamiec zaalokowana w roznych
    //miejscach
    //np. taki Vec<> ma swoj bufor zaalokowany gdzie indziej i trzeba go kopiowac, co moze byc
    //kosztowne
    //dlatego copy jest domyslne dla np. intow
    //jesli uwazasz ze twoj typ jest tani do skopiowania, to mozesz mu dac #[derive(Copy, Clone)]
    //let a = 0;
    //let b = a;
    //tutaj nie ma move, bo jak typ jest Copy to jest domyslnie kopiowany
    //
    //let a = Vec::new();
    //let b = a;
    //Vec nie jest Copy, wiec tutaj jest move,
    //jest tak dlatego, zeby przy takich typach trzeba bylo 'explicit' powiedziec, ze chcesz
    //skopiowac
    //let b = a.clone();
    //ok łapię

    //Tu zrobić wybieranie
}
