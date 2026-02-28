pub enum TipoRespuesta {
    Buscar,
    Persona,
    Hora,
    Hecho,
    Abrir,
    Fecha
}

pub fn get_function(prompt: &str, user_data: &json::JsonValue) -> (TipoRespuesta, String) {
    let interrogativas = ["quien", "quienes", "que", "cuando", "donde", "como", "cual", "cuales", "cuanto", "cuanta", "cuantos", "cuantas", "porque"];
    let preposiciones = ["en", "a", "de", "con", "sin", "por", "para", "sobre", "entre", "hacia", "hasta", "desde"];
    let verbo_poder = ["puedes", "podrías"];
    let verbo_abrir = ["abre", "abrir", "abreme", "abrime", "abrirme"];
    let verbo_buscar = ["busca", "buscar", "buscame", "buscarme"];
    let verbo_poner = ["pon", "ponme", "poner"];
    let imperativos = ["abre", "busca", "pon", "dime"];

    let prompt_separated = &prompt.split_whitespace().collect::<Vec<&str>>();

    if prompt_separated.is_empty() {
        return (TipoRespuesta::Hecho, "".to_string());
    }

    if preposiciones.contains(&prompt_separated[0]){
        match prompt_separated[1] {
            "que" => {
                if prompt_separated[2] == "dia" && prompt_separated[3] == "estamos"{
                    return (TipoRespuesta::Fecha, "".to_string());
                } else {
                return (TipoRespuesta::Buscar, prompt.to_string());
                }
            },
            "quien" | "quienes" | "cuando" | "donde" => {
                return (TipoRespuesta::Buscar, prompt.to_string());
            },
            _ => {
                println!("Lo siento, no te he entendido");
                return (TipoRespuesta::Hecho, "".to_string());
            }
        }
    } else if interrogativas.contains(&prompt_separated[0]){
        match prompt_separated[0] {
            "quien" => {
                match prompt_separated[1] {
                    "eres" => {
                        println!("Mi nombre es KAREN, soy tu asistente aqui disponible para cualquier contenido multimedia que quieras acceder");
                        return (TipoRespuesta::Hecho, "".to_string());
                    },
                    "soy" => {
                        println!("Eres {}, es un placer conocerte, espero poder ayudarte en lo que necesites", user_data["username"]);
                        return (TipoRespuesta::Hecho, "".to_string());
                    },
                    "es" => {
                        return (TipoRespuesta::Persona, prompt_separated[2..].join(" "));
                    },
                    _ => {
                        return (TipoRespuesta::Buscar, prompt.to_string());
                    },
                }
            },
            "que" => {
                if prompt_separated[1] == "hora" {
                    return (TipoRespuesta::Hora, "".to_string());
                } else {
                    return (TipoRespuesta::Buscar, prompt.to_string());
                }
            },
            _ => {
                return (TipoRespuesta::Buscar, prompt.to_string());
            }
        }
    } else if verbo_poder.contains(&prompt_separated[0]){
        if verbo_abrir.contains(&prompt_separated[1]){
            return (TipoRespuesta::Abrir, prompt_separated[2..].join(" "));
        } else if verbo_buscar.contains(&prompt_separated[1]){
            return (TipoRespuesta::Buscar, prompt_separated[2..].join(" "));
        } else if verbo_poner.contains(&prompt_separated[1]){
            return (TipoRespuesta::Abrir, prompt_separated[2..].join(" "));
        } else {
            println!("Lo siento, no te he entendido");
            return (TipoRespuesta::Hecho, "".to_string());
        }
    } else if imperativos.contains(&prompt_separated[0]){
        if verbo_abrir.contains(&prompt_separated[0]){
            return (TipoRespuesta::Abrir, prompt_separated[1..].join(" "));
        } else if verbo_buscar.contains(&prompt_separated[0]){
            return (TipoRespuesta::Buscar, prompt_separated[1..].join(" "));
        } else if verbo_poner.contains(&prompt_separated[0]){
            return (TipoRespuesta::Abrir, prompt_separated[1..].join(" "));
        } else {
            println!("Lo siento, no te he entendido");
            return (TipoRespuesta::Hecho, "".to_string());
        }
    } else {
        println!("Lo siento, no te he entendido");
        return (TipoRespuesta::Hecho, "".to_string());
    }
}