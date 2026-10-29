# Rust nel browser

Piccolo frontend scritto in Rust con Leptos, eseguito nel browser tramite WebAssembly (CSR). Mostra un catalogo di attori e attrici dalle API pubbliche Lanciweb, con ricerca per nome, filtro per categoria, ordinamento alfabetico e reset. Le operazioni sui filtri sono locali e non ripetono le richieste di rete.

## Preparazione iniziale

Installa Rust e Cargo tramite [rustup](https://rustup.rs/), poi esegui:

```bash
rustup update stable
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```

Questi passaggi servono una sola volta per preparare il computer. Il progetto è stato verificato con Rust/Cargo 1.99.1 e Trunk 0.22.1.

## Avvio in sviluppo

Dalla cartella del progetto:

```bash
trunk serve --address 127.0.0.1 --port 8080
```

Apri `http://127.0.0.1:8080`. Trunk compila l'app e aggiorna la pagina quando modifichi i sorgenti. Per fermare il server premi `Ctrl+C`.

Non serve installare manualmente le dipendenze né compilare prima dell'avvio: Cargo scarica le dipendenze indicate in `Cargo.toml`, mentre Trunk gestisce la compilazione in WASM. Il primo avvio richiede una connessione Internet e può essere più lento.

## Build di produzione

```bash
trunk build --release
```

I file pronti per un server statico vengono generati in `dist/`. Per provarli localmente, ferma prima il server di sviluppo con `Ctrl+C`, poi, se hai Python 3, esegui:

```bash
python3 -m http.server 8080 --bind 127.0.0.1 --directory dist
```

Apri `http://127.0.0.1:8080`. I file vanno serviti via HTTP, non aperti direttamente dal filesystem. Anche questo server si ferma con `Ctrl+C`.

## Struttura ed entry point

- `index.html`: ingresso HTML; indica a Trunk il programma Rust e il foglio CSS da caricare.
- `src/main.rs`: ingresso Rust. `main()` attiva i messaggi di panic in console e monta il componente `App` nel `body`. `App` gestisce caricamento, errore, catalogo vuoto e retry. `Catalog` deriva i risultati dai filtri; `ActorCard` presenta una persona.
- `src/model.rs`: dati deserializzati e normalizzazione con ID distinti per sorgente.
- `src/api.rs`: richieste concorrenti, validazione e timeout di 20 secondi. Se una sorgente fallisce, il catalogo completo mostra un errore con Riprova.
- `style.css`: griglia responsive, immagini di riserva e rispetto del movimento ridotto.
- `Cargo.toml` e `Cargo.lock`: dipendenze e versioni risolte.
- `Trunk.toml`: configurazione del watcher dei sorgenti.
- `roadmap.md`: contesto del progetto e fasi di sviluppo.

Non sono necessari backend, credenziali o file `.env`.

## Modello Rust in breve

Una `struct` descrive i dati di una persona: `String` possiede il testo, mentre `Option` rappresenta informazioni che possono mancare, come anno di morte e immagine. Le proprietà del componente passano una persona a `ActorCard`; il componente ne prende possesso per costruire la vista.

Le richieste sono funzioni `async`: `await` attende la risposta senza bloccare il browser. Serde converte il JSON nelle strutture Rust; `Result` distingue dati validi ed errore, propagato con `?`. `LocalResource` avvia il caricamento client e permette il retry. I signals conservano i controlli e un `Memo` calcola la lista filtrata senza duplicare lo stato del catalogo. Le closures `move` catturano i valori usati dagli eventi e dalle viste reattive.

## Verifiche effettuate

Build `trunk build --release` e prova degli artefatti con il server Python sopra indicato. Nel browser: 40 attori e 22 attrici dalle API reali, filtri, ricerca senza risultati, ordine A–Z/Z–A, reset, retry e navigazione da tastiera. Layout verificato a 375 e 1280 px senza overflow.

Con risposte simulate nel browser: una scheda locale, campi facoltativi assenti, immagine rotta, testo lungo, catalogo vuoto, JSON non valido ed errore HTTP di una sola sorgente, oltre al fallimento di rete. Dopo il retry sono tornati i 62 risultati reali. Nessun errore applicativo osservato nella prova reale; i fallimenti di rete intenzionali compaiono nella console del browser. Chromium emette inoltre un avviso relativo al preload con integrity generato da Trunk.
