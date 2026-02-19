use std::io::{self, Write};
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
        if response.contains(char::is_whitespace) == false{
            //println!("Simplefunction")
            _state = simplefunc(response); 
        }else{
            //println!("DoubleFunction");
            let collected: Vec<String> = response.split_whitespace().map(|v| v.to_string()).collect();
            _state = doublefunc(collected);
        }  
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

fn bananasplit(st:String) -> String {
    let mut vec = vec![];
    vec.push(st);
    let last = vec.last().unwrap().to_string();
    return last;
}

// FUNCIONES BASICAS DE TEXTO
fn simplefunc(_resp: String) -> bool{

    //========================VARIABLES========================
    let _salutations = ["hola", "buenas", "saludos"];
    let _lastback = bananasplit(_resp.clone());

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
    //else if _resp == "abre"{
    //    open::that("https://open.spotify.com/intl-es").unwrap();
    //    println!("Un poco de musica siempre va bien");
    //    return true;}

    else {
        println!("Disculpame pero no he entendido");
        return true;}
}

fn doublefunc(_resp: Vec<String>) -> bool{   
    if _resp[0] == "abre" {
        if _resp[1] == "spotify" {
            println!("Un poco de musica siempre va bien");
            open::that("https://open.spotify.com/intl-es").unwrap();
            return true;
        }else if _resp[1] == "reddit"{
            println!("A surfear los foros!");
            open::that("https://reddit.com").unwrap();
            return true;
        }else if _resp[1] == "github"{
            println!("Que repositorio querra ver, Jefe?");
            open::that("https://github.com").unwrap();
            return true;
        }else if _resp[1] == "portafolio"{
            println!("Que va a modificar, Jefe?");
            open::that("https://trestoxd.github.io/Portafolio/").unwrap();
            return true;
        }else if _resp[1] == "youtube"{
            println!("a ver algunos videos");
            open::that("https://www.youtube.com/").unwrap();
            return true;
        }else if _resp[1] == "Netflix"{
            println!("a ver algunos videos");
            open::that("https://www.netflix.com/").unwrap();
            return true;
        }
            
        // ERROR
        else {
            println!("Programa no encontrado");
            return true;
        }
    }else if _resp[0] == "quien"{
        if _resp[1] == "eres"{
            println!("Mi nombre es KAREN, soy tu asistente aqui disponible para cualquier contenido multimedia que quieras acceder");
            return true;
        }else if _resp[1] == "soy"{
            println!("No logro reconocerte, asi que deducire que eres mi ensamblador, Vito Minaya González");
            return true;
        }
        
        else {
            println!("No encontrado");
            return true;
        }
    }
    
    else {
        println!("Disculpame pero no he entendido");
        return true;
    }
}