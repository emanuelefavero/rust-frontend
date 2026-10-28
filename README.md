# Rust nel browser

Piccolo frontend scritto in Rust con Leptos, eseguito nel browser tramite WebAssembly (CSR). Al momento mostra una pagina con un contatore interattivo.

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
- `src/main.rs`: ingresso Rust. `main()` attiva i messaggi di panic in console e monta il componente `App` nel `body`. `App` contiene la vista e lo stato del contatore.
- `style.css`: stili della pagina.
- `Cargo.toml` e `Cargo.lock`: dipendenze e versioni risolte.
- `Trunk.toml`: configurazione del watcher dei sorgenti.
- `roadmap.md`: contesto del progetto e fasi di sviluppo.

Non sono necessari backend, credenziali o file `.env`.
