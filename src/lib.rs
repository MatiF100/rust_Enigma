mod machine;

pub fn hello() {
    let x = machine::assembly::drum::Drum::new_dummy();
    println!("I'm a dummy! {}", x);
}
