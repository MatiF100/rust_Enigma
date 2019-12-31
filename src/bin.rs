use rust_enigma::{config::Config, Enigma};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut enigma = Enigma::new(Config::load_from_file("config.fesz")?);

    let mut key = String::new();
    let mut message = String::new();

    let mut left = String::new();
    let mut mid = String::new();
    let mut right = String::new();
    let mut reverse = String::new();

    let [drums, revs] = enigma.aod();
    for drum in drums {
        println!("{}: {}", drum.0, drum.1);
    }
    print!("Type in number of the right drum: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut right)?;

    print!("Type in number of the middle drum: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut mid)?;

    print!("Type in number of the left drum: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut left)?;

    for rev in revs {
        println!("{}: {}", rev.0, rev.1);
    }
    print!("Type in number of the reversing drum: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut reverse)?;

    let left = left.trim().parse::<usize>().unwrap();
    let mid = mid.trim().parse::<usize>().unwrap();
    let right = right.trim().parse::<usize>().unwrap();
    let reverse = reverse.trim().parse::<usize>().unwrap();

    print!("Key: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut key)?;

    print!("Message: ",);
    io::stdout().flush()?;
    io::stdin().read_line(&mut message)?;

    let output = enigma.run(key.trim(), message.trim(), [right, mid, left, reverse]);
    println!("Output: {}", output);

    Ok(())
}
