use websearch::{SearchOptions, providers::DuckDuckGoProvider, web_search};


pub async fn search(query: String){

    let duckduckgo = DuckDuckGoProvider::new();

    let results = web_search(SearchOptions {
        query: query,
        max_results: Some(5),
        provider: Box::new(duckduckgo),
        ..Default::default()
    }).await;

    match results {
        Ok(results) => {
            println!("Resultados de la búsqueda:");
            for result in results.iter() {
                
                println!("\n\n{} ({})", result.title, result.url);
                result.snippet.iter().for_each(|line| println!("{}", line));
            }

        },
        Err(e) => {
            eprintln!("Ups, hubo un error durante la búsqueda: {:?}", e);

        }
    }


}