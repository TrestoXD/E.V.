use std::io;
fn main() {
    println!("Hola soy KAREN");

    println!("Que necesitas?");

    let mut response = String::new();

    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");

    println!("You Said {response}");
    whatmeans(response);
}

fn whatmeans(resp: String){
    println!("Thinking...");

    if resp == "Hola"{
        println!("Hola como estás?")
    }
}