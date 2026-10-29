use crate::model::{Actor, ActorData, normalize};
use futures::future::{Either, select};
use gloo_net::http::Request;
use gloo_timers::future::TimeoutFuture;

async fn fetch_source(url: &str, category: &'static str) -> Result<Vec<Actor>, String> {
    let request = async {
        let response = Request::get(url).send().await.map_err(|_| {
            format!("Impossibile raggiungere la sorgente {category}. Controlla la connessione.")
        })?;
        if !response.ok() {
            return Err(format!(
                "La sorgente {category} ha risposto con errore HTTP {}.",
                response.status()
            ));
        }
        let data = response
            .json::<Vec<ActorData>>()
            .await
            .map_err(|_| format!("La sorgente {category} ha restituito dati non validi."))?;
        normalize(data, category)
    };
    // Anche una connessione bloccata termina con un errore visibile.
    match select(Box::pin(request), Box::pin(TimeoutFuture::new(20_000))).await {
        Either::Left((result, _)) => result,
        Either::Right(_) => Err(format!(
            "Tempo di attesa scaduto per la sorgente {category}. Riprova."
        )),
    }
}

pub async fn load_catalog() -> Result<Vec<Actor>, String> {
    let (actors, actresses) = futures::join!(
        fetch_source("https://lanciweb.github.io/demo/api/actors/", "attori"),
        fetch_source("https://lanciweb.github.io/demo/api/actresses/", "attrici")
    );
    // Il catalogo è completo oppure mostra un errore: nessun risultato parziale nascosto.
    let mut catalog = actors?;
    catalog.extend(actresses?);
    Ok(catalog)
}
