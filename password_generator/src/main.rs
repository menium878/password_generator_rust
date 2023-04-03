mod unittest;

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
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz";

const NUMSET: &[u8] = b"0123456789";
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
    println!("Lenght of the password: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Given lenght: {}",input);
            match input.trim().parse(){ // !remember trim because will take whitespace if you don't do it
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
    
    match properties.kind { 
        PasswordKind::Letter=>{(0..properties.lenght).map(|_| {let idx = thread_rng().gen_range(0..CHARSET.len());
                CHARSET[idx] as char
                }).collect()},
        PasswordKind::Number=>{(0..properties.lenght).map(|_| {let idx = thread_rng().gen_range(0..NUMSET.len());
                    NUMSET[idx] as char
                }).collect()},
        PasswordKind::Mix=>{thread_rng()
                .sample_iter(&Alphanumeric)
                .take(properties.lenght.try_into().unwrap())
                .map(char::from)
                .collect()},
    }
}
fn main() {
    let lenght=input_lenght();
    let kind = choose_kind();
    let passprop=PasswordProperties::new(lenght, kind);
    let password=generate_password(passprop);
    println!("{}",password); // ! {:?} will give output with ""
    
}
