use std::collections::HashMap;
use std::fs;
use std::{io::{Write}};
use json::{self, JsonValue};


pub fn program_data_initialization() -> [HashMap<&'static str, &'static str>; 2] {
    let nombres_respuestas = HashMap::from([
            ("yang", "yang es su mejor amigo jefe, desea conocer más información de él?"),
            ("alba", "alba es el Amor de su vida señor, la razón de su esfuerzo, se encuentra bien? o simplemente necesita más información sobre ella?"),
    ]);

    let apodos = HashMap::from([
        ("jiayang", "yang"),
        ("yang", "yang"),
        ("alba", "alba"),
        ("rene", "alba"),
        ("mi amor", "alba"),
    ]);

    return [nombres_respuestas, apodos];
}

pub fn user_data_initialization() -> JsonValue {
    let dir = std::env::current_dir().unwrap();
    let dir = dir.join("user_data.json");
    
    match fs::read_to_string(&dir) {
        Ok(response) => {
            let data: JsonValue = json::parse(&response).unwrap();
            return data;
        },
        Err(_e) => {
            println!("Creando archivo de datos del usuario.");
            print!("Nombre: ");
            std::io::stdout().flush().unwrap();
            let mut nombre = String::new();
            std::io::stdin().read_line(&mut nombre).unwrap();
            let created_data = json::object!{
                "username": nombre.trim()
            };
            fs::write(dir, created_data.to_string()).unwrap();
            return created_data;
        }
    }
}


