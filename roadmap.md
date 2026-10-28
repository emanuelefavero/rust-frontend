# Roadmap — Frontend Rust con Leptos e WebAssembly

## 1. Scopo del documento

Questo documento è il contesto operativo per gli agenti AI che svilupperanno l'applicazione nella directory `/Users/emanuelefavero/Projects/rust-frontend`.

Descrive le decisioni concordate con l'utente, l'ordine di sviluppo, i criteri di completamento e i confini del lavoro. Le istruzioni dell'utente e gli eventuali `AGENTS.md` applicabili hanno precedenza su questo documento.

La roadmap non autorizza a implementare tutte le fasi in un unico intervento. Ogni sessione deve rispettare la fase o la funzionalità richiesta dall'utente.

## 2. Obiettivo del progetto

Realizzare un sito frontend funzionante scritto in Rust, compilato in WebAssembly ed eseguito nel browser, usando Leptos.

L'obiettivo principale è vedere Rust in azione nella costruzione di un'interfaccia web e comprenderne progressivamente il modello. La replica esatta dell'app React di riferimento non è un requisito.

Ordine delle priorità:

1. Applicazione funzionante e contenuti visibili.
2. Codice leggibile e comprensibile per chi non ha mai usato Rust per il frontend.
3. Sviluppo incrementale, con una sola nuova funzionalità alla volta.
4. Gestione chiara degli stati e degli errori.
5. Aspetto curato e responsive.
6. Ulteriori interazioni e ottimizzazioni, solo quando utili.

Leptos supporta interfacce dinamiche. La semplicità iniziale è una scelta didattica e progettuale, non una limitazione imposta da Rust.

## 3. Decisioni già concordate

| Aspetto                       | Decisione                                            |
| ----------------------------- | ---------------------------------------------------- |
| Framework                     | Leptos                                               |
| Esecuzione                    | Client-side rendering (CSR), nel browser             |
| Compilazione                  | Rust verso WebAssembly                               |
| Strumento di sviluppo e build | Trunk                                                |
| Stile                         | CSS normale, con riuso ragionato del riferimento     |
| Dati                          | API pubbliche già usate dall'app React               |
| Backend applicativo           | Non previsto                                         |
| App React                     | Riferimento per contenuti, comportamento e aspetto   |
| Approccio                     | Una funzionalità per volta, con verifica nel browser |

Non introdurre SSR, hydration, server functions, database o servizi backend per completare le fasi di questa roadmap. Il server locale di sviluppo serve i file dell'applicazione e non costituisce un backend applicativo.

Il browser caricherà il modulo WASM tramite il JavaScript generato dagli strumenti. HTML, CSS, DOM e API del browser restano parte del funzionamento dell'applicazione.

## 4. Applicazione di riferimento

Percorso locale:

`/Users/emanuelefavero/code/boolean/react-api`

Il progetto di riferimento usa React e Vite e presenta un catalogo di attori e attrici. Dall'ispezione iniziale risultano:

- Richieste a due API pubbliche, combinate in un elenco.
- Normalizzazione e validazione dei dati.
- Stati di caricamento, successo ed errore, con retry.
- Schede con immagine e informazioni sulla persona.
- Filtri e ordinamento.
- Visualizzazione incrementale con pulsante e `IntersectionObserver`.
- Componenti UI condivisi, CSS responsive e animazioni CSS.

File utili da consultare quando pertinenti alla fase:

- `src/features/actors/Actors.jsx`: orchestrazione e stati.
- `src/features/actors/api.js`: endpoint e caricamento dei due elenchi.
- `src/features/actors/types.js`: descrizione dei dati.
- `src/features/actors/validation.js`: controlli sui dati ricevuti.
- `src/features/actors/utils.js`: normalizzazione, filtri e ordinamento.
- `src/features/actors/components/`: catalogo, schede, filtri e relativi stili.
- `src/components/shared/IncrementalList.jsx`: caricamento incrementale, opzionale per il porting.
- `src/index.css` e `src/App.css`: stile generale.

Endpoint individuati:

- Attori: `https://lanciweb.github.io/demo/api/actors/`
- Attrici: `https://lanciweb.github.io/demo/api/actresses/`

Queste informazioni descrivono l'ispezione iniziale: prima di usarle per implementare una funzionalità, rileggere i file pertinenti e verificare la risposta corrente delle API.

Il progetto React deve rimanere invariato. Non copiarne dipendenze, architettura o ottimizzazioni senza una necessità concreta.

## 5. Regole operative per gli agenti AI

### Prima di intervenire

1. Leggere le istruzioni applicabili e lo stato corrente del progetto.
2. Individuare la fase richiesta e il suo risultato visibile.
3. Controllare file esistenti e modifiche dell'utente prima di effettuare edit.
4. Verificare la documentazione corrente per API, configurazione e comandi degli strumenti.
5. Comunicare brevemente cosa verrà realizzato.

Per la documentazione seguire le istruzioni Context7 del progetto: usare `ctx7`, risolvere prima il nome della libreria e consultare poi il relativo ID. Eseguire le richieste fuori dal sandbox predefinito, rispettare il limite di tre comandi per domanda e non includere dati sensibili nelle query. In caso di quota esaurita, dichiararlo e indicare le opzioni di autenticazione previste dalle istruzioni. Non presentare esempi ricordati o obsoleti come API correnti verificate.

Scegliere release stabili e compatibili tra loro. Non adottare prerelease solo perché compaiono come release più recenti. Annotare le versioni effettivamente scelte nei file di configurazione appropriati; non fissarle in anticipo in questa roadmap.

### Durante lo sviluppo

- Mantenere il progetto compilabile e la pagina utilizzabile al termine di ogni fase.
- Preferire soluzioni dirette e idiomatiche, con pochi componenti e dipendenze motivate.
- Adattare il codice al modello reattivo di Leptos; non tradurre automaticamente ogni hook React.
- Spiegare i concetti Rust nel momento in cui servono: ownership, borrowing, closures, `Option`, `Result`, strutture ed enum.
- Preservare le modifiche dell'utente ed evitare refactor estranei alla funzionalità.
- Non aggiungere documenti, framework CSS, librerie di stato o strumenti di test senza una necessità concreta.
- Non effettuare commit, push, pubblicazioni o creazione di servizi senza istruzioni dell'utente.
- Non avviare subagenti senza una richiesta esplicita o istruzioni applicabili che li richiedano.

### Alla conclusione di una fase

- Eseguire le verifiche pertinenti e aprire la pagina nel browser quando disponibile.
- Distinguere chiaramente build riuscita, controllo statico, verifica con dati simulati e prova reale delle API.
- Riportare risultato, modalità di avvio e limiti ancora presenti.
- Non dichiarare verificato un comportamento che non è stato osservato.
- Fermarsi al confine della funzionalità richiesta; non anticipare automaticamente le successive.

## 6. Fasi di sviluppo

### Fase 1 — Prima pagina interattiva Rust nel browser

**Obiettivo:** verificare l'intera catena Rust → WASM → browser con un esempio piccolo e comprensibile.

Attività:

1. Controllare la disponibilità della toolchain Rust e degli strumenti necessari.
2. Preparare un progetto Leptos CSR minimale usando Trunk e versioni stabili compatibili.
3. Configurare ingresso HTML, punto di avvio Rust e foglio CSS.
4. Mostrare titolo, breve descrizione e un contatore aggiornabile con un pulsante.
5. Configurare messaggi utili per eventuali panic nel browser.
6. Verificare avvio in sviluppo e build di produzione.

Criteri di completamento:

- La pagina si apre dal server locale e mostra i contenuti.
- Il contatore cambia al clic senza ricaricare la pagina.
- Il progetto compila per il browser e produce una build di produzione.
- Non ci sono errori runtime riconducibili all'app nella console durante la prova.
- L'utente riceve i comandi effettivamente verificati per avviare il progetto.

Concetti da spiegare: ruolo di Cargo e Trunk, modulo WASM, componente, macro della vista, signal ed evento.

Non aggiungere chiamate API, routing o struttura completa del catalogo in questa fase.

### Fase 2 — Una scheda attore con dati locali

**Obiettivo:** costruire la prima parte reale dell'interfaccia senza dipendere dalla rete.

Attività:

1. Consultare il modello dati del riferimento e scegliere un sottoinsieme minimo.
2. Definire una struttura Rust con nome, immagine e descrizione.
3. Creare un componente scheda alimentato da dati locali.
4. Aggiungere stile essenziale e comportamento responsive.
5. Curare testo alternativo dell'immagine e leggibilità del contenuto.

Criteri di completamento:

- Una scheda mostra correttamente i dati locali.
- La pagina rimane leggibile su viewport mobile e desktop.
- La separazione tra dati e presentazione è chiara, senza astrazioni premature.

Concetti da spiegare: strutture, stringhe, proprietà dei componenti e passaggio dei dati.

### Fase 3 — Primo elenco reale dall'API

**Obiettivo:** ottenere il primo catalogo funzionante con contenuti reali.

Attività:

1. Verificare risposta, campi opzionali e accessibilità dal browser dell'endpoint attori.
2. Scegliere il minimo necessario per richiesta HTTP e deserializzazione JSON, verificando il supporto WASM.
3. Separare il caricamento dei dati dalla presentazione.
4. Adattare il modello Rust ai dati effettivi, distinguendo campi obbligatori e facoltativi.
5. Gestire caricamento, successo con dati, elenco vuoto ed errore.
6. Renderizzare tutte le schede ricevute, senza caricamento incrementale.

Criteri di completamento:

- All'apertura vengono caricati e mostrati gli attori dell'API reale.
- Un errore di rete o una risposta non valida produce un messaggio visibile.
- Un elenco vuoto viene rappresentato esplicitamente.
- La UI non rimane indefinitamente nello stato di caricamento al termine della richiesta.
- Non sono necessari backend, credenziali o proxy per il caso d'uso verificato.

Concetti da spiegare: operazioni asincrone, deserializzazione, `Result`, `Option` e rappresentazione degli stati.

**Primo traguardo applicativo:** aprire il sito Rust e vedere un catalogo alimentato da un'API pubblica.

### Fase 4 — Catalogo completo e presentazione responsive

**Obiettivo:** mostrare attori e attrici in un catalogo coerente.

Attività:

1. Verificare anche la risposta dell'endpoint attrici.
2. Caricare i due elenchi; preferire richieste concorrenti se la soluzione rimane leggibile.
3. Normalizzare le eventuali differenze e combinare i risultati.
4. Garantire identificatori univoci anche in caso di ID uguali tra le due sorgenti.
5. Stabilire un comportamento esplicito quando una delle richieste fallisce.
6. Rifinire griglia, spaziature e immagini; aggiungere informazioni come nazionalità, anno di nascita, film e premi quando disponibili.

Comportamento iniziale consigliato: se una delle due richieste fallisce, mostrare un errore per il catalogo completo. Il supporto a risultati parziali è una possibile evoluzione, non un requisito iniziale.

Criteri di completamento:

- Entrambe le sorgenti sono rappresentate e gli identificatori non collidono.
- I campi facoltativi non causano crash o contenuti incomprensibili.
- L'errore di una sorgente segue il comportamento scelto.
- La griglia è utilizzabile su mobile e desktop.

### Fase 5 — Interazioni incrementali

**Obiettivo:** esplorare la reattività aggiungendo una sola funzionalità per intervento.

Ordine proposto, modificabile secondo le preferenze dell'utente:

1. Un filtro semplice, per esempio attori/attrici/tutti.
2. Messaggio per assenza di risultati dopo il filtro.
3. Reset dei filtri.
4. Ordinamento alfabetico.
5. Retry dopo un errore di caricamento.
6. Eventuali altri filtri realmente utili.
7. Animazioni CSS leggere, rispettando la preferenza di movimento ridotto.

Per ciascuna funzionalità:

- Definire il comportamento prima dell'implementazione.
- Usare signals e valori derivati secondo il modello Leptos.
- Evitare stato duplicato e richieste di rete non necessarie per filtri locali.
- Verificare l'interazione e i casi vuoti nel browser.
- Mantenere funzionanti le caratteristiche già completate.

Non è necessario riprodurre tutti i filtri o gli effetti dell'app React.

### Fase 6 — Verifica finale e possibili evoluzioni

**Obiettivo:** consolidare la versione raggiunta, quando l'utente considera sufficiente il set di funzionalità.

Verifiche:

- Caricamento reale delle API e comportamento in caso di errore.
- Contenuti leggibili, controlli etichettati e navigazione da tastiera.
- Layout mobile e desktop, immagini mancanti e testi lunghi.
- Build di produzione e prova locale degli artefatti prodotti.
- Assenza di errori applicativi evidenti nella console.

Evoluzioni da valutare separatamente, solo su richiesta o in presenza di un problema concreto:

- Pulsante per mostrare altre schede.
- Caricamento progressivo con `IntersectionObserver`.
- Ottimizzazione del peso WASM e delle immagini.
- Routing o pagina di dettaglio.
- Pubblicazione su hosting statico, dopo aver concordato il servizio e il piano operativo.

Queste evoluzioni non fanno parte dei requisiti minimi e non devono ritardare il primo catalogo funzionante.

## 7. Architettura: indicazioni, non vincoli prematuri

Partire con pochi file. Introdurre separazioni quando esistono responsabilità reali:

- Avvio dell'applicazione.
- Componente principale e composizione della pagina.
- Modello dei dati.
- Accesso alle API e adattamento della risposta.
- Presentazione delle schede e del catalogo.
- Stili.

Nomi e cartelle verranno scelti durante lo sviluppo. Non creare in anticipo layer generici, design system, store globali o moduli vuoti per fasi future.

Usare lo stato locale finché sufficiente. Gli aggiornamenti reattivi devono essere espliciti e comprensibili. Gestire cleanup e durata dei callback se si introducono osservatori o listener manuali.

## 8. Strategia di verifica

Adattare la verifica al cambiamento:

- **Toolchain e configurazione:** compilazione, avvio locale e build di produzione.
- **Interfaccia e signals:** prova nel browser delle interazioni introdotte.
- **API:** controllo della risposta corrente e integrazione reale dal browser; una richiesta da terminale non verifica le condizioni CORS.
- **Trasformazioni dati:** test unitari mirati quando la logica presenta casi significativi, per esempio collisioni di ID o filtri combinati.
- **Stile:** controllo visivo su mobile e desktop.

Non introdurre un'infrastruttura di test ampia per un contatore o per semplici modifiche CSS. Non sostituire la prova dell'applicazione con il solo esito della compilazione. Se il browser o la rete non sono disponibili, dichiarare precisamente cosa resta da verificare.

## 9. Limiti e rischi da gestire

- **Curva di apprendimento:** introdurre i concetti Rust attraverso funzionalità piccole e osservabili.
- **Tempi di compilazione:** mantenere dipendenze e struttura contenute; non promettere tempi equivalenti a Vite.
- **Interoperabilità web:** usare API e wrapper compatibili con WASM; non assumere che tutte le crate Rust funzionino nel browser.
- **Dipendenze React:** Axios e componenti come `lucide-react` non si trasferiscono direttamente; scegliere equivalenti minimi o SVG quando necessario.
- **Avvio e SEO:** CSR richiede il caricamento di JavaScript e WASM prima della UI; questo compromesso è accettato per l'esperimento.
- **Prestazioni:** WASM non garantisce automaticamente maggiore velocità; ottimizzare solo dopo aver osservato un problema.
- **API esterne:** disponibilità, schema e CORS possono cambiare; gestire gli errori senza nascondere il problema.
- **Sicurezza del frontend:** il modulo WASM è scaricabile dal browser e non è un luogo adatto a conservare segreti.

## 10. Stato iniziale e ripresa del lavoro

Al momento della redazione, la directory di destinazione non contiene ancora l'applicazione. La scelta dello stack e l'approccio incrementale sono concordati; nessuna fase di implementazione è dichiarata completata da questo documento.

Prima attività prevista: **Fase 1 — Prima pagina interattiva Rust nel browser**, quando richiesta dall'utente.

Alla ripresa, verificare sempre lo stato reale dei file: questa sezione descrive il punto di partenza, non lo stato corrente garantito del progetto.
