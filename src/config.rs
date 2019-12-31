use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
pub struct Config {
    pub alphabet: Vec<char>,
    pub drum_settings: Vec<DrumSetting>,
}

#[derive(Debug, Clone)]
pub struct DrumSetting {
    pub name: String,
    pub settings: Vec<char>,
    pub rot_idx: char,
}

enum ParserMode {
    Alphabet,
    Drum,
    Rev,
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
		let f = File::open(path)?;
		Self::load_from_buf(f)
	}

	pub fn load_from_buf<R: io::Read>(r: R) -> io::Result<Self> {
		let buf = BufReader::new(r);

        let mut config = Config {
            alphabet: Vec::new(),
            drum_settings: Vec::new(),
        };

        let mut mode = ParserMode::Alphabet;
        for line in buf.lines().filter_map(Result::ok) {
            if line.starts_with("#") || line.is_empty() {
                continue;
            }
            if line.starts_with("~") {
                mode = ParserMode::from_str(&line[1..]).unwrap(); //TODO error handling
            } else {
                match mode {
                    ParserMode::Alphabet => {
                        //TODO split into functions
                        config.alphabet = line
                            .trim()
                            .chars()
                            .filter(|&c| c != ',')
                            .collect::<Vec<char>>();
                    }
                    ParserMode::Drum => {
                        let name_len = line.find(",").unwrap(); //TODO: parser error handling
                        let rot_pos = line.find("&").unwrap();
                        let set = line[name_len..rot_pos]
                            .trim()
                            .chars()
                            .filter(|&c| c != ',')
                            .collect::<Vec<char>>();
                        let rot = line[(rot_pos + 1)..].trim().chars().collect::<Vec<char>>()[0];
                        let drum_setting = DrumSetting {
                            name: line[0..name_len].to_string(),
                            settings: set,
                            rot_idx: rot,
                        };
                        config.drum_settings.push(drum_setting);
                    }
                    ParserMode::Rev => {
                        let name_len = line.find(",").unwrap(); //TODO: parser error handling
                        let set = line[name_len..]
                            .trim()
                            .chars()
                            .filter(|&c| c != ',')
                            .collect::<Vec<char>>();
                        let drum_setting = DrumSetting {
                            name: line[0..name_len].to_string(),
                            settings: set,
                            rot_idx: ' ',
                        };
                        config.drum_settings.push(drum_setting);
                    }
                }
            }
        }
        Ok(config)
    }
}

impl ParserMode {
    fn from_str(name: &str) -> Result<ParserMode, String> {
        //TODO error handling
        match name {
            "Alphabet" => Ok(ParserMode::Alphabet),
            "Drum Settings" => Ok(ParserMode::Drum),
            "Rev Settings" => Ok(ParserMode::Rev),
            _ => Err(format!("Unknown mode: {}", name)),
        }
    }
}
