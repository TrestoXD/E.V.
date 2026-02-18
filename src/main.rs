use std::{io::{self, Write}, str::SplitWhitespace, u8};
use open;
fn main() {
    //==Variables==
    let mut _state: bool = true;
    

    // =WELLCOME=
    initialize();

    while _state {
        let mut response: String = String::new();
        
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line");
            response = response.trim().to_lowercase();

        _state = whatmeans(response);   
    }
}

// FUNCION DE INICIO

fn initialize() {
    println!("Hola, mi nombre es:");
        print!("====================================");
    println!( r#"    
    __ __ ___    ____  _______   __
   / //_//   |  / __ \/ ____/ | / /
  / ,<  / /| | / /_/ / __/ /  |/ / 
 / /| |/ ___ |/ _, _/ /___/ /|  /  
/_/ |_/_/  |_/_/ |_/_____/_/ |_/
    "#);
    println!("====================================");
    println!("Que necesitas?
    ");
}

// FUNCIONES RELACIONADAS CON EL PROCESAMIENTO DEL TEXTO

fn bananasplit(st:String, id:u8) -> String {
    let mut i:usize = 0;
    let mut res: [String; 2] = [; 2];
    for token in st.split_whitespace(){
        res[i] = token;
        i+=1;
    }
    return res(id);
}

fn whatmeans(_response: String,) -> bool{

    //========================VARIABLES========================
    let _salutations = ["hola", "buenas", "saludos"];
    let mut _resp: String = bananasplit(_response);

    //===FUNCIONES PRINCIPALES===
    if _resp ==  "exit" {
        println!("Apagando...");
        return false;
    }else if _resp == "title" {
        initialize();
        return true;}


    //=================================FUNCIONES DE RESPUESTA================================

    //=========================================SALUDOS========================================
    else if _resp == _salutations[1] || _resp == _salutations[0] || _resp == _salutations[2]{
        println!("Hola! Como Estas?");   
        return true;}
    else if _resp == "version"{
        println!(env!("CARGO_PKG_VERSION"));
        return true;}
    else if _resp == "abre"{
        open::that("https://open.spotify.com/intl-es").unwrap();
        println!("Un poco de musica siempre va bien");
        return true;}

    else {
        println!("Disculpame pero no he entendido");
        return true;}
}