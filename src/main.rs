use std::{io::{self, Write}};
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

        let response = response.as_str(); //Es mejor trabajar con &str por más facilidad de manejo

        if response.contains(char::is_whitespace) == false{
            //println!("Simplefunction")
            _state = simplefunc(response); 
        }else{
            //println!("DoubleFunction");
            let collected: Vec<&str> = response.split_whitespace().map(|v| v).collect();
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

fn bananasplit(st:&str) -> &str {
    let mut vec = vec![];
    vec.push(st);
    let last = vec.last().unwrap();
    return last;
}

// FUNCIONES BASICAS DE TEXTO
fn simplefunc(_resp: &str) -> bool{

    //========================VARIABLES========================
    let _salutations = ["hola", "buenas", "saludos"];
    let _lastback = bananasplit(_resp);

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

fn doublefunc(_resp: Vec<&str>) -> bool{   
    //Cambiar los "if" por "match", cuando hay varios casos es mucho más rápido porque solo comprueba el valor 1 vez
    
    //Abrir
    if _resp[0] == "abre" {
        match _resp[1] {
            "spotify" => {
                println!("Un poco de musica siempre va bien");
                open::that("https://open.spotify.com/intl-es").unwrap();
                return true;
            },

            "reddit" => {
                println!("A surfear los foros!");
                open::that("https://reddit.com").unwrap();
                return true;
            },

            "github" => {
                println!("Que repositorio querra ver, Jefe?");
                open::that("https://github.com").unwrap();
                return true;
            },

            "portafolio" => {
                println!("Que va a modificar, Jefe?");
                open::that("https://trestoxd.github.io/Portafolio/").unwrap();
                return true;
            },

            "youtube" => {
                println!("a ver algunos videos");
                open::that("https://www.youtube.com/").unwrap();
                return true;
            },

            "netflix" => {
                println!("no volverá a ver inside job o si?");
                open::that("https://www.netflix.com/").unwrap();
                return true;
            }

            _ => {
                println!("Programa no encontrado");
                return true;
            },
        };
    // QUIEN
    } else if _resp[0] == "quien"{

        match _resp[1] {
            "eres" => {
                println!("Mi nombre es KAREN, soy tu asistente aqui disponible para cualquier contenido multimedia que quieras acceder");
                return true;
            },

            "soy" => {
                println!("No logro reconocerte, asi que deducire que eres mi ensamblador, Vito Minaya González");
                return true;
            },

            "es" => {
                if _resp.len() == 2 || _resp[2].contains(char::is_whitespace) {
                    println!("No logro reconocer a esa persona, mi visión y audio parecen estar desactivados");
                    return true;
                } else if _resp[2] == "jiayang" || _resp[2] == "yang"{
                    let _name = _resp[2].to_string();
                    println!("{_name}, es su mejor amigo jefe, desea conocer más información de el?");
                    return true;
                }else if _resp[2] == "alba" || _resp[2] == "rene"{
                    let _name = _resp[2].to_string();
                    println!("{_name}, es el Amor de su vida señor, la razón de su esfuerzo, se encuentra bien? o simplemente necsita mas información sobre ella.");
                    return true;
                }else if _resp[2] == "mi"{
                    if _resp.len() == 3{
                        let _name = _resp[2].to_string();
                        println!("{_name}? no logro reconocerlo");
                        return true;
                    }else if _resp[3] == "amor"{
                        println!("Su Amor jefe, es su amada Alba");
                        return true;
                    }else {
                        println!("Eso esta fuera de mi base de datos");
                        return true;
                    }
                }else {
                    println!("Esa persona esta fuera de mi base de datos");
                    return true;
                }
            },

            _ => {
                println!("No encontrado");
                return true;
            }
        }
    }
    
    if _resp[0] == "buscar"{
        if _resp[1] == "es"{
            return true;
        }else if _resp[1] == "platano"{
            return true;
        }else {
            open::that(busqueda_web(_resp[1].to_string(), "Default".to_string())).unwrap();
            return true;
        }
    }
    else {
        println!("Disculpame pero no he entendido");
        return true;
    }
}

fn busqueda_web(_input:String, _browser:String) -> String{
    let path:String;
    let name:String;
    let result:String;

    if _browser.to_lowercase() == "default"{
        path = "https://duckduckgo.com/?q=".to_string();
        name = _input;

        result = format!("{}{}", path, name);   
        return result;
    }else {
        path = "https://duckduckgo.com/?q=".to_string();

        return path;
    }
}