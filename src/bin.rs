use rust_enigma::{config::Config, Enigma};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut enigma = Enigma::new(Config::load_from_file("config.fesz")?);

    let mut key = String::new();
    let mut message = String::new();

    print!("Key: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut key)?;

    print!("Message: ",);
    io::stdout().flush()?;
    io::stdin().read_line(&mut message)?;

    let output = enigma.run(key.trim(), message.trim());
    println!("Output: {}", output);

    Ok(())
}
