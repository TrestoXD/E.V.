use std::{io::{self, Write}};
use open;
use tokio::runtime::Runtime;

use crate::features::{interpreter::{self, get_function}, search};

mod features {
    pub mod search;
    pub mod interpreter;
    pub mod data;
}
fn main() {
    //==Variables==
    let mut _state: bool = true;


    
    let mut _user_data = features::data::user_data_initialization();

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

        let response = response.as_str();

        if response.contains(char::is_whitespace) == false{
            if response != ""{
                _state = simplefunc(response); 
            }
            
        }else{
            _state = doublefunc(response, &_user_data);
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

fn bananasplit(st: &str) -> &str { //Función obsoleta. Hay que cambiarlo
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
    if _resp ==  "exit" || _resp == "salir" {
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

    else {
        println!("Disculpame pero no he entendido");
        return true;}
}

fn doublefunc(_resp: &str, _user_data: &json::JsonValue) -> bool{   
    let (tipo_respuesta, objeto) = get_function(_resp, &_user_data);
    
    
    match tipo_respuesta {
        interpreter::TipoRespuesta::Hecho => {return true;},
        interpreter::TipoRespuesta::Buscar => {
            Runtime::new().unwrap().block_on(search::search(objeto));
            return true;
        },
        interpreter::TipoRespuesta::Persona => {
            let datos_inicializados = features::data::program_data_initialization();
            let nombres_respuestas = &datos_inicializados[0];
            let apodos = &datos_inicializados[1];

            match nombres_respuestas.get(objeto.as_str()) {
                Some(respuesta) => {println!("{}", respuesta); return true;}, 
                None => {}
            }
            match apodos.get(objeto.as_str()) {
                Some(apodo) => {
                    match nombres_respuestas.get(apodo) {
                        Some(respuesta) => println!("{}", respuesta), 
                        None => println!("No tengo información sobre {}, mi visión y audio parecen estar desactivados", apodo)
                    }   
                }, 
                None => println!("No tengo información sobre {}, mi visión y audio parecen estar desactivados", objeto) 
            }

            return true;
        },
        interpreter::TipoRespuesta::Fecha => {
            let fecha = chrono::Local::now().format("%d/%m/%Y").to_string();
            println!("La fecha de hoy es: {}", fecha);
            return true;
        },
        interpreter::TipoRespuesta::Hora => {
            let hora = chrono::Local::now().format("%H:%M:%S").to_string();
            println!("Son las {}", hora);
            return true;
        },
        interpreter::TipoRespuesta::Abrir => {
            match objeto.as_str() {
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
        },
        _ => {return true;}
    }

}
