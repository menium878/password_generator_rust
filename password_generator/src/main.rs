use std::io;
#[derive(Debug)]
struct PasswordProperties{
    lenght:i32,
    kind: PasswordKind,
}
#[derive(Debug)]
enum PasswordKind {
    Number,
    Letter,
    Mix,
}
impl PasswordProperties{
    fn new(lenght:i32,kind:PasswordKind)->PasswordProperties{
        PasswordProperties { lenght, kind }
    }
}
fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Podany login to{}",input);
        }
        Err(error) => println!("error: {error}"),
    }
    let passprop=PasswordProperties::new(32, PasswordKind::Letter);
    println!("{:?}",passprop);
    
}
