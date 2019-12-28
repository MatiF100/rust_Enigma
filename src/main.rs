use std::collections::HashMap;
mod config;

fn main() {
    println!("Your message is now: {}", enigma("message", "key"));
}

fn enigma<'a>(message: &'a str, key: &str) -> &'a str {
    //"An error occured!"
    let config = config::Config::load_from_file("config.fesz").unwrap();
    //dbg!(&config);

    let mut drums: Vec<(HashMap<char, char>, char)> = Vec::new();
    set_drums(&mut drums, &config.drum_settings);
    process_letter(&drums);
    //dbg!(&drums);

    todo!("An error occured!")
}

fn process_letter(drums: &Vec<(HashMap<char, char>, char)>) {
    let mut tmp = 'G';

    //Feeding drums with the letter
    tmp = *drums[2].0.get(&tmp).unwrap();
    tmp = *drums[1].0.get(&tmp).unwrap();
    tmp = *drums[0].0.get(&tmp).unwrap();

    //Signal passed through REVERSING DRUM
    tmp = *drums[3].0.get(&tmp).unwrap();

    //Feeding drums with the letter from beheind
    tmp = drums[0]
        .0
        .iter()
        .filter(|&(_, v)| *v == tmp)
        .map(|(a, _)| *a)
        .collect::<Vec<char>>()[0];
    tmp = drums[1]
        .0
        .iter()
        .filter(|&(_, v)| *v == tmp)
        .map(|(a, _)| *a)
        .collect::<Vec<char>>()[0];
    tmp = drums[2]
        .0
        .iter()
        .filter(|&(_, v)| *v == tmp)
        .map(|(a, _)| *a)
        .collect::<Vec<char>>()[0];
    println!("{}", tmp);
}

fn set_drums(drums: &mut Vec<(HashMap<char, char>, char)>, sets: &Vec<config::DrumSetting>) {
    drums.push((sets[0].settings.clone(), sets[0].rot_idx));
    drums.push((sets[1].settings.clone(), sets[1].rot_idx));
    drums.push((sets[2].settings.clone(), sets[2].rot_idx));
    drums.push((sets[3].settings.clone(), sets[3].rot_idx));

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
