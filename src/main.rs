use std::io::{self, Write};
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

fn initialize() {
    println!("Hola soy");
        println!("=================================
        ");
    println!( r#"    __ __ ___    ____  _______   __
   / //_//   |  / __ \/ ____/ | / /
  / ,<  / /| | / /_/ / __/ /  |/ / 
 / /| |/ ___ |/ _, _/ /___/ /|  /  
/_/ |_/_/  |_/_/ |_/_____/_/ |_/
    "#);
    println!("=================================");
    println!("Que necesitas?
    ");
}

fn whatmeans(_resp: String) -> bool{

    if _resp ==  "exit" {
        println!("Apagando...");
        return false;
    }else {
        println!("Disculpame pero no he entendido");
        return true;
    }
}