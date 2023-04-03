use std::{io, process};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
#[derive(Debug)]
struct PasswordProperties{
    lenght:i32,
    kind: PasswordKind,
}
impl PasswordProperties{
    fn new(lenght:i32,kind:PasswordKind)->PasswordProperties{
        PasswordProperties { lenght, kind }
    }
}
#[derive(Debug)]
enum PasswordKind {
    Number,
    Letter,
    Mix,
}


fn choose_kind()->PasswordKind{
println!("Choose_kind: \n1.Letter\n2.Number\n3.Mix");
let mut input = String::new();
match io::stdin().read_line(&mut input){
    Ok(_)=>{match input.as_str().trim(){
        "Letter" | "1"=>{println!("{}",input);
            PasswordKind::Letter}
        "Number" | "2"=>{println!("{}",input);
        PasswordKind::Number}
        "Mix" | "3"=>{println!("{}",input);
        PasswordKind::Mix}
        _ => {println!("Couldn't fine the type");
            process::exit(1)},}
    }
    _ => {println!("Error input");
        process::exit(1)},}
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

fn generate_password(properties:PasswordProperties)->String{
    let mut pwd = String::new(); // ? Do I need to keep it mut pytanie na później
    match properties.kind { 
        PasswordKind::Letter=>{println!("1");
        for x in 1..properties.lenght{
            //TODO: Logika funkcji + rand ale w zakresie ascii 65-90 A-Z i 97-172 a-z
        }pwd},
        PasswordKind::Number=>{println!("2");
        for x in 1..properties.lenght{
            //TODO: Logika funkcji + rand jak zrobić 
        }pwd},
        PasswordKind::Mix=>{println!("3");
            //TODO: Logika funkcji + rand jak zrobić połączyć oba rozwiązania 
            pwd = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(properties.lenght.try_into().unwrap())
                .map(char::from)
                .collect();
            pwd},
    }
}
fn main() {
    let lenght=input_lenght();
    let kind = choose_kind();
    let passprop=PasswordProperties::new(lenght, kind);
    let password=generate_password(passprop);
    println!("{:?}",password);
    
}
