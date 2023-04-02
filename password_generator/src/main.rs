use std::{io, process};
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
fn input_lenght()->i32{
    println!("Podaj długość hasła: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Podana długość to: {}",input);
            match input.trim().parse(){ // !pamiętaj musi być trim bo inaczej występuje błąd, że bierze enter jako znak (tak obstawiam)
                Ok(input) => input,
                Err(_e) => {println!("Error_1 {}",_e);
            process::exit(1)},
            }   
        }
        Err(error) =>{println!("Error {}",error);
        process::exit(1);},
        }
        
    
}
fn main() {
    let lenght=input_lenght();
    let passprop=PasswordProperties::new(32, PasswordKind::Letter);
    println!("{:?}",passprop);
    
}
